#include "pch.h"
#include "RandomGraph.h"

unsigned BronKerboschStudy::parseInt(std::string const& str) {
    unsigned factor = 1;
    if (*str.rbegin() == 'M')
        factor = 1'000'000;
    if (*str.rbegin() == 'k')
        factor = 1'000;
    auto i = std::stoi(str);
    if (i < 0) {
        std::cerr << str << " is negative\n";
        std::exit(EXIT_FAILURE);
    }
    return unsigned(i) * factor;
}
