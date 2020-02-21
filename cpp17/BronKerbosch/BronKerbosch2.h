//! Bron-Kerbosch algorithm with pivot picked arbitrarily

#pragma once

#include "pch.h"
#include "BronKerboschPivot.h"
#include "UndirectedGraph.h"
#include "Util.h"

namespace BronKerbosch {
    class BronKerbosch2 {
    public:
        template <typename VertexSet, typename Reporter>
        static void explore(UndirectedGraph<VertexSet> const& graph, Reporter& reporter) {
            auto candidates = graph.connected_vertices();
            auto num_candidates = candidates.size();
            if (num_candidates) {
                BronKerboschPivot::visit(
                    graph,
                    reporter,
                    PivotChoice::Arbitrary,
                    PivotChoice::Arbitrary,
                    std::move(candidates),
                    Util::with_capacity<VertexSet>(num_candidates),
                    VertexList{});
            }
        }
    };
}
