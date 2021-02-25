//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

#pragma once

#include "BronKerboschDegeneracy.h"
#include "CliqueList.h"

namespace BronKerbosch {
    class BronKerbosch3GPX {
    public:
        template <typename VertexSet>
        static CliqueList explore(UndirectedGraph<VertexSet> const& graph) {
            return BronKerboschDegeneracy::explore(graph, PivotChoice::MaxDegreeLocalX);
        }
    };
}

