//! Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

#pragma once

#include "BronKerboschPivot.h"
#include "CliqueList.h"

namespace BronKerbosch {
    class BronKerbosch2GPX {
       public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph) {
            return BronKerboschPivot::explore(graph, PivotChoice::MaxDegreeLocalX);
        }
    };
}
