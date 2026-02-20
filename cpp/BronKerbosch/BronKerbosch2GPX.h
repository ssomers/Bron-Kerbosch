//! Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

#pragma once

#include "BronKerboschPivot.h"

namespace BronKerbosch {
    class BronKerbosch2GPX {
      public:
        template <typename Reporter, typename VertexSet>
        static Reporter::Result explore(UndirectedGraph<VertexSet> const& graph) {
            return BronKerboschPivot::explore<Reporter>(graph, PivotChoice::MaxDegreeLocalX);
        }
    };
}
