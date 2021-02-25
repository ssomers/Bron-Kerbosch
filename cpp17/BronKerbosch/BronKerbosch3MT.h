//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot arbitrarily

#pragma once

#include "BronKerboschPivot.h"
#include "CliqueList.h"
#include "GraphDegeneracy.h"
#pragma warning(push)
#pragma warning(disable : 4265)
#include "cppcoro/async_generator.hpp"
#include "cppcoro/sequence_barrier.hpp"
#include "cppcoro/single_producer_sequencer.hpp"
#include "cppcoro/multi_producer_sequencer.hpp"
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
        static const int NUM_VISITING_FIBERS = 5;
        static const size_t STARTS = 64;
        static const size_t VISIT_JOBS = 64;
        static const size_t CLIQUES = 64;
        static const Vertex SENTINEL_VTX = std::numeric_limits<Vertex>::max();

        struct VisitJob {
            Vertex start = SENTINEL_VTX;
            VertexSet candidates;
            VertexSet excluded;

            VisitJob() = default;
            VisitJob(Vertex start, VertexSet&& candidates, VertexSet&& excluded)
                : start(start)
                , candidates(std::move(candidates))
                , excluded(std::move(excluded))
            {
            }
        };

        template <typename VertexSet>
        static cppcoro::task<> start_producer(
            UndirectedGraph<VertexSet> const& graph,
            cppcoro::single_producer_sequencer<size_t>& start_sequencer,
            Vertex (*starts)[STARTS], // pass-by-reference avoiding ICE
            cppcoro::static_thread_pool& tp
        )
        {
            auto ordering = DegeneracyOrderIter<VertexSet>::degeneracy_ordering(graph, -1);
            while (auto next = ordering.next()) {
                size_t seq = co_await start_sequencer.claim_one(tp);
                (*starts)[seq % STARTS] = *next;
                start_sequencer.publish(seq);
            }

            auto seq = co_await start_sequencer.claim_one(tp);
            (*starts)[seq % VISIT_JOBS] = SENTINEL_VTX;
            start_sequencer.publish(seq);
        }

        template <typename VertexSet>
        static cppcoro::task<> visit_producer(
            UndirectedGraph<VertexSet> const& graph,
            cppcoro::sequence_barrier<size_t>& start_barrier,
            cppcoro::single_producer_sequencer<size_t> const& start_sequencer,
            cppcoro::single_producer_sequencer<size_t>& visit_sequencer,
            Vertex const (*starts)[STARTS], // pass-by-reference avoiding ICE
            VisitJob (*visit_jobs)[VISIT_JOBS], // pass-by-reference avoiding ICE
            cppcoro::static_thread_pool& tp
        )
        {
            auto excluded = Util::with_capacity<VertexSet>(std::max(1u, graph.order()) - 1);
            bool reachedEnd = false;
            std::size_t nextToRead = 0;
            while (!reachedEnd)
            {
                const std::size_t available = co_await start_sequencer.wait_until_published(nextToRead, tp);
                do
                {
                    Vertex v = (*starts)[nextToRead % VISIT_JOBS];
                    reachedEnd = v == SENTINEL_VTX;
                    if (reachedEnd) {
                        assert(nextToRead == available);
                        break;
                    }
                    auto const& neighbours = graph.neighbours(v);
                    assert(!neighbours.empty());
                    auto neighbouring_candidates = Util::difference(neighbours, excluded);
                    if (neighbouring_candidates.empty()) {
                        assert(!Util::are_disjoint(neighbours, excluded));
                    }
                    else {
                        auto neighbouring_excluded = Util::intersection(neighbours, excluded);

                        size_t seq = co_await visit_sequencer.claim_one(tp);
                        (*visit_jobs)[seq % VISIT_JOBS] = VisitJob{ v, std::move(neighbouring_candidates), std::move(neighbouring_excluded) };
                        visit_sequencer.publish(seq);
                    }
                    excluded.insert(v);
                    ++nextToRead;
                } while (nextToRead < available);

                start_barrier.publish(available);
            }

            auto seq = co_await visit_sequencer.claim_one(tp);
            (*visit_jobs)[seq % VISIT_JOBS].start = SENTINEL_VTX;
            visit_sequencer.publish(seq);
        }

        template <typename VertexSet>
        static cppcoro::task<> clique_producer(
            UndirectedGraph<VertexSet> const& graph,
            cppcoro::sequence_barrier<size_t>& visit_barrier,
            cppcoro::single_producer_sequencer<size_t> const& visit_sequencer,
            cppcoro::multi_producer_sequencer<size_t>& clique_sequencer,
            VisitJob (*visit_jobs)[VISIT_JOBS], // pass-by-reference avoiding ICE
            CliqueList (*cliques)[CLIQUES], // pass-by-reference avoiding ICE
            cppcoro::static_thread_pool& tp
        )
        {
            bool reachedEnd = false;
            std::size_t nextToRead = 0;
            while (!reachedEnd)
            {
                const std::size_t available = co_await visit_sequencer.wait_until_published(nextToRead, tp);
                do
                {
                    auto& job = (*visit_jobs)[nextToRead % VISIT_JOBS];
                    reachedEnd = job.start == SENTINEL_VTX;
                    if (reachedEnd) {
                        assert(nextToRead == available);
                        break;
                    }
                    auto pile = VertexPile{ job.start };
                    size_t seq = co_await clique_sequencer.claim_one(tp);
                    (*cliques)[seq % CLIQUES] = BronKerboschPivot::visit<VertexSet>(
                        graph,
                        PivotChoice::MaxDegreeLocal,
                        PivotChoice::MaxDegreeLocal,
                        std::move(job.candidates),
                        std::move(job.excluded),
                        &pile);
                    clique_sequencer.publish(seq);
                    ++nextToRead;
                } while (nextToRead < available);

                visit_barrier.publish(available);
            }

            auto seq = co_await clique_sequencer.claim_one(tp);
            auto sentinel_clique = CliqueList{};
            sentinel_clique.push_back(VertexList{ SENTINEL_VTX });
            (*cliques)[seq % CLIQUES] = sentinel_clique;
            clique_sequencer.publish(seq);
        }

        static cppcoro::task<> clique_consumer(
            cppcoro::sequence_barrier<size_t>& clique_barrier,
            cppcoro::multi_producer_sequencer<size_t> const& clique_sequencer,
            CliqueList (*cliques)[CLIQUES], // pass-by-reference avoiding ICE
            size_t producers,
            CliqueList& all_cliques,
            cppcoro::static_thread_pool& tp
        )
        {
            std::size_t nextToRead = 0;
            while (producers)
            {
                const std::size_t available = co_await clique_sequencer.wait_until_published(nextToRead, nextToRead-1, tp);
                do
                {
                    auto some_cliques = (*cliques)[nextToRead % VISIT_JOBS];
                    if (!some_cliques.empty()) {
                        bool reachedEnd = (*some_cliques.begin())[0] == SENTINEL_VTX;
                        if (reachedEnd) {
                            --producers;
                        } else {
                            all_cliques.splice(all_cliques.end(), std::move(some_cliques));
                        }
                    }
                    ++nextToRead;
                } while (nextToRead < available);

                clique_barrier.publish(available);
            }
        }

    public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph)
        {
            auto tp = cppcoro::static_thread_pool{ 6 };
            auto start_barrier = cppcoro::sequence_barrier<size_t>{};
            auto start_sequencer = cppcoro::single_producer_sequencer<size_t>{ start_barrier, STARTS };
            Vertex starts[STARTS];

            auto visit_barrier = cppcoro::sequence_barrier<size_t>{};
            auto visit_sequencer = cppcoro::single_producer_sequencer<size_t>{ visit_barrier, VISIT_JOBS };
            VisitJob visit_jobs[VISIT_JOBS];

            auto clique_barrier = cppcoro::sequence_barrier<size_t>{};
            auto clique_sequencer = cppcoro::multi_producer_sequencer<size_t>{ clique_barrier, CLIQUES };
            CliqueList cliques[CLIQUES];

            const ptrdiff_t VISITORS = 1;
            auto tasks = std::vector<cppcoro::task<void>>{};
            auto all_cliques = CliqueList{};
            tasks.emplace_back(BronKerbosch3MT::start_producer(graph, start_sequencer, &starts, tp));
            tasks.emplace_back(BronKerbosch3MT::visit_producer(graph, start_barrier, start_sequencer, visit_sequencer, &starts, &visit_jobs, tp));
            for (auto _ = 0; _ < VISITORS; ++_) {
                tasks.emplace_back(BronKerbosch3MT::clique_producer(graph, visit_barrier, visit_sequencer, clique_sequencer, &visit_jobs, &cliques, tp));
            }
            tasks.emplace_back(BronKerbosch3MT::clique_consumer(clique_barrier, clique_sequencer, &cliques, VISITORS, all_cliques, tp));
            cppcoro::sync_wait(cppcoro::when_all(std::move(tasks)));
            return all_cliques;
        }
    };
}