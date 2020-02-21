#include "pch.h"
#include "Portfolio.h"

const char* const BronKerbosch::Portfolio::FUNC_NAMES[NUM_FUNCS] = { "Ver1+", "Ver2+", "Ver2+GP", "Ver2+GPX" };

void BronKerbosch::Portfolio::sort_cliques(std::vector<VertexList>& cliques) {
    for (VertexList& clique : cliques)
        std::sort(clique.begin(), clique.end());
    std::sort(cliques.begin(), cliques.end(), &clique_less);
}

bool BronKerbosch::Portfolio::clique_less(VertexList const& lhs, VertexList const& rhs) {
    for (size_t i = 0; i < lhs.size() && i < rhs.size(); ++i) {
        int d = int(lhs[i]) - int(rhs[i]);
        if (d != 0)
            return d < 0;
    }
    throw std::logic_error("got overlapping or equal cliques");
}
