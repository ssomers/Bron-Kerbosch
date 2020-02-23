//! Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

#pragma once

#include "BronKerboschPivot.h"
#include "UndirectedGraph.h"
#include "Util.h"

namespace BronKerbosch {
    class BronKerbosch2GPX {
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
                    PivotChoice::MaxDegreeLocalX,
                    std::move(candidates),
                    Util::with_capacity<VertexSet>(num_candidates),
                    NULL);
            }
        }
    };
}
