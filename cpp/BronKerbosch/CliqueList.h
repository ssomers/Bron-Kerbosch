#pragma once

#include <list>
#include <vector>

#include "Vertex.h"

namespace BronKerbosch {
    using VertexList = std::vector<Vertex>;
    using CliqueList = std::list<VertexList>;
}
