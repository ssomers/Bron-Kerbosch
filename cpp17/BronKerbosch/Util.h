#pragma once

#include <set>
#include <unordered_set>

namespace BronKerbosch {
    template <typename T>
    class ordered_vector {
        std::vector<T> vals;

    public:
        using size_type = typename std::vector<T>::size_type;
        using iterator = typename std::vector<T>::iterator;
        using const_iterator = typename std::vector<T>::const_iterator;
        using value_type = T;

        ordered_vector() = default;

        ordered_vector(std::initializer_list<T> &&list) : vals(list) {
            assert(std::is_sorted(begin(), end()));
        }

        template <typename I>
        ordered_vector(I begin, I end) : vals(begin, end) {
            std::sort(vals.begin(), vals.end());
        }

        size_type size() const {
            return vals.size();
        }

        bool empty() const {
            return vals.empty();
        }

        size_t count(T val) const {
            auto pos = std::lower_bound(vals.begin(), vals.end(), val);
            auto found = pos != vals.end() && *pos == val;
            return found ? 1 : 0;
        }

        bool operator==(ordered_vector const& rhs) const {
            return vals == rhs.vals;
        }

        iterator insert(iterator pos, T val) {
            return vals.insert(pos, val);
        }

        std::pair<iterator, bool> insert(T val) {
            auto pos = std::lower_bound(vals.begin(), vals.end(), val);
            auto found = pos != vals.end() && *pos == val;
            if (!found) {
                vals.insert(pos, val);
            }
            return std::make_pair(pos, !found);
        }

        bool erase(T val) {
            auto pos = std::lower_bound(vals.begin(), vals.end(), val);
            auto found = pos != vals.end() && *pos == val;
            if (found) {
                vals.erase(pos);
            }
            return found;
        }

        void erase(iterator pos) {
            assert(pos != vals.end());
            vals.erase(pos);
        }

        void reserve(size_t capacity) {
            vals.reserve(capacity);
        }

        const_iterator begin() const {
            return vals.begin();
        }

        const_iterator end() const {
            return vals.end();
        }

        iterator begin() {
            return vals.begin();
        }

        iterator end() {
            return vals.end();
        }
    };

    namespace Util {
        template <typename T>
        static T pop_arbitrary(std::set<T>& set) {
            assert(!set.empty());
            auto it = set.begin();
            auto result = *it;
            set.erase(it);
            return result;
        }
        template <typename T>
        static T pop_arbitrary(ordered_vector<T>& set) {
            assert(!set.empty());
            auto it = set.end() - 1;
            auto result = *it;
            set.erase(it);
            return result;
        }
        template <typename T>
        static T pop_arbitrary(std::unordered_set<T>& set) {
            assert(!set.empty());
            auto it = set.begin();
            auto result = *it;
            set.erase(it);
            return result;
        }

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
        static bool are_disjoint(ordered_vector<T> const& lhs, ordered_vector<T> const& rhs) {
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
        static size_t intersection_size(ordered_vector<T> const& lhs, ordered_vector<T> const& rhs) {
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
        static ordered_vector<T> intersection(ordered_vector<T> const& lhs, ordered_vector<T> const& rhs) {
            ordered_vector<T> result;
            result.reserve(std::min(lhs.size(), rhs.size()));
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
        static ordered_vector<T> difference(ordered_vector<T> const& lhs, ordered_vector<T> const& rhs) {
            ordered_vector<T> result;
            result.reserve(lhs.size());
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

        template <typename T>
        static void reserve(std::set<T>&, size_t) {
        }
        template <typename T>
        static void reserve(ordered_vector<T>& set, size_t capacity) {
            set.reserve(capacity);
        }
        template <typename T>
        static void reserve(std::unordered_set<T>& set, size_t capacity) {
            set.reserve(capacity);
        }

        template <typename Set>
        static Set with_capacity(size_t capacity) {
            Set result;
            reserve(result, capacity);
            return result;
        }
    }
}
