// Naive Bron-Kerbosch algorithm

#pragma once

#include "CliqueList.h"
#include "UndirectedGraph.h"
#include "Util.h"
#include "VertexPile.h"

namespace BronKerbosch {
    class BronKerbosch1 {
    public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph)
        {
            auto candidates = graph.connected_vertices();
            auto num_candidates = candidates.size();
            if (num_candidates) {
                return visit(
                    graph,
                    std::move(candidates),
                    Util::with_capacity<VertexSet>(num_candidates),
                    nullptr);
            } else {
                return CliqueList{};
            }
        }

        template <typename VertexSet>
        static CliqueList visit(UndirectedGraph<VertexSet> const& graph,
            VertexSet&& candidates, VertexSet&& excluded, const VertexPile* clique)
        {
            auto cliques = CliqueList{};
            assert(!candidates.empty());
            for (;;) {
                Vertex v = Util::pop_arbitrary(candidates);
                auto const& neighbours = graph.neighbours(v);
                assert(!neighbours.empty());
                auto neighbouring_candidates = Util::intersection(candidates, neighbours);
                if (!neighbouring_candidates.empty()) {
                    auto neighbouring_excluded = Util::intersection(excluded, neighbours);
                    auto newclique = VertexPile { v, clique };
                    cliques.splice(cliques.end(), visit(graph,
                        std::move(neighbouring_candidates),
                        std::move(neighbouring_excluded),
                        &newclique));
                } else {
                    if (Util::are_disjoint(excluded, neighbours))
                        cliques.push_back(VertexPile(v, clique).collect());
                    if (candidates.empty())
                        break;
                }
                excluded.insert(v);
            }
            return cliques;
        }
    };
}
