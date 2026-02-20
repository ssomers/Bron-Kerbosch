//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)

#pragma once

#include "BronKerboschDegeneracy.h"

namespace BronKerbosch {
    class BronKerbosch3GP {
      public:
        template <typename Reporter, typename VertexSet>
        static Reporter::Result explore(UndirectedGraph<VertexSet> const& graph) {
            return BronKerboschDegeneracy::explore<Reporter>(graph, PivotChoice::MaxDegreeLocal);
        }
    };
}
