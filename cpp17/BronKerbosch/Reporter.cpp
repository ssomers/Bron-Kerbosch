#include "pch.h"

#include "Reporter.h"

void BronKerbosch::SimpleReporter::record(VertexList&& clique) {
    assert(clique.size() > 1);
    cliques.push_back(clique);
}
