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
            for (auto elt : lhs) {
                if (rhs.count(elt) == 0) {
                    result.insert(elt);
                }
            }
            return result;
        }
        template <>
        static std::set<Vertex> difference(std::set<Vertex> const& lhs, std::set<Vertex> const& rhs) {
            std::set<Vertex> result;
            std::set_difference(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(), std::inserter(result, result.end()));
            return result;
        }

        template <typename VertexSet>
        static size_t intersectSize(VertexSet const& lhs, VertexSet const& rhs) {
            if (lhs.size() > rhs.size()) {
                return intersectSize(rhs, lhs);
            }
            size_t count = 0;
            for (auto elt : lhs) {
                count += rhs.count(elt);
            }
            return count;
        }
        template <>
        static size_t intersectSize(std::set<Vertex> const& lhs, std::set<Vertex> const& rhs) {
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
            if (lhs.size() > rhs.size()) {
                return intersection(rhs, lhs);
            }
            VertexSet result;
            for (auto elt : lhs) {
                if (rhs.count(elt)) {
                    result.insert(elt);
                }
            }
            return result;
        }
        template <>
        static std::set<Vertex> intersection(std::set<Vertex> const& lhs, std::set<Vertex> const& rhs) {
            std::set<Vertex> result;
            std::set_intersection(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(), std::inserter(result, result.end()));
            return result;
        }

    };
}