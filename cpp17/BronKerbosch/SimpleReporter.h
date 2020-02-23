#pragma once

#include "VertexPile.h"
#include <vector>

namespace BronKerbosch {
    using VertexList = std::vector<Vertex>;

    class SimpleReporter {
    public:
        std::vector<std::vector<Vertex>> cliques;

        void record(VertexPile &&pile) {
            std::vector<Vertex> clique = pile.collect();
            assert(clique.size() > 1);
            cliques.push_back(clique);
        }
    };
}
