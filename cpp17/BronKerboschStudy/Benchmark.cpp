#include "pch.h"

#include "BronKerbosch/BronKerbosch1.h"
#include "BronKerbosch/Portfolio.h"
#include "BronKerbosch/SimpleReporter.h"
#include "BronKerbosch/UndirectedGraph.h"
#include "Console.h"
#include "RandomGraph.h"
#include "SampleStatistics.h"

using BronKerbosch::Portfolio;
using BronKerbosch::SimpleReporter;
using BronKerbosch::UndirectedGraph;
using BronKerbosch::Vertex;
using BronKerbosch::VertexList;
using BronKerboschStudy::RandomGraph;
using BronKerboschStudy::SampleStatistics;

class Benchmark {
    static int const SET_TYPES = 2;
    static const char* const SET_TYPE_NAMES[SET_TYPES];

    using Times = std::array<SampleStatistics<double>, Portfolio::NUM_FUNCS>;

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
                    std::cout << "  " << std::setw(8) << Portfolio::FUNC_NAMES[func_index] << ": " << std::setw(5) << secs << "s" << std::endl;
                    std::cout.precision(p);
                    std::cout.setf(f);
                }
                if (sample < 2) {
                    Portfolio::sort_cliques(reporter.cliques);
                    if (first) {
                        if (*first != reporter.cliques) {
                            throw std::logic_error("got different cliques");
                        }
                    } else {
                        first = std::make_unique<std::vector<VertexList>>(reporter.cliques);
                    }
                }
                times[func_index].put(secs);
            }
        }
        return times;
    }

    template <typename VertexSet>
    static Times bk_core(std::string const& orderstr, unsigned size, std::function<std::vector<int>(unsigned size)> includedFuncs, int samples) {
        auto func_indices = includedFuncs(size);
        auto g = RandomGraph::readUndirected<VertexSet>(orderstr, size);
        return timed(g, func_indices, samples);
    };

public:
    static void bk(std::string const& orderstr, std::vector<unsigned> const& sizes, std::function<std::vector<int>(unsigned size)> includedFuncs, int samples) {
        auto tmpfname = "tmp.csv";
        {
            auto fo = std::ofstream{ tmpfname };
            fo << "Size";
            for (int set_type_index = 0; set_type_index < SET_TYPES; ++set_type_index) {
                for (auto func_name : Portfolio::FUNC_NAMES) {
                    auto name = std::string(func_name) + "@" + SET_TYPE_NAMES[set_type_index];
                    fo << "," << name << "  min," << name << " mean," << name << " max";
                }
            }
            fo << "\n";
            for (int size : sizes) {
                fo << size;
                for (int set_type_index = 0; set_type_index < SET_TYPES; ++set_type_index) {
                    Times stats;
                    switch (SET_TYPE_NAMES[set_type_index][0]) {
                        case 's': stats = bk_core<std::set<Vertex>>(orderstr, size, includedFuncs, samples); break;
                        case 'u': stats = bk_core<std::unordered_set<Vertex>>(orderstr, size, includedFuncs, samples); break;
                    }
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
                        if (!std::isnan(mean)) {
                            std::cout
                                << "order " << std::setw(4) << orderstr
                                << " size " << std::setw(7) << size
                                << " " << std::setw(8) << func_name
                                << "@" << std::setw(10) << SET_TYPE_NAMES[set_type_index]
                                << ": " << std::setw(5) << mean << "s ±" << std::setw(5) << dev << "s"
                                << std::endl;
                            std::cout.precision(p);
                            std::cout.setf(f);
                        }
                    }
                }
                fo << "\n";
            }
        }
        auto path = "..\\bron_kerbosch_c++_order_" + orderstr + ".csv";
        std::remove(path.c_str());
        auto rc = std::rename(tmpfname, path.c_str());
        if (rc != 0) {
            std::cerr << "Failed to rename " << tmpfname << " to " << path << "(" << rc << ")\n";
        }
    }
};

const char* const Benchmark::SET_TYPE_NAMES[SET_TYPES] = {
    "std::set",
    "unordered"
};

template <typename T>
static std::vector<T> range(T first, T last, T step) {
    std::vector<T> result;
    for (T i = first; i <= last; i += step) {
        result.push_back(i);
    }
    return result;
}

template<typename... Args> void concat1(std::vector<unsigned>&) {
}

template<typename... Args> void concat1(std::vector<unsigned>& builder, std::vector<unsigned> const& arg, Args... args) {
    std::copy(arg.begin(), arg.end(), std::back_inserter(builder));
    concat1(builder, args...);
}

template<typename... Args> std::vector<unsigned> concat(Args... args) {
    std::vector<unsigned> result;
    concat1(result, args...);
    return result;
}

int main(int argc, char** argv) {
    console_init();
#ifndef NDEBUG
    std::cerr << "Run Release build for meaningful measurements\n";
    //return EXIT_FAILURE;
#endif

    auto all_func_indices = [](unsigned) {
        std::vector<int> all_func_indices(Portfolio::NUM_FUNCS);
        std::iota(all_func_indices.begin(), all_func_indices.end(), 0);
        return all_func_indices; };
    if (argc == 1) {
        Benchmark::bk("100", range(2'000u, 3'000u, 50u), all_func_indices, 5); // max 4'950
        Benchmark::bk("10k", range(100'000u, 400'000u, 100'000u), all_func_indices, 3);
        /*
        Benchmark::bk("10k", range(100'000u, 800'000u, 100'000u), all_func_indices, 3);
        Benchmark::bk("1M", concat(range(2'000u, 20'000u, 2'000u),
                                   range(200'000u, 1'000'000u, 200'000u),
                                   range(1'000'000u, 3'000'000u, 1'000'000u)),
                      [](unsigned size) { return size <= 20'000 ? std::vector<int> { 0, 1 } : std::vector<int>{ 1, 2, 3, 4, 5 }; }, 3);
        */
        return EXIT_SUCCESS;
    } else if (argc == 3) {
        auto orderstr = argv[1];
        unsigned size = BronKerboschStudy::RandomGraph::parseInt(argv[2]);
        Benchmark::bk(orderstr, range(size, size, 1u), all_func_indices, 1);
        return EXIT_SUCCESS;
    } else {
        std::cerr << "Specify order and size\n";
        return EXIT_FAILURE;
    }
}
