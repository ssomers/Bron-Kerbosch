//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

#pragma once

#include "pch.h"
#include "BronKerboschPivot.h"
#include "UndirectedGraph.h"
#include "Util.h"

namespace BronKerbosch {
    class BronKerbosch2GP {
    public:
        template <typename VertexSet, typename Reporter>
        static void explore(UndirectedGraph<VertexSet> const& graph, Reporter& reporter) {
            auto candidates = graph.connected_vertices();
            auto num_candidates = candidates.size();
            if (num_candidates) {
                BronKerboschPivot::visit(
                    graph,
                    reporter,
                    PivotChoice::MaxDegree,
                    PivotChoice::MaxDegreeLocal,
                    std::move(candidates),
                    Util::with_capacity<VertexSet>(num_candidates),
                    VertexList{});
            }
        }
    };
}
