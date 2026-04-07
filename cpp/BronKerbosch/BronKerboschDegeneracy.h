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
                Vertex v = *next;
                auto const& neighbours = graph.neighbours(v);
                assert(!neighbours.empty());
                VertexSet neighbouring_candidates = Util::with_capacity<VertexSet>(neighbours.size());
                VertexSet neighbouring_excluded = Util::with_capacity<VertexSet>(neighbours.size() - 1);
                for (Vertex w : neighbours) {
                    if (degeneracy.is_candidate(w)) {
                        neighbouring_candidates.insert(w);
                    } else {
                        neighbouring_excluded.insert(w);
                    }
                }
                assert(!neighbouring_candidates.empty());
                auto pile = VertexPile{v};
                Reporter::add_all(cliques, BronKerboschPivot::visit<Reporter>(
                                                graph, pivot_choice, pivot_choice,
                                                std::move(neighbouring_candidates),
                                                std::move(neighbouring_excluded), &pile));
            }
            return cliques;
        }
    };
}
