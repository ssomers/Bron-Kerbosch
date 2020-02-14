#pragma once

#include "pch.h"
#include "BronKerbosch/UndirectedGraph.h"

namespace BronKerbosch {
    struct Util {
        static VertexList append(VertexList const& clique, Vertex v) {
            auto result = VertexList(clique.size() + 1);
            std::copy(clique.begin(), clique.end(), result.begin());
            *result.rbegin() = v;
            return result;
        }

        template <typename VertexSet>
        static bool are_disjoint(VertexSet const& lhs, VertexSet const& rhs) {
            return intersectSize(lhs, rhs) == 0;
        }

        template <typename VertexSet>
        static VertexSet difference(VertexSet const& lhs, VertexSet const& rhs) {
            VertexSet result;
            std::set_difference(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(), std::back_inserter(result));
            return result;
        }

        template <typename VertexSet>
        static size_t intersectSize(VertexSet const& lhs, VertexSet const& rhs) {
            struct output_counter {
                typedef Vertex value_type;
                typedef size_t iterator;

                size_t count = 0;

                iterator insert(iterator, Vertex) {
                    return ++count;
                }
            };

            output_counter counter;
            std::set_intersection(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(), std::inserter(counter, 0));
            return counter.count;
        }

        template <typename VertexSet>
        static VertexSet intersection(VertexSet const& lhs, VertexSet const& rhs) {
            VertexSet result;
            std::set_intersection(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(), std::inserter(result, result.end()));
            return result;
        }
    };
}