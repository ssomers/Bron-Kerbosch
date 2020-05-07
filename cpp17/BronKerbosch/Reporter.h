#pragma once

#include "VertexPile.h"
#include <vector>

namespace BronKerbosch {
    using VertexList = std::vector<Vertex>;

    class SimpleReporter {
    public:
        std::vector<std::vector<Vertex>> cliques;

        void record(VertexList&& clique) {
            assert(clique.size() > 1);
            cliques.push_back(clique);
        }
    };

    class CountingReporter {
    public:
        size_t cliques = 0;

        void record(VertexList&&) {
            cliques += 1;
        }
    };
}
