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
        template <typename Reporter, typename VertexSet>
        static Reporter::Result explore(UndirectedGraph<VertexSet> const& graph,
                                        PivotChoice pivot_choice) {
            auto cliques = Reporter::empty();
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            auto degeneracy = DegeneracyIter<VertexSet>{graph};
            while (auto next = degeneracy.next()) {
                auto pair = *next;
                Vertex v = pair.first;
                VertexSet neighbouring_excluded = std::move(pair.second);
                auto const& neighbours = graph.neighbours(v);
                assert(!neighbours.empty());
                if (neighbouring_excluded.size() < neighbours.size()) {
                    auto neighbouring_candidates =
                        Util::difference(neighbours, neighbouring_excluded);
                    auto pile = VertexPile{v};
                    Reporter::add_all(cliques, BronKerboschPivot::visit<Reporter>(
                                                   graph, pivot_choice, pivot_choice,
                                                   std::move(neighbouring_candidates),
                                                   std::move(neighbouring_excluded), &pile));
                }
            }
            return cliques;
        }
    };
}
