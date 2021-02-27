//! Bron-Kerbosch algorithm with pivot picked arbitrarily

#pragma once

#include "BronKerboschPivot.h"
#include "UndirectedGraph.h"
#include "Util.h"

namespace BronKerbosch {
    class BronKerbosch2 {
       public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph) {
            auto candidates = graph.connected_vertices();
            auto num_candidates = candidates.size();
            if (num_candidates) {
                return BronKerboschPivot::visit(
                    graph, PivotChoice::Arbitrary, PivotChoice::Arbitrary, std::move(candidates),
                    Util::with_capacity<VertexSet>(num_candidates), NULL);
            } else {
                return CliqueList{};
            }
        }
    };
}
