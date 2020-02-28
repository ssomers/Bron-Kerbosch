//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)

#pragma once

#include "BronKerboschDegeneracy.h"

namespace BronKerbosch {
    class BronKerbosch3GP {
    public:
        template <typename VertexSet, typename Reporter>
        static void explore(UndirectedGraph<VertexSet> const& graph, Reporter& reporter) {
            BronKerboschDegeneracy::explore(graph, reporter, PivotChoice::MaxDegreeLocal);
        }
    };
}

