//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from candidates only (IK_GP)

#pragma once

#include "BronKerboschDegeneracy.h"
#include "CliqueList.h"

namespace BronKerbosch {
    class BronKerbosch3GP {
    public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph) {
            return BronKerboschDegeneracy::explore(graph, PivotChoice::MaxDegreeLocal);
        }
    };
}

