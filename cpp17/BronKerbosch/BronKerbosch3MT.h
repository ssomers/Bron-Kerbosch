//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot arbitrarily

#pragma once

#include "BronKerboschPivot.h"
#include "CliqueList.h"
#include "GraphDegeneracy.h"
#pragma warning(push)
#pragma warning(disable : 4265 5204 26495)
#include "cppcoro/async_generator.hpp"
#include "cppcoro/multi_producer_sequencer.hpp"
#include "cppcoro/sequence_barrier.hpp"
#include "cppcoro/single_producer_sequencer.hpp"
#include "cppcoro/static_thread_pool.hpp"
#include "cppcoro/sync_wait.hpp"
#include "cppcoro/task.hpp"
#include "cppcoro/when_all.hpp"
#pragma warning(pop)
#pragma warning(disable : 4623)

namespace BronKerbosch {
    template <typename VertexSet>
    class BronKerbosch3MT {
       private:
        static const size_t VISITORS = 8;
        static const size_t STARTS = 8;
        static const size_t CLIQUES = 8;
        static const Vertex SENTINEL_VTX = std::numeric_limits<Vertex>::max();

        struct VisitJob {
            Vertex start = SENTINEL_VTX;
            VertexSet candidates;
            VertexSet excluded;
#ifndef NDEBUG
            std::atomic_bool waiting;
#endif

            VisitJob() noexcept = default;
            VisitJob(VisitJob&&) = delete;
            VisitJob(VisitJob const&) = delete;
            VisitJob& operator=(VisitJob&&) = delete;
            VisitJob& operator=(VisitJob const&) = delete;

            void schedule(Vertex new_start,
                          VertexSet&& new_candidates,
                          VertexSet&& new_excluded) noexcept {
                start = new_start;
                candidates = std::move(new_candidates);
                excluded = std::move(new_excluded);
#ifndef NDEBUG
                waiting.store(true);
#endif
            }
        };

        template <typename VertexSet>
        static cppcoro::task<> start_producer(
            UndirectedGraph<VertexSet> const& graph,
            cppcoro::single_producer_sequencer<size_t>& start_sequencer,
            Vertex (*starts)[STARTS],  // pass-by-reference avoiding ICE
            cppcoro::static_thread_pool& tp) {
            auto ordering = DegeneracyOrderIter<VertexSet>::degeneracy_ordering(graph, -1);
            while (auto next = ordering.next()) {
                size_t seq = co_await start_sequencer.claim_one(tp);
                (*starts)[seq % STARTS] = *next;
                start_sequencer.publish(seq);
            }

            size_t seq = co_await start_sequencer.claim_one(tp);
            (*starts)[seq % STARTS] = SENTINEL_VTX;
            start_sequencer.publish(seq);
        }

        template <typename VertexSet>
        static cppcoro::task<> visit_producer(
            UndirectedGraph<VertexSet> const& graph,
            cppcoro::sequence_barrier<size_t>& start_barrier,
            cppcoro::single_producer_sequencer<size_t> const& start_sequencer,
            std::unique_ptr<cppcoro::single_producer_sequencer<size_t>> (
                *visit_sequencers)[VISITORS],  // pass-by-reference avoiding ICE
            Vertex const (*starts)[STARTS],    // pass-by-reference avoiding ICE
            VisitJob (*visit_jobs)[VISITORS],  // pass-by-reference avoiding ICE
            cppcoro::static_thread_pool& tp) {
            auto excluded = Util::with_capacity<VertexSet>(std::max(1u, graph.order()) - 1);
            size_t visitor = 0;
            size_t producers = 1;
            std::size_t nextToRead = 0;
            while (producers > 0) {
                const std::size_t available =
                    co_await start_sequencer.wait_until_published(nextToRead, tp);
                do {
                    assert(producers > 0);
                    Vertex start = (*starts)[nextToRead % STARTS];
                    if (start == SENTINEL_VTX) {
                        --producers;
                    } else {
                        auto const& neighbours = graph.neighbours(start);
                        assert(!neighbours.empty());
                        auto neighbouring_candidates = Util::difference(neighbours, excluded);
                        if (neighbouring_candidates.empty()) {
                            assert(!Util::are_disjoint(neighbours, excluded));
                        } else {
                            auto neighbouring_excluded = Util::intersection(neighbours, excluded);

                            auto visit_sequencer = (*visit_sequencers)[visitor].get();
                            size_t seq = co_await visit_sequencer->claim_one(tp);
                            (*visit_jobs)[visitor].schedule(start,
                                                            std::move(neighbouring_candidates),
                                                            std::move(neighbouring_excluded));
                            visit_sequencer->publish(seq);
                            visitor = ++visitor < VISITORS ? visitor : 0;
                        }
                        excluded.insert(start);
                    }
                    ++nextToRead;
                } while (nextToRead <= available);

                start_barrier.publish(available);
            }

            for (visitor = 0; visitor < VISITORS; ++visitor) {
                auto visit_sequencer = (*visit_sequencers)[visitor].get();
                size_t seq = co_await visit_sequencer->claim_one(tp);
                (*visit_jobs)[visitor].start = SENTINEL_VTX;
                visit_sequencer->publish(seq);
            }
        }

        template <typename VertexSet>
        static cppcoro::task<> clique_producer(
            UndirectedGraph<VertexSet> const& graph,
            cppcoro::sequence_barrier<size_t>& visit_barrier,
            cppcoro::single_producer_sequencer<size_t> const& visit_sequencer,
            cppcoro::multi_producer_sequencer<size_t>& clique_sequencer,
            VisitJob& visit_job,
            CliqueList (*cliques)[CLIQUES],  // pass-by-reference avoiding ICE
            cppcoro::static_thread_pool& tp) {
            size_t producers = 1;
            std::size_t nextToRead = 0;
            while (producers > 0) {
                const std::size_t available =
                    co_await visit_sequencer.wait_until_published(nextToRead, tp);
                do {
                    assert(producers > 0);
                    if (visit_job.start == SENTINEL_VTX) {
                        --producers;
                    } else {
                        assert(visit_job.waiting.exchange(false));
                        auto pile = VertexPile{visit_job.start};
                        size_t seq = co_await clique_sequencer.claim_one(tp);
                        (*cliques)[seq % CLIQUES] = BronKerboschPivot::visit<VertexSet>(
                            graph, PivotChoice::MaxDegreeLocal, PivotChoice::MaxDegreeLocal,
                            std::move(visit_job.candidates), std::move(visit_job.excluded), &pile);
                        clique_sequencer.publish(seq);
                    }
                    ++nextToRead;
                } while (nextToRead <= available);

                visit_barrier.publish(available);
            }

            size_t seq = co_await clique_sequencer.claim_one(tp);
            auto sentinel_clique = CliqueList{};
            sentinel_clique.push_back(VertexList{SENTINEL_VTX});
            (*cliques)[seq % CLIQUES] = sentinel_clique;
            clique_sequencer.publish(seq);
        }

        static cppcoro::task<> clique_consumer(
            cppcoro::sequence_barrier<size_t>& clique_barrier,
            cppcoro::multi_producer_sequencer<size_t> const& clique_sequencer,
            CliqueList (*cliques)[CLIQUES],  // pass-by-reference avoiding ICE
            size_t producers,
            CliqueList& all_cliques,
            cppcoro::static_thread_pool& tp) noexcept {
            std::size_t nextToRead = 0;
            while (producers) {
                const std::size_t available =
                    co_await clique_sequencer.wait_until_published(nextToRead, nextToRead - 1, tp);
                do {
                    auto some_cliques = (*cliques)[nextToRead % CLIQUES];
                    if (!some_cliques.empty()) {
                        if ((*some_cliques.begin())[0] == SENTINEL_VTX) {
                            --producers;
                        } else {
                            all_cliques.splice(all_cliques.end(), std::move(some_cliques));
                        }
                    }
                    ++nextToRead;
                } while (nextToRead <= available);

                clique_barrier.publish(available);
            }
        }

       public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph) {
            auto tp = cppcoro::static_thread_pool{6};
            auto start_barrier = cppcoro::sequence_barrier<size_t>{};
            auto start_sequencer =
                cppcoro::single_producer_sequencer<size_t>{start_barrier, STARTS};
            Vertex starts[STARTS];

            cppcoro::sequence_barrier<size_t> visit_barriers[VISITORS];
            std::unique_ptr<cppcoro::single_producer_sequencer<size_t>> visit_sequencers[VISITORS];
            for (auto i = 0; i < VISITORS; ++i) {
                visit_sequencers[i] = std::make_unique<cppcoro::single_producer_sequencer<size_t>>(
                    visit_barriers[i], 1);
            }
            VisitJob visit_jobs[VISITORS];

            auto clique_barrier = cppcoro::sequence_barrier<size_t>{};
            auto clique_sequencer =
                cppcoro::multi_producer_sequencer<size_t>{clique_barrier, CLIQUES};
            CliqueList cliques[CLIQUES];

            auto tasks = std::vector<cppcoro::task<void>>{};
            tasks.reserve(2 + VISITORS + 1);
            auto all_cliques = CliqueList{};
            tasks.push_back(BronKerbosch3MT::start_producer(graph, start_sequencer, &starts, tp));
            tasks.push_back(BronKerbosch3MT::visit_producer(
                graph, start_barrier, start_sequencer, &visit_sequencers, &starts, &visit_jobs, tp));
            for (auto i = 0; i < VISITORS; ++i) {
                tasks.push_back(BronKerbosch3MT::clique_producer(
                    graph, visit_barriers[i], *visit_sequencers[i], clique_sequencer, visit_jobs[i],
                    &cliques, tp));
            }
            tasks.push_back(BronKerbosch3MT::clique_consumer(clique_barrier, clique_sequencer,
                                                             &cliques, VISITORS, all_cliques, tp));
            cppcoro::sync_wait(cppcoro::when_all_ready(std::move(tasks)));
            return all_cliques;
        }
    };
}