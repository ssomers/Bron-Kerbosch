#include "pch.h"

#include "BronKerbosch/VertexPile.h"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace BronKerbosch {
    TEST_CLASS(VertexPileUnitTest) {
       public:
        TEST_METHOD(collect) {
            auto p1 = VertexPile{4};
            {
                auto p2 = VertexPile{2, &p1};
                Assert::IsTrue(p2.collect() == std::vector<Vertex>{4, 2});
            }
            Assert::IsTrue(p1.collect() == std::vector<Vertex>{4});
        }
    };
}
