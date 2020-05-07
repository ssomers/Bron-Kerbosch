// Naive Bron-Kerbosch algorithm

#pragma once

#include "UndirectedGraph.h"
#include "VertexPile.h"
#include "Util.h"

namespace BronKerbosch {
    class BronKerbosch1 {
    public:
        template <typename VertexSet, typename Reporter>
        static void explore(UndirectedGraph<VertexSet> const& graph, Reporter& reporter) {
            auto candidates = graph.connected_vertices();
            auto num_candidates = candidates.size();
            if (num_candidates) {
                visit(
                    graph,
                    reporter,
                    std::move(candidates),
                    Util::with_capacity<VertexSet>(num_candidates),
                    nullptr);
            }
        }

        template <typename VertexSet, typename Reporter>
        static void visit(UndirectedGraph<VertexSet> const& graph, Reporter& reporter,
                          VertexSet &&candidates, VertexSet &&excluded, const VertexPile* clique) {
            assert(!candidates.empty());
            for (;;) {
                Vertex v = Util::pop_arbitrary(candidates);
                auto const& neighbours = graph.neighbours(v);
                assert(!neighbours.empty());
                auto neighbouring_candidates = Util::intersection(candidates, neighbours);
                if (!neighbouring_candidates.empty()) {
                    auto neighbouring_excluded = Util::intersection(excluded, neighbours);
                    auto newclique = VertexPile{ v, clique };
                    visit(graph, reporter,
                          std::move(neighbouring_candidates),
                          std::move(neighbouring_excluded),
                          &newclique);
                } else {
                    if (Util::are_disjoint(excluded, neighbours))
                        reporter.record(VertexPile(v, clique).collect());
                    if (candidates.empty())
                        break;
                }
                excluded.insert(v);
            }
        }
    };
}
