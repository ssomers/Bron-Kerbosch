#include "pch.h"

#include "BronKerbosch/GraphDegeneracy.h"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace BronKerbosch {
    TEST_CLASS(GraphDegeneracyTest) {
       public:
        template <typename VertexSet>
        void test_empty() {
            UndirectedGraph<VertexSet> const g{typename UndirectedGraph<VertexSet>::Adjacencies{}};
            Assert::IsFalse(DegeneracyOrderIter<VertexSet>::degeneracy_ordering(g).has_next());
            Assert::IsFalse(DegeneracyOrderIter<VertexSet>::degeneracy_ordering(g, -1).has_next());
        }

        template <typename VertexSet>
        void test_pair() {
            UndirectedGraph<VertexSet> const g{
                typename UndirectedGraph<VertexSet>::Adjacencies{{1u}, {0u}}};
            auto it = DegeneracyOrderIter<VertexSet>::degeneracy_ordering(g);
            auto first = it.next();
            auto second = it.next();
            Assert::IsTrue(first.has_value());
            Assert::IsTrue(second.has_value());
            Assert::IsFalse(it.next().has_value());
            Assert::AreEqual(0u, std::min(*first, *second));
            Assert::AreEqual(1u, std::max(*first, *second));

            auto it1 = DegeneracyOrderIter<VertexSet>::degeneracy_ordering(g, -1);
            Assert::IsTrue(first == it1.next());
            Assert::IsFalse(it1.next().has_value());

            Assert::IsFalse(
                DegeneracyOrderIter<VertexSet>::degeneracy_ordering(g, -2).next().has_value());
        }

        template <typename VertexSet>
        void test_split() {
            UndirectedGraph<VertexSet> const g{
                typename UndirectedGraph<VertexSet>::Adjacencies{{1u}, {0u, 2u}, {1u}}};
            auto it = DegeneracyOrderIter<VertexSet>::degeneracy_ordering(g);
            Assert::AreNotEqual(1u, *it.next());
            Assert::IsTrue(it.next().has_value());
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
