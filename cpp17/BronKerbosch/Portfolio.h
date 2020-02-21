#pragma once

#include "BronKerbosch/BronKerbosch1.h"
#include "BronKerbosch/BronKerbosch2.h"
#include "BronKerbosch/BronKerbosch2GP.h"
#include "BronKerbosch/BronKerbosch2GPX.h"
#include "BronKerbosch/UndirectedGraph.h"

namespace BronKerbosch {
    class Portfolio {
    public:
        static int const NUM_FUNCS = 4;
        static const char* const FUNC_NAMES[NUM_FUNCS];

        template <typename VertexSet, typename Reporter>
        static void explore(int func_index, UndirectedGraph<VertexSet> const& graph, Reporter& reporter) {
            switch (func_index) {
                case 0: return BronKerbosch1::explore(graph, reporter);
                case 1: return BronKerbosch2::explore(graph, reporter);
                case 2: return BronKerbosch2GP::explore(graph, reporter);
                case 3: return BronKerbosch2GPX::explore(graph, reporter);
            }
        }

        static void sort_cliques(std::vector<VertexList>& cliques);

    private:
        static bool clique_less(VertexList const&, VertexList const&);
    };
}

