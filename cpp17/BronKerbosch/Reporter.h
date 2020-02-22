#pragma once

#include "BronKerbosch/UndirectedGraph.h"
#include <vector>

namespace BronKerbosch {
    using VertexList = std::vector<Vertex>;

    class SimpleReporter {
    public:
        std::vector<VertexList> cliques;

        void record(VertexList&& clique) {
            assert(clique.size() > 1);
            cliques.push_back(clique);
        }
    };
}
