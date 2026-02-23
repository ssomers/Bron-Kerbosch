#pragma once

#include "Util.h"
#include "Vertex.h"
#include <algorithm>
#include <ranges>
#include <vector>

namespace BronKerbosch {
    template <typename VertexSet>
    class UndirectedGraph {
      public:
        using Adjacencies = std::vector<VertexSet>;

        explicit UndirectedGraph(Adjacencies&& adjacencies)
            : itsAdjacencies(assert_valid_adjacencies(adjacencies)),
              itsSize(calc_size(itsAdjacencies)),
              itsMaxDegree(calc_max_degree(itsAdjacencies)) {
        }

        unsigned order() const {
            return unsigned(itsAdjacencies.size());
        }

        unsigned size() const {
            return itsSize;
        }

        unsigned max_degree() const {
            return itsMaxDegree;
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

        auto max_degree_vertices() const {
            return vertices() |
                   std::ranges::views::filter([&](Vertex v) { return degree(v) == itsMaxDegree; });
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

        static unsigned calc_size(Adjacencies const& adjacencies) {
            size_t total = 0;
            for (auto neighbours : adjacencies) {
                total += neighbours.size();
            }
            assert(total % 2 == 0);
            return unsigned(total / 2);
        }

        static unsigned calc_max_degree(Adjacencies const& adjacencies) {
            unsigned max_degree = 0;
            for (auto neighbours : adjacencies) {
                max_degree = std::max(max_degree, unsigned(neighbours.size()));
            }
            return max_degree;
        }

        static Adjacencies&& assert_valid_adjacencies(Adjacencies& adjacencies) {
            assert(are_valid_adjacencies(adjacencies));
            return std::move(adjacencies);
        }

      private:
        Adjacencies const itsAdjacencies;
        unsigned const itsSize;
        unsigned const itsMaxDegree;
    };
}