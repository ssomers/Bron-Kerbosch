#pragma once

#include <vector>
#include "BronKerbosch1.h"
#include "BronKerbosch2.h"
#include "BronKerbosch2GP.h"
#include "BronKerbosch2GPX.h"
#include "BronKerbosch3.h"
#include "BronKerbosch3GP.h"
#include "BronKerbosch3GPX.h"
#ifdef CPPCORO_WORKS
#include "BronKerbosch3MT.h"
#endif
#include "UndirectedGraph.h"

namespace BronKerbosch {
    class Portfolio {
       public:
#ifdef CPPCORO_WORKS
        static int const NUM_FUNCS = 8;
#else
        static int const NUM_FUNCS = 7;
#endif
        static const char* const FUNC_NAMES[NUM_FUNCS];

        template <typename VertexSet>
        static CliqueList explore(int func_index, UndirectedGraph<VertexSet> const& graph) {
            switch (func_index) {
                case 0: return BronKerbosch1::explore(graph);
                case 1: return BronKerbosch2::explore(graph);
                case 2: return BronKerbosch2GP::explore(graph);
                case 3: return BronKerbosch2GPX::explore(graph);
                case 4: return BronKerbosch3::explore(graph);
                case 5: return BronKerbosch3GP::explore(graph);
                case 6: return BronKerbosch3GPX::explore(graph);
#ifdef CPPCORO_WORKS
                case 7: return BronKerbosch3MT<VertexSet>::explore(graph);
#endif
            }
            throw std::logic_error("invalid func_index");
        }

        static void sort_cliques(std::vector<std::vector<Vertex>>& cliques);

       private:
        static bool clique_less(std::vector<Vertex> const&, std::vector<Vertex> const&);
    };
}
