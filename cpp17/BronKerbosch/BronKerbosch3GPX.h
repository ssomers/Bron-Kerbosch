//! Bron-Kerbosch algorithm with degeneracy ordering, with nested searches
//! choosing a pivot from both candidates and excluded vertices (IK_GPX)

#pragma once

#include "BronKerboschDegeneracy.h"

namespace BronKerbosch {
    class BronKerbosch3GPX {
    public:
        template <typename VertexSet, typename Reporter>
        static void explore(UndirectedGraph<VertexSet> const& graph, Reporter& reporter) {
            BronKerboschDegeneracy::explore(graph, reporter, PivotChoice::MaxDegreeLocalX);
        }
    };
}

