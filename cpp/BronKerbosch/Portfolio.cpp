#include "pch.h"

#include "Portfolio.h"

#include <stdexcept>

const char* const BronKerbosch::Portfolio::FUNC_NAMES[NUM_FUNCS] = {
    "Ver1\xc2\xbd",    "Ver2\xc2\xbd-GP",  "Ver2\xc2\xbd-GPX",
    "Ver3\xc2\xbd-GP", "Ver3\xc2\xbd-GPX", "Ver3\xc2\xbd=GPc",
};

void BronKerbosch::Portfolio::sort_cliques(std::vector<std::vector<Vertex>>& cliques) {
    for (auto& clique : cliques)
        std::sort(clique.begin(), clique.end());
    std::sort(cliques.begin(), cliques.end(), &clique_less);
}

bool BronKerbosch::Portfolio::clique_less(std::vector<Vertex> const& lhs,
                                          std::vector<Vertex> const& rhs) {
    for (size_t i = 0; i < lhs.size() && i < rhs.size(); ++i) {
        if (lhs[i] < rhs[i])
            return true;
        if (rhs[i] < lhs[i])
            return false;
    }
    throw std::logic_error("got overlapping or equal cliques");
}
