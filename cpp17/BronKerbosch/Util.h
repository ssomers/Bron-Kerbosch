#pragma once

#include <set>
#include <unordered_set>
#include <vector>

namespace BronKerbosch {
    struct Util {
        template <typename T>
        static std::vector<T> append(std::vector<T> const& vec, T val) {
            auto result = std::vector<T>(vec.size() + 1);
            std::copy(vec.begin(), vec.end(), result.begin());
            *result.rbegin() = val;
            return result;
        }

        template <typename Set>
        static bool are_disjoint(Set const& lhs, Set const& rhs) {
            return intersectSize(lhs, rhs) == 0;
        }

        template <typename T>
        static std::set<T> difference(std::set<T> const& lhs, std::set<T> const& rhs) {
            std::set<T> result;
            std::set_difference(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(), std::inserter(result, result.end()));
            return result;
        }
        template <typename T>
        static std::unordered_set<T> difference(std::unordered_set<T> const& lhs, std::unordered_set<T> const& rhs) {
            std::unordered_set<T> result;
            result.reserve(lhs.size());
            for (auto elt : lhs) {
                if (rhs.count(elt) == 0) {
                    result.insert(elt);
                }
            }
            return result;
        }

        template <typename T>
        static size_t intersectSize(std::set<T> const& lhs, std::set<T> const& rhs) {
            struct output_counter {
                typedef T value_type;
                typedef size_t iterator;

                size_t count = 0;

                iterator insert(iterator, T) {
                    return ++count;
                }
            };

            output_counter counter;
            std::set_intersection(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(), std::inserter(counter, 0));
            return counter.count;
        }
        template <typename T>
        static size_t intersectSize(std::unordered_set<T> const& lhs, std::unordered_set<T> const& rhs) {
            if (lhs.size() > rhs.size()) {
                return intersectSize(rhs, lhs);
            }
            size_t count = 0;
            for (auto elt : lhs) {
                count += rhs.count(elt);
            }
            return count;
        }

        template <typename T>
        static std::set<T> intersection(std::set<T> const& lhs, std::set<T> const& rhs) {
            std::set<T> result;
            std::set_intersection(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(), std::inserter(result, result.end()));
            return result;
        }
        template <typename T>
        static std::unordered_set<T> intersection(std::unordered_set<T> const& lhs, std::unordered_set<T> const& rhs) {
            if (lhs.size() > rhs.size()) {
                return intersection(rhs, lhs);
            }
            std::unordered_set<T> result;
            result.reserve(lhs.size());
            for (auto elt : lhs) {
                if (rhs.count(elt)) {
                    result.insert(elt);
                }
            }
            return result;
        }

        template <typename Set>
        static Set with_capacity(size_t capacity) {
            Set result;
            Util::reserve(result, capacity);
            return result;
        }

        template <typename T>
        static void reserve(std::set<T>&, size_t) {
        }
        template <typename T>
        static void reserve(std::unordered_set<T>& set, size_t capacity) {
            set.reserve(capacity);
        }
    };
}