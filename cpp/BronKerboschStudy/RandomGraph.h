#pragma once

#include "BronKerbosch/UndirectedGraph.h"

namespace BronKerboschStudy {
    unsigned parseInt(std::string const&);

    using BronKerbosch::UndirectedGraph;

    template <typename VertexSet>
    class RandomGraph : public UndirectedGraph<VertexSet> {
       public:
        const size_t clique_count;

       private:
        RandomGraph(std::vector<VertexSet>&& adjacencies, size_t clique_count)
            : UndirectedGraph<VertexSet>(std::move(adjacencies)), clique_count(clique_count) {
        }

       public:
        static RandomGraph<VertexSet> readUndirected(std::string const& orderstr, unsigned size) {
            unsigned order = parseInt(orderstr);
            auto fully_meshed_size = (unsigned long)order * (order - 1) / 2;
            if (size > fully_meshed_size) {
                std::cerr << order << " nodes accommodate at most " << fully_meshed_size
                          << " edges\n";
                std::exit(EXIT_FAILURE);
            }

            auto edges_path = std::string("..\\data\\random_edges_order_") + orderstr + ".txt";
            auto stats_path = std::string("..\\data\\random_stats.txt");
            auto adjacencies = readEdges(edges_path, orderstr, size);
            auto expected_clique_count = readStats(stats_path, orderstr, size);
            auto g = RandomGraph<VertexSet>{std::move(adjacencies), expected_clique_count};
            if (g.order() != order || g.size() != size) {
                std::cerr << "Messed up while reading " << edges_path << "\n";
                std::exit(EXIT_FAILURE);
            }
            return g;
        }

       private:
        static std::vector<VertexSet> readEdges(std::string const& path,
                                                std::string const& orderstr,
                                                unsigned size) {
            unsigned order = parseInt(orderstr);
            unsigned line_idx = 0;
            std::vector<VertexSet> adjacencies(order);
            {
                std::ifstream file(path.c_str());
                if (!file) {
                    std::cerr << "Missing " << path << "\n";
                    std::exit(EXIT_FAILURE);
                }
                for (; line_idx < size; ++line_idx) {
                    int v, w;
                    if (!(file >> v >> w)) {
                        break;
                    }
                    auto added1 = adjacencies[v].insert(w).second;
                    auto added2 = adjacencies[w].insert(v).second;
                    if (!added1 || !added2) {
                        std::cerr << "Corrupt " << path << "\n";
                        std::exit(EXIT_FAILURE);
                    }
                }
            }
            if (line_idx < size) {
                std::cerr << "Exhausted generated list of " << line_idx << " edges in " << path
                          << "\n";
                std::exit(EXIT_FAILURE);
            }
            return adjacencies;
        }

        static size_t readStats(std::string const& path,
                                std::string const& orderstr,
                                unsigned size) {
            std::ifstream file(path.c_str());
            if (!file) {
                std::cerr << "Missing " << path << "\n";
                std::exit(EXIT_FAILURE);
            }
            std::string header;
            getline(file, header);
            std::string o;
            unsigned s;
            size_t clique_count;
            while ((file >> o >> s >> clique_count)) {
                if (o == orderstr && s == size) {
                    return clique_count;
                }
            }
            std::cerr << "Missing entry in " << path << "\n";
            std::exit(EXIT_FAILURE);
        }
    };
}
