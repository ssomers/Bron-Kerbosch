#pragma once

#include "pch.h"
#include "BronKerbosch/UndirectedGraph.h"

namespace BronKerbosch {
    using VertexList = std::vector<Vertex>;

    class SimpleReporter {
    public:
        std::vector<VertexList> cliques;

        void record(VertexList&& clique);
    };
}
