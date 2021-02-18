//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot arbitrarily

#pragma once

#include "BronKerboschPivot.h"
#include "GraphDegeneracy.h"
#include "Reporter.h"
#pragma warning(push)
#pragma warning(disable: 4265)
#include "cppcoro/async_generator.hpp"
#include "cppcoro/sequence_barrier.hpp"
#include "cppcoro/single_producer_sequencer.hpp"
#include "cppcoro/static_thread_pool.hpp"
#include "cppcoro/sync_wait.hpp"
#include "cppcoro/task.hpp"
#include "cppcoro/when_all.hpp"
#pragma warning(pop)
#pragma warning(disable: 4623)


namespace BronKerbosch {
    template <typename VertexSet>
    class BronKerbosch3MT {
    private:
        static const int NUM_VISITING_FIBERS = 5;
        static const size_t STARTS = 64;
        static const size_t VISIT_JOBS = 64;
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
            Vertex starts[STARTS],
            cppcoro::static_thread_pool& tp
        )
        {
            auto ordering = DegeneracyOrderIter<VertexSet>::degeneracy_ordering(graph, -1);
            while (auto next = ordering.next()) {
                size_t seq = co_await start_sequencer.claim_one(tp);
                starts[seq % STARTS] = *next;
                start_sequencer.publish(seq);
            }

            auto seq = co_await start_sequencer.claim_one(tp);
            starts[seq % VISIT_JOBS] = SENTINEL_VTX;
            start_sequencer.publish(seq);
        }

        template <typename VertexSet>
        static cppcoro::task<> visit_producer(
            UndirectedGraph<VertexSet> const& graph,
            cppcoro::sequence_barrier<size_t>& start_barrier,
            cppcoro::single_producer_sequencer<size_t> const& start_sequencer,
            cppcoro::single_producer_sequencer<size_t>& visit_sequencer,
            Vertex const starts[STARTS],
            VisitJob visit_jobs[VISIT_JOBS],
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
                    Vertex v = starts[nextToRead % VISIT_JOBS];
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
                        visit_jobs[seq % VISIT_JOBS] = VisitJob{ v, std::move(neighbouring_candidates), std::move(neighbouring_excluded) };
                        visit_sequencer.publish(seq);
                    }
                    excluded.insert(v);
                    ++nextToRead;
                } while (nextToRead < available);

                start_barrier.publish(available);
            }

            auto seq = co_await visit_sequencer.claim_one(tp);
            visit_jobs[seq % VISIT_JOBS].start = SENTINEL_VTX;
            visit_sequencer.publish(seq);
        }

        template <typename VertexSet, typename Reporter>
        static cppcoro::task<> visit_consumer(
            UndirectedGraph<VertexSet> const& graph,
            Reporter& reporter,
            cppcoro::sequence_barrier<size_t>& visit_barrier,
            cppcoro::single_producer_sequencer<size_t> const& visit_sequencer,
            VisitJob visit_jobs[VISIT_JOBS],
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
                    auto& job = visit_jobs[nextToRead % VISIT_JOBS];
                    reachedEnd = job.start == SENTINEL_VTX;
                    if (reachedEnd) {
                        assert(nextToRead == available);
                        break;
                    }
                    auto pile = VertexPile{ job.start };
                    BronKerboschPivot::visit<VertexSet>(
                        graph,
                        reporter,
                        PivotChoice::MaxDegreeLocal,
                        PivotChoice::MaxDegreeLocal,
                        std::move(job.candidates),
                        std::move(job.excluded),
                        &pile);
                    ++nextToRead;
                } while (nextToRead < available);

                visit_barrier.publish(available);
            }
        }

    public:
        template <typename VertexSet, typename Reporter>
        static void explore(UndirectedGraph<VertexSet> const& graph, Reporter& reporter)
        {
            auto tp = cppcoro::static_thread_pool{ 6 };
            auto start_barrier = cppcoro::sequence_barrier<size_t>{};
            auto start_sequencer = cppcoro::single_producer_sequencer<size_t>{ start_barrier, STARTS };
            Vertex starts[STARTS];

            auto visit_barrier = cppcoro::sequence_barrier<size_t>{};
            auto visit_sequencer = cppcoro::single_producer_sequencer<size_t>{ visit_barrier, VISIT_JOBS };
            VisitJob visit_jobs[VISIT_JOBS];

            auto tasks = std::vector<cppcoro::task<void>>{};
            tasks.emplace_back(BronKerbosch3MT::start_producer(graph, start_sequencer, starts, tp));
            tasks.emplace_back(BronKerbosch3MT::visit_producer(graph, start_barrier, start_sequencer, visit_sequencer, starts, visit_jobs, tp));
            tasks.emplace_back(BronKerbosch3MT::visit_consumer(graph, reporter, visit_barrier, visit_sequencer, visit_jobs, tp));
            cppcoro::sync_wait(cppcoro::when_all(std::move(tasks)));
        }
    };
}