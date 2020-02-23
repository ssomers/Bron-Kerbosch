#pragma once

#include <set>
#include <unordered_set>

namespace BronKerbosch {
    struct Util {
        template <typename T>
        static bool are_disjoint(std::set<T> const& lhs, std::set<T> const& rhs) {
            auto lit = lhs.begin();
            auto rit = rhs.begin();
            while (lit != lhs.end() && rit != rhs.end()) {
                if (*lit < *rit) {
                    ++lit;
                } else if (*rit < *lit) {
                    ++rit;
                } else {
                    return false;
                }
            }
            return true;
        }
        template <typename T>
        static bool are_disjoint(std::unordered_set<T> const& lhs,
                                 std::unordered_set<T> const& rhs) {
            if (lhs.size() > rhs.size()) {
                return are_disjoint(rhs, lhs);
            }
            for (auto elt : lhs) {
                if (rhs.count(elt)) {
                    return false;
                }
            }
            return true;
        }

        template <typename T>
        static size_t intersection_size(std::set<T> const& lhs, std::set<T> const& rhs) {
            size_t count = 0;
            auto lit = lhs.begin();
            auto rit = rhs.begin();
            while (lit != lhs.end() && rit != rhs.end()) {
                if (*lit < *rit) {
                    ++lit;
                } else if (*rit < *lit) {
                    ++rit;
                } else {
                    ++count;
                    ++lit;
                    ++rit;
                }
            }
            return count;
        }
        template <typename T>
        static size_t intersection_size(std::unordered_set<T> const& lhs,
                                        std::unordered_set<T> const& rhs) {
            if (lhs.size() > rhs.size()) {
                return intersection_size(rhs, lhs);
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
            std::set_intersection(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(),
                                  std::inserter(result, result.end()));
            return result;
        }
        template <typename T>
        static std::unordered_set<T> intersection(std::unordered_set<T> const& lhs,
                                                  std::unordered_set<T> const& rhs) {
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

        template <typename T>
        static std::set<T> difference(std::set<T> const& lhs, std::set<T> const& rhs) {
            std::set<T> result;
            std::set_difference(lhs.begin(), lhs.end(), rhs.begin(), rhs.end(),
                                std::inserter(result, result.end()));
            return result;
        }
        template <typename T>
        static std::unordered_set<T> difference(std::unordered_set<T> const& lhs,
                                                std::unordered_set<T> const& rhs) {
            std::unordered_set<T> result;
            result.reserve(lhs.size());
            for (auto elt : lhs) {
                if (rhs.count(elt) == 0) {
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
