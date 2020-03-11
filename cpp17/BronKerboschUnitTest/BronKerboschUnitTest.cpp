#include "pch.h"

#include "BronKerbosch/Portfolio.h"
#include "BronKerbosch/Reporter.h"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace BronKerbosch {
    TEST_CLASS(BronKerboschUnitTest) {
public:
    template <typename VertexSet>
    static void bk_core(std::vector<std::vector<Vertex>> const& adjacencies_in,
                        std::vector<std::vector<Vertex>> const& expected_cliques) {
        auto adjacencies = std::vector<VertexSet>{};
        adjacencies.resize(adjacencies_in.size());
        std::transform(adjacencies_in.begin(), adjacencies_in.end(), adjacencies.begin(),
                       [](std::vector<Vertex> const& v) { return VertexSet(v.begin(), v.end()); });
        auto graph = UndirectedGraph<VertexSet>{ std::move(adjacencies) };
        for (int func_index = 0; func_index < Portfolio::NUM_FUNCS; ++func_index) {
            auto reporter = SimpleReporter{};
            Portfolio::explore(func_index, graph, reporter);
            Assert::AreEqual(expected_cliques.size(), reporter.cliques.size());
            Portfolio::sort_cliques(reporter.cliques);
            assert_same_cliques(expected_cliques, reporter.cliques);
        }
    }

    static void bk(std::vector<std::vector<Vertex>>&& adjacencies_in,
                   std::vector<std::vector<Vertex>>&& expected_cliques) {
        bk_core<std::set<Vertex>>(adjacencies_in, expected_cliques);
        bk_core<ordered_vector<Vertex>>(adjacencies_in, expected_cliques);
        bk_core<std::unordered_set<Vertex>>(adjacencies_in, expected_cliques);
    }

    static void assert_same_cliques(std::vector<std::vector<Vertex>> const& lhs,
                                    std::vector<std::vector<Vertex>> const& rhs) {
        Assert::AreEqual(lhs.size(), rhs.size());
        for (size_t i = 0; i < lhs.size(); ++i) {
            Assert::AreEqual(lhs[i].size(), rhs[i].size());
            for (size_t j = 0; j < lhs[i].size(); ++j) {
                Assert::AreEqual(lhs[i][j], rhs[i][j]);
            }
        }
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
