// Naive Bron-Kerbosch algorithm

#pragma once

#include "pch.h"
#include "UndirectedGraph.h"
#include "Reporter.h"
#include "Util.h"

namespace BronKerbosch {

    class BronKerbosch1 {
    public:
        template <typename VertexSet, typename Reporter>
        static void explore(UndirectedGraph<VertexSet> const& graph, Reporter& reporter) {
            auto candidates = graph.connected_vertices();
            if (!candidates.empty()) {
                visit(
                    graph,
                    reporter,
                    std::move(candidates),
                    VertexSet{},
                    VertexList{});
            }
        }

        template <typename VertexSet, typename Reporter>
        static void visit(UndirectedGraph<VertexSet> const& graph, Reporter& reporter,
                          VertexSet candidates, VertexSet excluded, VertexList clique) {
            assert(!candidates.empty());
            for (;;) {
                auto choice = candidates.begin();
                Vertex v = *choice;
                candidates.erase(choice);
                auto neighbours = graph.neighbours(v);
                assert(!neighbours.empty());
                auto neighbouring_candidates = Util::intersection(candidates, neighbours);
                if (!neighbouring_candidates.empty()) {
                    auto neighbouring_excluded = Util::intersection(excluded, neighbours);
                    visit(graph, reporter,
                          std::move(neighbouring_candidates),
                          std::move(neighbouring_excluded),
                          Util::append(clique, v));
                } else {
                    if (Util::are_disjoint(excluded, neighbours))
                        reporter.record(Util::append(clique, v));
                    if (candidates.empty())
                        break;
                }
                excluded.insert(v);
            }
        }
    };
}
