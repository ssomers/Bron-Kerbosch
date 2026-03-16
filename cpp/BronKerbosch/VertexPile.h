#pragma once

#include "Vertex.h"
#include <vector>

namespace BronKerbosch {
    class VertexPile {
      public:
        unsigned const height;

        /// Create a pile, optionally on top of an existing pile
        explicit VertexPile(Vertex v, const VertexPile* lower = NULL)
            : height(lower ? lower->height + 1 : 1), top(v), lower(lower) {
        }

        /// Clone contained elements into a vector, in the order they were placed
        std::vector<Vertex> collect() const {
            auto result = std::vector<Vertex>{};
            result.reserve(height);
            push_to(result);
            assert(result.size() == height);
            return result;
        }

      private:
        void push_to(std::vector<Vertex>& result) const {
            if (lower)
                lower->push_to(result);
            result.push_back(top);
        }

        Vertex const top;
        const VertexPile* const lower;
    };
}
