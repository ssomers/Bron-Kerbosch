//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot arbitrarily

#pragma once

#include "BronKerboschPivot.h"
#include "GraphDegeneracy.h"
#pragma warning(push)
#pragma warning(disable : 4189 4265 4623 5204 26495)
#include "cppcoro/multi_producer_sequencer.hpp"
#include "cppcoro/sequence_barrier.hpp"
#include "cppcoro/single_producer_sequencer.hpp"
#include "cppcoro/static_thread_pool.hpp"
#include "cppcoro/sync_wait.hpp"
#include "cppcoro/task.hpp"
#include "cppcoro/when_all_ready.hpp"
#pragma warning(pop)

namespace BronKerbosch {
    template <typename Reporter, typename VertexSet>
    class BronKerbosch3MT {
      private:
        static const size_t VISITORS = 8;
        static const size_t STARTS = 8;
        static const size_t CLIQUES = 8;
        static inline const Vertex SENTINEL_VTX = Vertex::sentinel();

        class VisitJob {
            Vertex start = SENTINEL_VTX;
            VertexSet candidates;
            VertexSet excluded;
#ifndef NDEBUG
            std::atomic_bool busy; // being read from or written to by a thread
            std::atomic_bool full; // having been written to
#endif

          public:
            VisitJob() noexcept = default;
            VisitJob(VisitJob&&) = delete;
            VisitJob(VisitJob const&) = delete;
            VisitJob& operator=(VisitJob&&) = delete;
            VisitJob& operator=(VisitJob const&) = delete;

            // Store in this slot a visit to be done.
            void schedule(Vertex new_start,
                          VertexSet&& new_candidates,
                          VertexSet&& new_excluded) noexcept {
                assert(!busy.exchange(true));
                assert(!full.load());
                start = new_start;
                candidates = std::move(new_candidates);
                excluded = std::move(new_excluded);
                assert(!full.exchange(true));
                assert(busy.exchange(false));
            }

            // Store in this slot a marker that no more visits are needed.
            void close() noexcept {
                assert(!busy.exchange(true));
                assert(!full.load());
                start = SENTINEL_VTX;
                assert(!full.exchange(true));
                assert(busy.exchange(false));
            }

            // Execute the visit stored in this slot, if any.
            std::optional<typename Reporter::Result> visit(UndirectedGraph<VertexSet> const& graph,
                                                           PivotChoice pivot_choice) noexcept {
                std::optional<Reporter::Result> result;
                assert(!busy.exchange(true));
                assert(full.load());
                if (start != SENTINEL_VTX) {
                    auto pile = VertexPile{start};
                    result = BronKerboschPivot::visit<Reporter, VertexSet>(
                        graph, pivot_choice, pivot_choice, std::move(candidates),
                        std::move(excluded), &pile);
                }
                assert(full.exchange(false));
                assert(busy.exchange(false));
                return result;
            }
        };

        static cppcoro::task<>
        start_producer(UndirectedGraph<VertexSet> const& graph,
                       cppcoro::single_producer_sequencer<size_t>& start_sequencer,
                       Vertex (*starts)[STARTS], // pass-by-reference avoiding ICE
                       cppcoro::static_thread_pool& tp) {
            auto degeneracy = DegeneracyIter{graph};
            while (auto next = degeneracy.next()) {
                size_t seq = co_await start_sequencer.claim_one(tp);
                (*starts)[seq % STARTS] = *next;
                start_sequencer.publish(seq);
            }

            size_t seq = co_await start_sequencer.claim_one(tp);
            (*starts)[seq % STARTS] = SENTINEL_VTX;
            start_sequencer.publish(seq);
        }

        static cppcoro::task<>
        visit_producer(UndirectedGraph<VertexSet> const& graph,
                       cppcoro::sequence_barrier<size_t>& start_barrier,
                       cppcoro::single_producer_sequencer<size_t> const& start_sequencer,
                       std::unique_ptr<cppcoro::single_producer_sequencer<size_t>> (
                           *visit_sequencers)[VISITORS], // pass-by-reference avoiding ICE
                       Vertex const (*starts)[STARTS],   // pass-by-reference avoiding ICE
                       VisitJob (*visit_jobs)[VISITORS], // pass-by-reference avoiding ICE
                       cppcoro::static_thread_pool& tp) {
            auto excluded = Util::with_capacity<VertexSet>(std::max(1u, graph.order()) - 1);
            size_t visitor = 0;
            bool producer = true;
            size_t nextToRead = 0;
            while (producer) {
                size_t const available =
                    co_await start_sequencer.wait_until_published(nextToRead, tp);
                assert(nextToRead <= available);
                for (; nextToRead <= available; ++nextToRead) {
                    Vertex start = (*starts)[nextToRead % STARTS];
                    if (start == SENTINEL_VTX) {
                        assert(producer);
                        producer = false;
                        assert(nextToRead == available);
                    } else {
                        auto const& neighbours = graph.neighbours(start);
                        assert(!neighbours.empty());
                        auto neighbouring_excluded = Util::intersection(neighbours, excluded);
                        auto neighbouring_candidates =
                            Util::difference(neighbours, neighbouring_excluded);

                        auto visit_sequencer = (*visit_sequencers)[visitor].get();
                        size_t seq = co_await visit_sequencer->claim_one(tp);
                        (*visit_jobs)[visitor].schedule(start, std::move(neighbouring_candidates),
                                                        std::move(neighbouring_excluded));
                        visit_sequencer->publish(seq);
                        visitor = ++visitor < VISITORS ? visitor : 0;
                        excluded.insert(start);
                    }
                }

                start_barrier.publish(available);
            }

            for (visitor = 0; visitor < VISITORS; ++visitor) {
                auto visit_sequencer = (*visit_sequencers)[visitor].get();
                size_t seq = co_await visit_sequencer->claim_one(tp);
                (*visit_jobs)[visitor].close();
                visit_sequencer->publish(seq);
            }
        }

        static cppcoro::task<>
        clique_producer(UndirectedGraph<VertexSet> const& graph,
                        cppcoro::sequence_barrier<size_t>& visit_barrier,
                        cppcoro::single_producer_sequencer<size_t> const& visit_sequencer,
                        cppcoro::multi_producer_sequencer<size_t>& clique_sequencer,
                        VisitJob& visit_job,
                        std::optional<typename Reporter::Result> (
                            *clique_produce)[CLIQUES], // pass-by-reference avoiding ICE
                        cppcoro::static_thread_pool& tp) {
            bool producer = true;
            size_t nextToRead = 0;
            while (producer) {
                size_t const available =
                    co_await visit_sequencer.wait_until_published(nextToRead, tp);
                assert(nextToRead <= available);
                for (; nextToRead <= available; ++nextToRead) {
                    auto cliques = visit_job.visit(graph, PivotChoice::MaxDegreeLocal);
                    if (cliques) {
                        size_t seq = co_await clique_sequencer.claim_one(tp);
                        (*clique_produce)[seq % CLIQUES].emplace(*std::move(cliques));
                        clique_sequencer.publish(seq);
                    } else {
                        assert(producer);
                        producer = false;
                        assert(nextToRead == available);
                    }
                }

                visit_barrier.publish(available);
            }

            size_t seq = co_await clique_sequencer.claim_one(tp);
            (*clique_produce)[seq % CLIQUES].reset();
            clique_sequencer.publish(seq);
        }

        static cppcoro::task<>
        clique_consumer(cppcoro::sequence_barrier<size_t>& clique_barrier,
                        cppcoro::multi_producer_sequencer<size_t> const& clique_sequencer,
                        std::optional<typename Reporter::Result> (
                            *clique_produce)[CLIQUES], // pass-by-reference avoiding ICE
                        size_t producers,
                        Reporter::Result& all_cliques,
                        cppcoro::static_thread_pool& tp) noexcept {
            size_t nextToRead = 0;
            while (producers) {
                size_t const available =
                    co_await clique_sequencer.wait_until_published(nextToRead, nextToRead - 1, tp);
                assert(nextToRead <= available);
                for (; nextToRead <= available; ++nextToRead) {
                    auto next_publication = (*clique_produce)[nextToRead % CLIQUES];
                    if (next_publication.has_value()) {
                        Reporter::add_all(all_cliques, std::move(next_publication).value());
                    } else {
                        --producers;
                    }
                }

                clique_barrier.publish(available);
            }
        }

      public:
        static Reporter::Result explore(UndirectedGraph<VertexSet> const& graph) {
            auto tp = cppcoro::static_thread_pool{6};
            auto start_barrier = cppcoro::sequence_barrier<size_t>{};
            auto start_sequencer =
                cppcoro::single_producer_sequencer<size_t>{start_barrier, STARTS};
            Vertex starts[STARTS];

            cppcoro::sequence_barrier<size_t> visit_barriers[VISITORS];
            std::unique_ptr<cppcoro::single_producer_sequencer<size_t>> visit_sequencers[VISITORS];
            for (size_t i = 0; i < VISITORS; ++i) {
                visit_sequencers[i] = std::make_unique<cppcoro::single_producer_sequencer<size_t>>(
                    visit_barriers[i], 1);
            }
            VisitJob visit_jobs[VISITORS] = {};

            auto clique_barrier = cppcoro::sequence_barrier<size_t>{};
            auto clique_sequencer =
                cppcoro::multi_producer_sequencer<size_t>{clique_barrier, CLIQUES};
            std::optional<Reporter::Result> cliques[CLIQUES];

            auto tasks = std::vector<cppcoro::task<void>>{};
            tasks.reserve(2 + VISITORS + 1);
            tasks.push_back(BronKerbosch3MT::start_producer(graph, start_sequencer, &starts, tp));
            tasks.push_back(BronKerbosch3MT::visit_producer(graph, start_barrier, start_sequencer,
                                                            &visit_sequencers, &starts, &visit_jobs,
                                                            tp));
            for (size_t i = 0; i < VISITORS; ++i) {
                tasks.push_back(BronKerbosch3MT::clique_producer(
                    graph, visit_barriers[i], *visit_sequencers[i], clique_sequencer, visit_jobs[i],
                    &cliques, tp));
            }

            auto all_cliques = Reporter::empty();
            tasks.push_back(BronKerbosch3MT::clique_consumer(clique_barrier, clique_sequencer,
                                                             &cliques, VISITORS, all_cliques, tp));
            cppcoro::sync_wait(cppcoro::when_all_ready(std::move(tasks)));
            return all_cliques;
        }
    };
}