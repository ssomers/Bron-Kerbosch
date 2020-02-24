#pragma once

#include "Vertex.h"
#include "Util.h"
#include <vector>

namespace BronKerbosch {
    template <typename VertexSet>
    class UndirectedGraph {
    public:
        using Adjacencies = std::vector<VertexSet>;

        UndirectedGraph(Adjacencies&& adjacencies) : itsAdjacencies(adjacencies) {
            assert(UndirectedGraph::are_valid_adjacencies(itsAdjacencies));
        }

        unsigned order() const {
            return unsigned(itsAdjacencies.size());
        }

        unsigned size() const {
            size_t total = 0;
            for (auto neighbours : itsAdjacencies) {
                total += neighbours.size();
            }
            assert(total % 2 == 0);
            return unsigned(total / 2);
        }

        unsigned degree(Vertex v) const {
            return unsigned(itsAdjacencies[v].size());
        }

        VertexSet const& neighbours(Vertex v) const {
            return itsAdjacencies[v];
        }

        VertexSet connected_vertices() const {
            auto order = this->order();
            auto result = Util::with_capacity<VertexSet>(order);
            for (Vertex v = 0; v < order; ++v) {
                if (degree(v) > 0) {
                    result.insert(v);
                }
            }
            return result;
        }

        static bool are_valid_adjacencies(Adjacencies const& adjacencies) {
            auto order = adjacencies.size();
            for (Vertex v = 0; v < order; ++v) {
                auto adjacent_to_v = adjacencies[v];
                for (Vertex w : adjacent_to_v) {
                    if (w == v || w >= order || adjacencies[w].count(v) == 0) {
                        return false;
                    }
                }
            }
            return true;
        }

    private:
        Adjacencies itsAdjacencies;
    };
}