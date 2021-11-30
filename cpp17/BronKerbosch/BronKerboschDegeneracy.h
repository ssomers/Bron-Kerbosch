//! Bron-Kerbosch algorithm with degeneracy ordering,
//! parametrized by the way nested searches choose a pivot.

#pragma once

#include "BronKerboschPivot.h"
#include "CliqueList.h"
#include "GraphDegeneracy.h"
#include "UndirectedGraph.h"
#include "Util.h"

namespace BronKerbosch {
    class BronKerboschDegeneracy {
       public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph,
                                  PivotChoice pivot_choice) {
            auto cliques = CliqueList{};
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            auto excluded = Util::with_capacity<VertexSet>(std::max(1u, graph.order()) - 1);
            auto ordering = DegeneracyOrderIter<VertexSet>::degeneracy_ordering(graph, -1);
            while (auto next = ordering.next()) {
                Vertex v = *next;
                auto const& neighbours = graph.neighbours(v);
                assert(!neighbours.empty());
                auto neighbouring_excluded = Util::intersection(neighbours, excluded);
                if (neighbouring_excluded.size() < neighbours.size()) {
                    auto neighbouring_candidates = Util::difference(neighbours, neighbouring_excluded);
                    auto pile = VertexPile{v};
                    cliques.splice(cliques.end(), BronKerboschPivot::visit(
                                                      graph, pivot_choice, pivot_choice,
                                                      std::move(neighbouring_candidates),
                                                      std::move(neighbouring_excluded), &pile));
                }
                excluded.insert(v);
            }
            return cliques;
        }
    };
}
