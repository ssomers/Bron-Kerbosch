#include "pch.h"

#include "BronKerbosch/BronKerbosch1.h"
#include "BronKerbosch/Portfolio.h"
#include "BronKerbosch/Reporter.h"
#include "BronKerbosch/UndirectedGraph.h"
#include "RandomGraph.h"
#include "SampleStatistics.h"

#pragma execution_character_set( "utf-8" )
#define NOMINMAX
#include <windows.h>

using BronKerbosch::Portfolio;
using BronKerbosch::SimpleReporter;
using BronKerbosch::UndirectedGraph;
using BronKerbosch::Vertex;
using BronKerbosch::VertexList;
using BronKerboschStudy::RandomGraph;
using BronKerboschStudy::SampleStatistics;

class Benchmark {
    using Times = std::array<SampleStatistics<double>, Portfolio::NUM_FUNCS>;

    static void assert_same_cliques(std::vector<VertexList> const& lhs, std::vector<VertexList> const& rhs) {
        assert(lhs.size() == rhs.size());
        for (size_t i = 0; i < lhs.size(); ++i) {
            assert(lhs[i].size() == rhs[i].size());
            for (size_t j = 0; j < lhs[i].size(); ++j) {
                assert(lhs[i][j] == rhs[i][j]);
            }
        }
    }

    template <typename VertexSet>
    static Times timed(UndirectedGraph<VertexSet> const& graph, std::vector<int> const& func_indices, int samples) {
        std::unique_ptr<std::vector<VertexList>> first;
        auto times = Times{};
        for (int sample = 0; sample < samples; ++sample) {
            for (int func_index : func_indices) {
                auto reporter = SimpleReporter{};
                auto begin = std::chrono::steady_clock::now();
                Portfolio::explore(func_index, graph, reporter);
                auto duration = std::chrono::steady_clock::now() - begin;
                auto secs = std::chrono::duration<double, std::ratio<1, 1>>(duration).count();
                if (duration >= std::chrono::seconds(3)) {
                    auto p = std::cout.precision(2);
                    auto f = std::cout.setf(std::ios_base::fixed);
                    std::cout << "  " << std::setw(8) << Portfolio::FUNC_NAMES[func_index] << ": " << std::setw(5) << secs << "s\n";
                    std::cout.precision(p);
                    std::cout.setf(f);
                }
                if (sample < 2) {
                    Portfolio::sort_cliques(reporter.cliques);
                    if (first)
                        assert_same_cliques(*first, reporter.cliques);
                    else
                        first = std::make_unique<std::vector<VertexList>>(reporter.cliques);
                }
                times[func_index].put(secs);
            }
        }
        return times;
    }

public:
    template <typename VertexSet>
    static void bk(std::string const& orderstr, std::vector<int> const& sizes, std::function<std::vector<int>(int size)> includedFuncs, int samples) {
        auto tmpfname = "tmp.csv";
        {
            auto fo = std::ofstream{ tmpfname };
            fo << "Size";
            for (auto name : Portfolio::FUNC_NAMES) {
                fo << "," << name << "  min," << name << " mean," << name << " max";
            }
            fo << "\n";
            for (int size : sizes) {
                auto func_indices = includedFuncs(size);
                auto g = RandomGraph::readUndirected<VertexSet>(orderstr, size);
                auto stats = timed(g, func_indices, samples);
                fo << size;
                for (int func_index = 0; func_index < Portfolio::NUM_FUNCS; ++func_index) {
                    auto func_name = Portfolio::FUNC_NAMES[func_index];
                    auto max = stats[func_index].max();
                    auto min = stats[func_index].min();
                    auto mean = stats[func_index].mean();
                    auto dev = stats[func_index].deviation();
                    fo << "," << min;
                    fo << "," << mean;
                    fo << "," << max;
                    auto p = std::cout.precision(2);
                    auto f = std::cout.setf(std::ios_base::fixed);
                    std::cout
                        << "order " << std::setw(4) << orderstr
                        << " size " << std::setw(7) << size
                        << " " << std::setw(8) << func_name
                        << ": " << std::setw(5) << mean << "s ±" << std::setw(5) << dev << "s\n";
                    std::cout.precision(p);
                    std::cout.setf(f);
                }
                fo << "\n";
            }
        }
        auto path = "..\\..\\bron_kerbosch_c++_order_" + orderstr + ".csv";
        std::remove(path.c_str());
        auto rc = std::rename(tmpfname, path.c_str());
        if (rc != 0) {
            std::cerr << "Failed to rename " << tmpfname << " to " << path << "(" << rc << ")\n";
        }
    }
};

static std::vector<int> range(int first, int last, int step) {
    std::vector<int> result;
    for (int i = first; i <= last; i += step) {
        result.push_back(i);
    }
    return result;
}

template<typename... Args> void concat1(std::vector<int>& builder) {
}

template<typename... Args> void concat1(std::vector<int>& builder, std::vector<int> const& arg, Args... args) {
    std::copy(arg.begin(), arg.end(), std::back_inserter(builder));
    concat1(builder, args...);
}

template<typename... Args> std::vector<int> concat(Args... args) {
    std::vector<int> result;
    concat1(result, args...);
    return result;
}

int main(int argc, char** argv) {
    SetConsoleOutputCP(65001);
#ifndef NDEBUG
    std::cerr << "Run Release build for meaningful measurements\n";
    //return EXIT_FAILURE;
#endif
    auto all_func_indices = [](int size) {
        std::vector<int> all_func_indices(Portfolio::NUM_FUNCS);
        std::iota(all_func_indices.begin(), all_func_indices.end(), 0);
        return all_func_indices; };
    Benchmark::bk<std::set<Vertex>>("100", range(2'000, 3'001, 50), all_func_indices, 5); // max 4'950
    Benchmark::bk<std::set<Vertex>>("10k", range(100'000, 800'001, 100'000), all_func_indices, 3);
    Benchmark::bk<std::set<Vertex>>("1M", concat(range(2'000, 20'001, 2'000), range(200'000, 1'000'000, 200'000), range(1'000'000, 3'000'001, 1'000'000)),
                                    [](int size) { return size <= 20'000 ? std::vector<int> { 0, 1 } : std::vector<int>{ 1, 2, 3, 4, 5 }; }, 3);
    return EXIT_SUCCESS;
}
