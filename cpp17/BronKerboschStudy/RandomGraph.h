#pragma once

#include "BronKerbosch/UndirectedGraph.h"

namespace BronKerboschStudy {
    using BronKerbosch::UndirectedGraph;

    struct RandomGraph {
        static unsigned parseInt(std::string const& orderstr) {
            unsigned factor = 1;
            if (*orderstr.rbegin() == 'M')
                factor = 1'000'000;
            if (*orderstr.rbegin() == 'k')
                factor = 1'000;
            auto i = std::stoi(orderstr);
            if (i < 0) {
                std::cerr << orderstr << " is negative\n";
                std::exit(EXIT_FAILURE);
            }
            return unsigned(i) * factor;
        }

        template <typename VertexSet>
        static UndirectedGraph<VertexSet> readUndirected(std::string const& orderstr, unsigned size) {
            unsigned order = parseInt(orderstr);
            unsigned long fully_meshed_size = (long) order * (order - 1) / 2;
            if (size > fully_meshed_size) {
                std::cerr << order << " nodes accommodate at most " << fully_meshed_size << " edges\n";
                std::exit(EXIT_FAILURE);
            }

            auto path = "..\\random_edges_order_" + orderstr + ".txt";
            unsigned linenum = 0;
            std::vector<VertexSet> adjacencies(order);
            {
                std::ifstream file(path.c_str());
                if (!file) {
                    std::cerr << "Missing " << path << "\n";
                    std::exit(EXIT_FAILURE);
                }
                for (; linenum < size; ++linenum) {
                    int v, w;
                    if (!(file >> v >> w)) {
                        break;
                    }
                    auto added1 = adjacencies[v].insert(w).second;
                    auto added2 = adjacencies[w].insert(v).second;
                    assert(added1);
                    assert(added2);
                }
            }
            if (linenum < size) {
                std::cerr << "Exhausted generated list of " << linenum << " edges in " << path << "\n";
                std::exit(EXIT_FAILURE);
            }
            auto g = UndirectedGraph<VertexSet>{ std::move(adjacencies) };
            assert(g.order() == order);
            assert(g.size() == size);
            return g;
        }
    };
}
