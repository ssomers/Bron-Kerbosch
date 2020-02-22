#include "pch.h"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace BronKerbosch {
    TEST_CLASS(BronKerboschUnitTest) {
public:
    template <typename VertexSet>
    static void bk_core(std::vector<VertexList> const& adjacencies_in, std::vector<VertexList> const& expected_cliques) {
        auto adjacencies = std::vector<VertexSet>{};
        adjacencies.resize(adjacencies_in.size());
        std::transform(adjacencies_in.begin(), adjacencies_in.end(), adjacencies.begin(), [](VertexList const& v) { return VertexSet(v.begin(), v.end()); });
        auto graph = UndirectedGraph<VertexSet>{ std::move(adjacencies) };
        for (int func_index = 0; func_index < Portfolio::NUM_FUNCS; ++func_index) {
            auto reporter = SimpleReporter();
            Portfolio::explore(func_index, graph, reporter);
            Assert::AreEqual(expected_cliques.size(), reporter.cliques.size());
            Portfolio::sort_cliques(reporter.cliques);
            assert_same_cliques(expected_cliques, reporter.cliques);
        }
    }

    static void bk(std::vector<VertexList>&& adjacencies_in, std::vector<VertexList>&& expected_cliques) {
        bk_core<std::set<Vertex>>(adjacencies_in, expected_cliques);
        bk_core<std::unordered_set<Vertex>>(adjacencies_in, expected_cliques);
    }

    static void assert_same_cliques(std::vector<VertexList> const& lhs, std::vector<VertexList> const& rhs) {
        Assert::AreEqual(lhs.size(), rhs.size());
        for (auto i = 0; i < lhs.size(); ++i) {
            Assert::AreEqual(lhs[i].size(), rhs[i].size());
            for (auto j = 0; j < lhs[i].size(); ++j) {
                Assert::AreEqual(lhs[i][j], rhs[i][j]);
            }
        }
    }

    TEST_METHOD(Util_append) {
        auto const empty = VertexList{};
        auto const one = Util::append(empty, 11u);
        Assert::AreEqual(std::size_t{ 1 }, one.size());
        Assert::AreEqual(11u, one[0]);
        auto const two = Util::append(one, 22u);
        Assert::AreEqual(std::size_t{ 2 }, two.size());
        Assert::AreEqual(11u, two[0]);
        Assert::AreEqual(22u, two[1]);
    }

    template <typename VertexSet>
    void Util_difference() {
        auto const empty = VertexSet{};
        auto const one = VertexSet{ 1 };
        auto const two = VertexSet{ 1, 2 };
        auto const six = VertexSet{ 0, 1, 2, 3, 4, 5 };
        Assert::IsTrue(Util::difference(empty, one) == empty);
        Assert::IsTrue(Util::difference(empty, two) == empty);
        Assert::IsTrue(Util::difference(empty, six) == empty);
        Assert::IsTrue(Util::difference(one, one) == empty);
        Assert::IsTrue(Util::difference(one, two) == empty);
        Assert::IsTrue(Util::difference(one, six) == empty);
        Assert::IsTrue(Util::difference(two, two) == empty);
        Assert::IsTrue(Util::difference(two, six) == empty);
        Assert::IsTrue(Util::difference(six, six) == empty);
        Assert::IsTrue(Util::difference(one, empty) == one);
        Assert::IsTrue(Util::difference(two, empty) == two);
        Assert::IsTrue(Util::difference(six, empty) == six);
        Assert::IsTrue(Util::difference(two, one) == VertexSet{ 2 });
        Assert::IsTrue(Util::difference(six, one) == VertexSet{ 0, 2, 3, 4, 5 });
        Assert::IsTrue(Util::difference(six, two) == VertexSet{ 0, 3, 4, 5 });
    }

    template <typename VertexSet>
    void Util_intersection() {
        auto const empty = VertexSet{};
        auto const one = VertexSet{ 1 };
        auto const two = VertexSet{ 1, 2 };
        auto const six = VertexSet{ 0, 1, 2, 3, 4, 5 };
        Assert::IsTrue(Util::intersection(empty, one) == empty);
        Assert::IsTrue(Util::intersection(one, empty) == empty);
        Assert::IsTrue(Util::intersection(empty, two) == empty);
        Assert::IsTrue(Util::intersection(two, empty) == empty);
        Assert::IsTrue(Util::intersection(empty, six) == empty);
        Assert::IsTrue(Util::intersection(six, empty) == empty);
        Assert::IsTrue(Util::intersection(one, two) == one);
        Assert::IsTrue(Util::intersection(two, one) == one);
        Assert::IsTrue(Util::intersection(one, six) == one);
        Assert::IsTrue(Util::intersection(six, one) == one);
        Assert::IsTrue(Util::intersection(two, six) == two);
        Assert::IsTrue(Util::intersection(six, two) == two);
        Assert::IsTrue(Util::intersection(one, one) == one);
        Assert::IsTrue(Util::intersection(two, two) == two);
        Assert::IsTrue(Util::intersection(six, six) == six);

        Assert::IsTrue(Util::intersectSize(empty, one) == 0);
        Assert::IsTrue(Util::intersectSize(one, empty) == 0);
        Assert::IsTrue(Util::intersectSize(empty, two) == 0);
        Assert::IsTrue(Util::intersectSize(two, empty) == 0);
        Assert::IsTrue(Util::intersectSize(empty, six) == 0);
        Assert::IsTrue(Util::intersectSize(six, empty) == 0);
        Assert::IsTrue(Util::intersectSize(one, two) == 1);
        Assert::IsTrue(Util::intersectSize(two, one) == 1);
        Assert::IsTrue(Util::intersectSize(one, six) == 1);
        Assert::IsTrue(Util::intersectSize(six, one) == 1);
        Assert::IsTrue(Util::intersectSize(two, six) == 2);
        Assert::IsTrue(Util::intersectSize(six, two) == 2);
        Assert::IsTrue(Util::intersectSize(one, one) == 1);
        Assert::IsTrue(Util::intersectSize(two, two) == 2);
        Assert::IsTrue(Util::intersectSize(six, six) == 6);
    }

    TEST_METHOD(Util_difference_set) {
        Util_difference<std::set<Vertex>>();
    }

    TEST_METHOD(Util_difference_unordered_set_1) {
        Util_difference<std::unordered_set<Vertex>>();
    }

    TEST_METHOD(Util_intersection_set) {
        Util_intersection<std::set<Vertex>>();
    }

    TEST_METHOD(Util_intersection_unordered_set_1) {
        Util_intersection<std::unordered_set<Vertex>>();
    }

    TEST_METHOD(TestOrder0) {
        bk({}, {});
    }

    TEST_METHOD(TestOrder1) {
        bk({ {} }, {});
    }

    TEST_METHOD(TestOrder2_Isolated) {
        bk({ { }, { } }, { });
    }

    TEST_METHOD(TestOrder2_Connected) {
        bk({ { 1 }, { 0 } }, { { 0, 1 } });
    }

    TEST_METHOD(TestOrder3_Size1_Left) {
        bk({ { 1 }, { 0 }, { } }, { { 0, 1 } });
    }

    TEST_METHOD(TestOrder3_Size1_Long) {
        bk({ { 2 }, { }, { 0 } }, { { 0, 2 } });
    }

    TEST_METHOD(TestOrder3_Size1_Right) {
        bk({ { }, { 2 }, { 1 } }, { { 1, 2 } });
    }

    TEST_METHOD(TestOrder3_Size2) {
        bk({ { 1 }, { 0, 2 }, { 1 } }, { { 0, 1 }, { 1, 2 } });
    }

    TEST_METHOD(TestOrder3_Size3) {
        bk({ { 1, 2 }, { 0, 2 }, { 0, 1 } }, { { 0, 1, 2 } });
    }

    TEST_METHOD(TestOrder4_Size2) {
        bk({ { 1 }, { 0 }, { 3 }, { 2 } }, { { 0, 1 }, { 2, 3 } });
    }

    TEST_METHOD(TestOrder4_Size3_Bus) {
        bk({ { 1 }, { 0, 2 }, { 1, 3 }, { 2 } }, { { 0, 1 }, { 1, 2 }, { 2, 3 } });
    }

    TEST_METHOD(TestOrder4_Size3_Star) {
        bk({ { 1, 2, 3 }, { 0 }, { 0 }, { 0 } }, { { 0, 1 }, { 0, 2 }, { 0, 3 } });
    }

    TEST_METHOD(TestOrder4_Size4_p) {
        bk({ { 1 }, { 0, 2, 3 }, { 1, 3 }, { 1, 2 } }, { { 0, 1 }, { 1, 2, 3 } });
    }

    TEST_METHOD(TestOrder4_Size4_Square) {
        bk({ { 1, 3 }, { 0, 2 }, { 1, 3 }, { 0, 2 } },
           { { 0, 1 }, { 0, 3 }, { 1, 2 }, { 2, 3 } });
    }

    TEST_METHOD(TestOrder4_Size5) {
        bk({ { 1, 2, 3 }, { 0, 2 }, { 0, 1, 3 }, { 0, 2 } },
           { { 0, 1, 2 }, { 0, 2, 3 } });
    }

    TEST_METHOD(TestOrder4_Size6) {
        bk({ { 1, 2, 3 }, { 0, 2, 3 }, { 0, 1, 3 }, { 0, 1, 2 } },
           { { 0, 1, 2, 3 } });
    }

    TEST_METHOD(TestOrder4_Size6_Penultimate) {
        bk({ { 1, 2, 3, 4 }, { 0, 2, 3, 4 }, { 0, 1, 3, 4 }, { 0, 1, 2 }, { 0, 1, 2 } },
           { { 0, 1, 2, 3 }, { 0, 1, 2, 4 } });
    }

    TEST_METHOD(TestSample) {
        bk({
            { },
            { 2, 3, 4 },
            { 1, 3, 4, 5 },
            { 1, 2, 4, 5 },
            { 1, 2, 3 },
            { 2, 3, 6, 7 },
            { 5, 7 },
            { 5, 6 } },
           {
            { 1, 2, 3, 4 },
            { 2, 3, 5 },
            { 5, 6, 7 } });
    }

    TEST_METHOD(TestBigger) {
        bk({
            { 1, 2, 3, 4, 6, 7},
            { 0, 3, 6, 7, 8, 9 },
            { 0, 3, 5, 7, 8, 9 },
            { 0, 1, 2, 4, 9 },
            { 0, 3, 6, 7, 9 },
            { 2, 6 },
            { 0, 1, 4, 5, 9 },
            { 0, 1, 2, 4, 9 },
            { 1, 2 },
            { 1, 2, 3, 4, 6, 7 } },
           {
            { 0, 1, 3 },
            { 0, 1, 6 },
            { 0, 1, 7 },
            { 0, 2, 3 },
            { 0, 2, 7 },
            { 0, 3, 4 },
            { 0, 4, 6 },
            { 0, 4, 7 },
            { 1, 3, 9 },
            { 1, 6, 9 },
            { 1, 7, 9 },
            { 1, 8 },
            { 2, 3, 9 },
            { 2, 5 },
            { 2, 7, 9 },
            { 2, 8 },
            { 3, 4, 9 },
            { 4, 6, 9 },
            { 4, 7, 9 },
            { 5, 6 } });
    }
    };
}
