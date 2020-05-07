#pragma once

#include "BronKerbosch1.h"
#include "BronKerbosch2.h"
#include "BronKerbosch2GP.h"
#include "BronKerbosch2GPX.h"
#include "BronKerbosch3.h"
#include "BronKerbosch3GP.h"
#include "BronKerbosch3GPX.h"
#include "BronKerbosch3MT.h"
#include "UndirectedGraph.h"
#include <vector>

namespace BronKerbosch {
    class Portfolio {
    public:
        static int const NUM_FUNCS = 8;
        static const char* const FUNC_NAMES[NUM_FUNCS];

        template <typename VertexSet, typename Reporter>
        static void explore(int func_index, UndirectedGraph<VertexSet> const& graph, Reporter& reporter) {
            switch (func_index) {
                case 0: return BronKerbosch1::explore(graph, reporter);
                case 1: return BronKerbosch2::explore(graph, reporter);
                case 2: return BronKerbosch2GP::explore(graph, reporter);
                case 3: return BronKerbosch2GPX::explore(graph, reporter);
                case 4: return BronKerbosch3::explore(graph, reporter);
                case 5: return BronKerbosch3GP::explore(graph, reporter);
                case 6: return BronKerbosch3GPX::explore(graph, reporter);
                case 7: return BronKerbosch3MT<VertexSet>::explore(graph, reporter);
            }
            throw std::logic_error("invalid func_index");
        }

        static void sort_cliques(std::vector<std::vector<Vertex>>& cliques);

    private:
        static bool clique_less(std::vector<Vertex> const&, std::vector<Vertex> const&);
    };
}

