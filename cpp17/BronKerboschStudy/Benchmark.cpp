﻿#include "pch.h"

#include "BronKerbosch/BronKerbosch1.h"
#include "BronKerbosch/Portfolio.h"
#include "BronKerbosch/SimpleReporter.h"
#include "BronKerbosch/UndirectedGraph.h"
#include "Console.h"
#include "RandomGraph.h"
#include "SampleStatistics.h"

using BronKerbosch::ordered_vector;
using BronKerbosch::Portfolio;
using BronKerbosch::SimpleReporter;
using BronKerbosch::UndirectedGraph;
using BronKerbosch::Vertex;
using BronKerbosch::VertexList;
using BronKerboschStudy::RandomGraph;
using BronKerboschStudy::SampleStatistics;

enum class SetType {
    hashset, std_set, ord_vec
};
static int const SET_TYPES = 3;

class Benchmark {
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
                    std::cout << "  " << std::setw(8) << Portfolio::FUNC_NAMES[func_index] << ": " << std::setw(6) << secs << "s" << std::endl;
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
    static Times bk_core(std::string const& orderstr, unsigned size,
                         std::vector<int> func_indices, int samples) {
        auto g = RandomGraph::readUndirected<VertexSet>(orderstr, size);
        return timed(g, func_indices, samples);
    };

    static Times bk_core(SetType set_type, std::string const& orderstr, unsigned size,
                         std::function<std::vector<int>(SetType, unsigned size)> includedFuncs, int samples) {
        auto func_indices = includedFuncs(set_type, size);
        if (func_indices.empty()) {
            return Times{};
        } else {
            switch (set_type) {
                case SetType::std_set: return bk_core<std::set<Vertex>>(orderstr, size, func_indices, samples);
                case SetType::ord_vec: return bk_core<ordered_vector<Vertex>>(orderstr, size, func_indices, samples);
                case SetType::hashset: return bk_core<std::unordered_set<Vertex>>(orderstr, size, func_indices, samples);
            }
            throw std::logic_error("unreachable");
        }
    }

    static const char* set_type_name(SetType set_type) {
        switch (set_type) {
            case SetType::std_set: return "std_set";
            case SetType::ord_vec: return "ord_vec";
            case SetType::hashset: return "hashset";
        }
        throw std::logic_error("unreachable");
    }

public:
    static void bk(std::string const& orderstr, std::vector<unsigned> const& sizes,
                   std::function<std::vector<int>(SetType, unsigned size)> includedFuncs, int samples) {
        auto tmpfname = "tmp.csv";
        {
            auto fo = std::ofstream{ tmpfname };
            fo << "Size";
            for (int set_type_index = 0; set_type_index < SET_TYPES; ++set_type_index) {
                for (auto func_name : Portfolio::FUNC_NAMES) {
                    auto name = std::string(func_name) + "@" + set_type_name(SetType(set_type_index));
                    fo << "," << name << "  min," << name << " mean," << name << " max";
                }
            }
            fo << "\n";
            for (int size : sizes) {
                fo << size;
                for (int set_type_index = 0; set_type_index < SET_TYPES; ++set_type_index) {
                    auto set_type = SetType(set_type_index);
                    Times stats = bk_core(set_type, orderstr, size, includedFuncs, samples);
                    for (int func_index = 0; func_index < Portfolio::NUM_FUNCS; ++func_index) {
                        auto func_name = Portfolio::FUNC_NAMES[func_index];
                        auto max = stats[func_index].max();
                        auto min = stats[func_index].min();
                        auto mean = stats[func_index].mean();
                        fo << "," << min;
                        fo << "," << mean;
                        fo << "," << max;
                        if (!std::isnan(mean)) {
                            auto reldev = int(100 * stats[func_index].deviation() / mean + .5);
                            auto p = std::cout.precision(3);
                            auto f = std::cout.setf(std::ios_base::fixed);
                            std::cout
                                << "order " << std::setw(4) << orderstr
                                << " size " << std::setw(7) << size
                                << " " << std::setw(8) << func_name
                                << "@" << set_type_name(set_type)
                                << ": " << std::setw(6) << mean << "s ± " << reldev << "%"
                                << std::endl;
                            std::cout.precision(p);
                            std::cout.setf(f);
                        }
                    }
                }
                fo << std::endl;
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

    std::vector<int> all_func_indices(Portfolio::NUM_FUNCS);
    std::vector<int> most_func_indices(Portfolio::NUM_FUNCS - 1);
    std::iota(all_func_indices.begin(), all_func_indices.end(), 0);
    std::iota(most_func_indices.begin(), most_func_indices.end(), 1);
    if (argc == 1) {
        Benchmark::bk("100", range(2'000u, 3'000u, 50u), // max 4'950
                      [&](SetType set_type, unsigned size) {
                          switch (set_type) {
                              case SetType::std_set:
                              case SetType::hashset: if (size <= 2500) return all_func_indices;
                                                   else if (size <= 2600) return std::vector<int>{1, 2, 3};
                                                   else return std::vector<int>{};
                              case SetType::ord_vec: return all_func_indices;
                          }; throw std::logic_error("unreachable"); }, 5);
        Benchmark::bk("10k", concat(range(10'000u, 90'000u, 10'000u),
                                    range(100'000u, 200'000u, 25'000u)),
                      [&](SetType set_type, unsigned size) {
                          switch (set_type) {
                              case SetType::std_set: return std::vector<int>{};
                              case SetType::hashset: if (size > 400'000) return std::vector<int>{};
                              case SetType::ord_vec: return most_func_indices;
                          }; throw std::logic_error("unreachable"); }, 3);
        Benchmark::bk("1M", concat(range(2'000u, 20'000u, 2'000u),
                                   range(40'000u, 100'000u, 20'000u),
                                   range(200'000u, 1'000'000u, 200'000u)),
                      [&](SetType set_type, unsigned size) {
                          if (size < 15'000) {
                              return std::vector<int>{0};
                          } else switch (set_type) {
                              case SetType::std_set: return std::vector<int>{};
                              case SetType::ord_vec: if (size > 50'000) return std::vector<int>{};
                              case SetType::hashset: return all_func_indices;
                          }; throw std::logic_error("unreachable"); }, 3);
        return EXIT_SUCCESS;
    } else if (argc == 3) {
        auto orderstr = argv[1];
        unsigned size = BronKerboschStudy::RandomGraph::parseInt(argv[2]);
        Benchmark::bk(orderstr, range(size, size, 1u), [&](SetType, unsigned) { return all_func_indices; }, 1);
        return EXIT_SUCCESS;
    } else {
        std::cerr << "Specify order and size\n";
        return EXIT_FAILURE;
    }
}