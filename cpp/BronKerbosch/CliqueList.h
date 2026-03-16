#pragma once

#include <list>
#include <vector>

#include "Vertex.h"

namespace BronKerbosch {
    using Clique = std::vector<Vertex>;
    using CliqueList = std::list<Clique>;
}
