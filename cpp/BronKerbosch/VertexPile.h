#pragma once

#include <vector>
#include "Vertex.h"

namespace BronKerbosch {
    class VertexPile {
       public:
        /// Create a pile, optionally on top of an existing pile
        explicit VertexPile(Vertex v, const VertexPile* lower = NULL)
            : top(v), height(lower ? lower->height : 1), lower(lower) {
        }

        /// Clone contained elements into a vector, in the order they were placed
        std::vector<Vertex> collect() const {
            auto result = std::vector<Vertex>{};
            result.reserve(height);
            push_to(result);
            return result;
        }

       private:
        void push_to(std::vector<Vertex>& result) const {
            if (lower)
                lower->push_to(result);
            result.push_back(top);
        }

        Vertex const top;
        unsigned const height;
        const VertexPile* const lower;
    };
}
