#include "pch.h"

#include "BronKerbosch/GraphDegeneracy.h"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace BronKerbosch {
    TEST_CLASS(GraphDegeneracyTest) {
      public:
        template <typename VertexSet>
        void test_empty() {
            UndirectedGraph<VertexSet> const g{typename UndirectedGraph<VertexSet>::Adjacencies{}};
            Assert::IsFalse(DegeneracyIter{g}.has_next());
        }

        template <typename VertexSet>
        void test_pair() {
            UndirectedGraph<VertexSet> const g{
                typename UndirectedGraph<VertexSet>::Adjacencies{{Vertex(1u)}, {Vertex(0u)}}};
            auto it = DegeneracyIter{g};
            auto first = it.next();
            Assert::IsTrue(first.has_value());
            Assert::IsFalse(it.next().has_value());
        }

        template <typename VertexSet>
        void test_split() {
            UndirectedGraph<VertexSet> const g{typename UndirectedGraph<VertexSet>::Adjacencies{
                {Vertex(1u)}, {Vertex(0u), Vertex(2u)}, {Vertex(1u)}}};
            auto it = DegeneracyIter{g};
            auto first = it.next();
            Assert::IsTrue(first.has_value());
            Assert::AreNotEqual(1u, first->index());
            Assert::IsTrue(it.next().has_value());
            Assert::IsFalse(it.next().has_value());
        }

        TEST_METHOD(empty) {
            test_empty<std::set<Vertex>>();
            test_empty<std::unordered_set<Vertex>>();
        }

        TEST_METHOD(pair) {
            test_pair<std::set<Vertex>>();
            test_pair<std::unordered_set<Vertex>>();
        }

        TEST_METHOD(split) {
            test_split<std::set<Vertex>>();
            test_split<std::unordered_set<Vertex>>();
        }
    };
}
