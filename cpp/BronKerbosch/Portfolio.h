#pragma once

#include "BronKerbosch1.h"
#include "BronKerbosch2GP.h"
#include "BronKerbosch2GPX.h"
#include "BronKerbosch3GP.h"
#include "BronKerbosch3GPX.h"
#include "BronKerbosch3MT.h"
#include "UndirectedGraph.h"
#include <vector>

namespace BronKerbosch {
    class Portfolio {
      public:
        static int const NUM_FUNCS = 6;
        static const char* const FUNC_NAMES[NUM_FUNCS];

        struct CollectingReporter {
            using Result = CliqueList;

            static Result empty() {
                return CliqueList{};
            }

            static void add_one(Result& cliques, VertexPile&& pile) {
                cliques.push_back(pile.collect());
            }

            static void add_all(Result& cliques, Result&& more_cliques) {
                cliques.splice(cliques.end(), more_cliques);
            }
        };

        struct CountingReporter {
            using Result = size_t;

            static Result empty() {
                return 0;
            }

            static void add_one(Result& cliques, VertexPile&&) {
                cliques += 1;
            }

            static void add_all(Result& cliques, Result&& more_cliques) {
                cliques += more_cliques;
            }
        };

        template <typename Reporter, typename VertexSet>
        static typename Reporter::Result explore(int func_index,
                                                 UndirectedGraph<VertexSet> const& graph) {
            switch (func_index) {
                case 0: return BronKerbosch1::explore<Reporter>(graph);
                case 1: return BronKerbosch2GP::explore<Reporter>(graph);
                case 2: return BronKerbosch2GPX::explore<Reporter>(graph);
                case 3: return BronKerbosch3GP::explore<Reporter>(graph);
                case 4: return BronKerbosch3GPX::explore<Reporter>(graph);
                case 5: return BronKerbosch3MT<Reporter, VertexSet>::explore(graph);
            }
            throw std::logic_error("invalid func_index");
        }

        static void sort_cliques(std::vector<std::vector<Vertex>>& cliques);

      private:
        static bool clique_less(std::vector<Vertex> const&, std::vector<Vertex> const&);
    };
}
