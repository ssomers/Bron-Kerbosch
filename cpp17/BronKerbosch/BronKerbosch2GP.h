//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

#pragma once

#include "BronKerboschPivot.h"
#include "CliqueList.h"
#include "UndirectedGraph.h"
#include "Util.h"

namespace BronKerbosch {
    class BronKerbosch2GP {
       public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph) {
            auto candidates = graph.connected_vertices();
            auto num_candidates = candidates.size();
            if (num_candidates) {
                return BronKerboschPivot::visit(
                    graph, PivotChoice::MaxDegree, PivotChoice::MaxDegreeLocal,
                    std::move(candidates), Util::with_capacity<VertexSet>(num_candidates), NULL);
            } else {
                return CliqueList{};
            }
        }
    };
}
