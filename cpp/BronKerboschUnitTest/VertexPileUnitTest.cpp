#include "pch.h"

#include "BronKerbosch/VertexPile.h"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace BronKerbosch {
    TEST_CLASS(VertexPileUnitTest) {
      public:
        TEST_METHOD(collect) {
            auto p1 = VertexPile{Vertex(4u)};
            {
                auto p2 = VertexPile{Vertex(2u), &p1};
                Assert::IsTrue(p2.collect() == std::vector<Vertex>{Vertex(4u), Vertex(2u)});
            }
            Assert::IsTrue(p1.collect() == std::vector<Vertex>{Vertex(4u)});
        }
    };
}
