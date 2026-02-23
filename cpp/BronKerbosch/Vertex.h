#pragma once

namespace BronKerbosch {
    class Vertex {
        unsigned itsIndex;

      public:
        static Vertex sentinel() {
            return Vertex(std::numeric_limits<unsigned>::max());
        }

        Vertex() = default;
        Vertex(Vertex const&) = default;
        Vertex(Vertex&&) = default;
        Vertex& operator=(Vertex const&) = default;
        Vertex& operator=(Vertex&&) = default;
        explicit Vertex(unsigned index) : itsIndex(index) {
        }

        bool operator==(Vertex rhs) const {
            return itsIndex == rhs.itsIndex;
        }

        bool operator<(Vertex rhs) const {
            return itsIndex < rhs.itsIndex;
        }

        unsigned index() const {
            return itsIndex;
        }

        Vertex& operator++() {
            ++itsIndex;
            return *this;
        }
    };
}

template <>
struct std::hash<BronKerbosch::Vertex> {
    size_t operator()(BronKerbosch::Vertex const& v) const {
        return std::hash<unsigned>()(v.index());
    }
};
