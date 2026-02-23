#pragma once

#include "Util.h"
#include "Vertex.h"
#include <ranges>
#include <vector>

namespace BronKerbosch {
    template <typename VertexSet>
    class UndirectedGraph {
      public:
        using Adjacencies = std::vector<VertexSet>;

        explicit UndirectedGraph(Adjacencies&& adjacencies) : itsAdjacencies(adjacencies) {
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
            return unsigned(neighbours(v).size());
        }

        VertexSet const& neighbours(Vertex v) const {
            return itsAdjacencies[v.index()];
        }

        auto vertices() const {
            return std::ranges::iota_view{0u, order()} |
                   std::ranges::views::transform([](unsigned i) { return Vertex(i); });
        }

        auto connected_vertices() const {
            return vertices() | std::ranges::views::filter([&](Vertex v) { return degree(v) > 0; });
        }

        Vertex max_degree_vertex() const {
            assert(order() > 0);
            auto vertices_stored = vertices();
            return *std::ranges::max_element(
                vertices_stored, [&](Vertex v, Vertex w) { return degree(v) < degree(w); });
        }

        static bool are_valid_adjacencies(Adjacencies const& adjacencies) {
            auto order = adjacencies.size();
            for (unsigned i = 0; i < order; ++i) {
                auto const v = Vertex(i);
                auto const& adjacent_to_v = adjacencies[i];
                for (Vertex w : adjacent_to_v) {
                    if (w == v || w.index() >= order || adjacencies[w.index()].count(v) == 0) {
                        return false;
                    }
                }
            }
            return true;
        }

      private:
        Adjacencies const itsAdjacencies;
    };
}