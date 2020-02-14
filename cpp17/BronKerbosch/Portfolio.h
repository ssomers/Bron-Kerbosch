#pragma once

#include "BronKerbosch/BronKerbosch1.h"
#include "BronKerbosch/Reporter.h"
#include "BronKerbosch/UndirectedGraph.h"

namespace BronKerbosch {
    class Portfolio {
    public:
        static int const FUNCS = 1;
        static const char* const FUNC_NAMES[FUNCS];

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

        static void sort_cliques(std::vector<VertexList>& cliques) {
            for (VertexList& clique : cliques)
                std::sort(clique.begin(), clique.end());
            std::sort(cliques.begin(), cliques.end(), &comparer);
        }

    private:
        static bool comparer(VertexList const& lhs, VertexList const& rhs) {
            for (auto i = 0; i < lhs.size() && i < rhs.size(); ++i) {
                auto d = lhs[i] - rhs[i];
                if (d != 0)
                    return d < 0;
            }
            throw std::logic_error("got overlapping or equal cliques");
        }
    };
}

