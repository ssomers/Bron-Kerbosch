//! Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

#pragma once

#include "BronKerboschPivot.h"

namespace BronKerbosch {
    class BronKerbosch2GP {
       public:
        template <typename Reporter, typename VertexSet>
        static Reporter::Result explore(UndirectedGraph<VertexSet> const& graph) {
            return BronKerboschPivot::explore<Reporter>(graph, PivotChoice::MaxDegreeLocal);
        }
    };
}
