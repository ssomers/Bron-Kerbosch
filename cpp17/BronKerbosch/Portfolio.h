#pragma once

#include "BronKerbosch/BronKerbosch1.h"
#include "BronKerbosch/Reporter.h"
#include "BronKerbosch/UndirectedGraph.h"

namespace BronKerbosch {
    class Portfolio {
    public:
        static int const NUM_FUNCS = 1;
        static const char* const FUNC_NAMES[NUM_FUNCS];

        template <typename VertexSet, typename Reporter>
        static void explore(int func_index, UndirectedGraph<VertexSet> const& graph, Reporter& reporter) {
            switch (func_index) {
                case 0: return BronKerbosch1::explore(graph, reporter);
                    /*
                    case 1: BronKerbosch2G.Explore(graph, reporter); break;
                    case 2: BronKerbosch2GP.Explore(graph, reporter); break;
                    case 3: BronKerbosch2GPX.Explore(graph, reporter); break;
                    case 4: BronKerbosch3GP.Explore(graph, reporter); break;
                    case 5: BronKerbosch3GPX.Explore(graph, reporter); break;
                    */
            }
        }

        static void sort_cliques(std::vector<VertexList>& cliques);

    private:
        static bool clique_less(VertexList const&, VertexList const&);
    };
}

