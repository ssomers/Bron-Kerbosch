//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot arbitrarily

#pragma once

#include "BronKerboschPivot.h"
#include "GraphDegeneracy.h"
#include "UndirectedGraph.h"
#include "Util.h"

namespace BronKerbosch {
    class BronKerboschDegeneracy {
    public:
        template <typename VertexSet, typename Reporter>
        static void explore(UndirectedGraph<VertexSet> const& graph, Reporter& reporter, PivotChoice pivot_choice) {
            auto excluded = Util::with_capacity<VertexSet>(std::max(1u, graph.order()) - 1);
            auto ordering = DegeneracyOrderIter<VertexSet>::degeneracy_ordering(graph, -1);
            while (auto next = ordering.next()) {
                Vertex v = *next;
                auto const& neighbours = graph.neighbours(v);
                assert(!neighbours.empty());
                auto neighbouring_candidates = Util::difference(neighbours, excluded);
                if (neighbouring_candidates.empty()) {
                    assert(!Util::are_disjoint(neighbours, excluded));
                } else {
                    auto neighbouring_excluded = Util::intersection(neighbours, excluded);
                    auto pile = VertexPile{ v };
                    BronKerboschPivot::visit(
                        graph,
                        reporter,
                        pivot_choice,
                        pivot_choice,
                        std::move(neighbouring_candidates),
                        std::move(neighbouring_excluded),
                        &pile
                    );
                }
                excluded.insert(v);
            }
        }
    };
}

