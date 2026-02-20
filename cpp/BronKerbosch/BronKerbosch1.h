// Naive Bron-Kerbosch algorithm

#pragma once

#include "UndirectedGraph.h"
#include "Util.h"
#include "VertexPile.h"

namespace BronKerbosch {
    class BronKerbosch1 {
      public:
        template <typename Reporter, typename VertexSet>
        static Reporter::Result explore(UndirectedGraph<VertexSet> const& graph) {
            auto candidates = graph.connected_vertices();
            auto num_candidates = candidates.size();
            if (num_candidates) {
                return visit<Reporter>(graph, std::move(candidates),
                                       Util::with_capacity<VertexSet>(num_candidates), nullptr);
            } else {
                return Reporter::empty();
            }
        }

        template <typename Reporter, typename VertexSet>
        static Reporter::Result visit(UndirectedGraph<VertexSet> const& graph,
                                      VertexSet&& candidates,
                                      VertexSet&& excluded,
                                      const VertexPile* clique) {
            assert(!candidates.empty());
            auto cliques = Reporter::empty();
            for (;;) {
                Vertex v = Util::pop_arbitrary(candidates);
                auto const& neighbours = graph.neighbours(v);
                assert(!neighbours.empty());
                auto neighbouring_candidates = Util::intersection(candidates, neighbours);
                if (!neighbouring_candidates.empty()) {
                    auto neighbouring_excluded = Util::intersection(excluded, neighbours);
                    auto newclique = VertexPile{v, clique};
                    Reporter::add_all(
                        cliques, visit<Reporter>(graph, std::move(neighbouring_candidates),
                                                 std::move(neighbouring_excluded), &newclique));
                } else {
                    if (Util::are_disjoint(excluded, neighbours))
                        Reporter::add_one(cliques, VertexPile(v, clique));
                    if (candidates.empty())
                        break;
                }
                excluded.insert(v);
            }
            return cliques;
        }
    };
}
