//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

#pragma once

#include "BronKerboschPivot.h"
#include "CliqueList.h"

namespace BronKerbosch {
    class BronKerbosch2GP {
       public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph) {
            return BronKerboschPivot::explore(graph, PivotChoice::MaxDegreeLocal);
        }
    };
}
