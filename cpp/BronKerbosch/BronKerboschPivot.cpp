#include "pch.h"

#include "BronKerboschPivot.h"

namespace BronKerbosch {
    std::set<Vertex> intersection(std::set<Vertex> const& lhs, std::vector<bool> const& rhs) {
        std::set<Vertex> result;
        std::remove_copy_if(lhs.begin(), lhs.end(), std::inserter(result, result.end()),
                            [&rhs](Vertex const& v) { return !rhs[v.index()]; });
        return result;
    }

    ordered_vector<Vertex> intersection(ordered_vector<Vertex> const& lhs,
                                        std::vector<bool> const& rhs) {
        ordered_vector<Vertex> result;
        result.reserve(lhs.size());
        std::remove_copy_if(lhs.begin(), lhs.end(), std::inserter(result, result.end()),
                            [&rhs](Vertex const& v) { return !rhs[v.index()]; });
        return result;
    }

    std::unordered_set<Vertex> intersection(std::unordered_set<Vertex> const& lhs,
                                            std::vector<bool> const& rhs) {
        std::unordered_set<Vertex> result;
        result.reserve(lhs.size());
        for (auto v : lhs) {
            if (rhs[v.index()]) {
                result.insert(v);
            }
        }
        return result;
    }
}
