//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot arbitrarily

#pragma once

#include "BronKerboschPivot.h"
#include "GraphDegeneracy.h"
#include "Reporter.h"
#pragma warning(push)
#pragma warning(disable: 5220)
#include <boost/fiber/buffered_channel.hpp>
#pragma warning(pop)
#include <thread>

namespace BronKerbosch {
    template <typename VertexSet>
    class BronKerbosch3MT {
    private:
        static const int NUM_VISITING_FIBERS = 5;

        struct VisitJob {
            Vertex start = std::numeric_limits<Vertex>::max();
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

        using start_channel_t = boost::fibers::buffered_channel<Vertex>;
        using visit_channel_t = boost::fibers::buffered_channel<VisitJob>;
        using report_channel_t = boost::fibers::buffered_channel<std::vector<Vertex>>;

        class SendingReporter {
            report_channel_t& chan;

        public:
            explicit SendingReporter(report_channel_t& chan)
                : chan(chan)
            {
            }

            void record(VertexList&& clique)
            {
                assert(clique.size() > 1);
                chan.push(clique);
            }
        };

    public:
        template <typename VertexSet, typename Reporter>
        static void explore(UndirectedGraph<VertexSet> const& graph, Reporter& reporter)
        {
            start_channel_t start_channel{ 64 };
            visit_channel_t visit_channel{ 64 };
            report_channel_t report_channel{ 64 };

            std::vector<std::thread> threads;
            threads.emplace_back([&start_channel, &graph]() {
                auto ordering = DegeneracyOrderIter<VertexSet>::degeneracy_ordering(graph, -1);
                while (auto next = ordering.next()) {
                    start_channel.push(*next);
                }
                start_channel.close();
                });

            threads.emplace_back([&start_channel, &visit_channel, &graph]() {
                auto excluded = Util::with_capacity<VertexSet>(std::max(1u, graph.order()) - 1);
                for (Vertex v : start_channel) {
                    auto const& neighbours = graph.neighbours(v);
                    assert(!neighbours.empty());
                    auto neighbouring_candidates = Util::difference(neighbours, excluded);
                    if (neighbouring_candidates.empty()) {
                        assert(!Util::are_disjoint(neighbours, excluded));
                    }
                    else {
                        auto neighbouring_excluded = Util::intersection(neighbours, excluded);
                        visit_channel.push(VisitJob{ v, std::move(neighbouring_candidates), std::move(neighbouring_excluded) });
                    }
                    excluded.insert(v);
                }
                visit_channel.close();
                });

            std::atomic_int visitorsLive = NUM_VISITING_FIBERS;
            for (int i = 0; i < NUM_VISITING_FIBERS; ++i) {
                threads.emplace_back([&visit_channel, &report_channel, &visitorsLive, &graph]() {
                    SendingReporter thread_reporter{ report_channel };
                    for (VisitJob& job : visit_channel) {
                        auto pile = VertexPile{ job.start };
                        BronKerboschPivot::visit<VertexSet>(
                            graph,
                            thread_reporter,
                            PivotChoice::MaxDegreeLocal,
                            PivotChoice::MaxDegreeLocal,
                            std::move(job.candidates),
                            std::move(job.excluded),
                            &pile);
                    }
                    if (visitorsLive.fetch_sub(1) == 1)
                        report_channel.close();
                    });
            }

            threads.emplace_back([&report_channel, &reporter]() {
                for (auto clique : report_channel) {
                    reporter.record(std::move(clique));
                }
                });

            for (auto& thread : threads) {
                thread.join();
            }
            assert(visitorsLive.load() == 0);
        }
    };

}