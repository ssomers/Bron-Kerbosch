//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

#pragma once

#include "BronKerboschDegeneracy.h"
#include "CliqueList.h"

namespace BronKerbosch {
    class BronKerbosch3GPX {
       public:
        template <typename Reporter, typename VertexSet>
        static Reporter::Result explore(UndirectedGraph<VertexSet> const& graph) {
            return BronKerboschDegeneracy::explore<Reporter>(graph, PivotChoice::MaxDegreeLocalX);
        }
    };
}
