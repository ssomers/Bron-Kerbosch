//! Core of Bron-Kerbosch algorithms with pivot

#pragma once

#include "UndirectedGraph.h"
#include "VertexPile.h"
#include <cassert>

namespace BronKerbosch {
    enum class PivotChoice {
        Arbitrary,
        MaxDegree,
        MaxDegreeLocal,
        MaxDegreeLocalX,
    };

    class BronKerboschPivot {
    public:
        template <typename VertexSet, typename Reporter>
        static void visit(UndirectedGraph<VertexSet> const& graph, Reporter& reporter,
                          PivotChoice initial_pivot_choice, PivotChoice further_pivot_choice,
                          VertexSet&& candidates, VertexSet&& excluded, VertexPile* clique) {
            assert(!candidates.empty());
            assert(Util::are_disjoint(candidates, excluded));

            if (candidates.size() == 1) {
                // Same logic as below, but stripped down for this common case
                for (Vertex v : candidates) {
                    auto const& neighbours = graph.neighbours(v);
                    if (Util::are_disjoint(neighbours, excluded)) {
                        reporter.record(VertexPile(v, clique).collect());
                    }
                }
                return;
            }

            auto pivot = std::numeric_limits<Vertex>::max();
            std::vector<Vertex> remaining_candidates;
            remaining_candidates.reserve(candidates.size());
            switch (initial_pivot_choice) {
                case PivotChoice::MaxDegreeLocal:
                case PivotChoice::MaxDegreeLocalX: {
                    // Quickly handle locally unconnected candidates while finding pivot
                    size_t seen_local_degree = 0;
                    for (Vertex v : candidates) {
                        auto const& neighbours = graph.neighbours(v);
                        auto local_degree = Util::intersection_size(neighbours, candidates);
                        if (local_degree == 0) {
                            // Same logic as below, but stripped down
                            if (Util::are_disjoint(neighbours, excluded)) {
                                reporter.record(VertexPile(v, clique).collect());
                            }
                        } else {
                            if (seen_local_degree < local_degree) {
                                seen_local_degree = local_degree;
                                pivot = v;
                            }
                            remaining_candidates.push_back(v);
                        }
                    }
                    if (remaining_candidates.empty()) {
                        return;
                    }
                    if (initial_pivot_choice == PivotChoice::MaxDegreeLocalX) {
                        for (Vertex v : excluded) {
                            auto const& neighbours = graph.neighbours(v);
                            auto local_degree = Util::intersection_size(neighbours, candidates);
                            if (seen_local_degree < local_degree) {
                                seen_local_degree = local_degree;
                                pivot = v;
                            }
                        }
                    }
                    break;
                }
                case PivotChoice::Arbitrary:
                case PivotChoice::MaxDegree: {
                    std::copy(candidates.begin(), candidates.end(), std::back_inserter(remaining_candidates));
                    pivot = choose(initial_pivot_choice, remaining_candidates, graph);
                }
            }

            assert(!remaining_candidates.empty());
            for (Vertex v : remaining_candidates) {
                auto const& neighbours = graph.neighbours(v);
                if (neighbours.count(pivot)) {
                    continue;
                }
                candidates.erase(v);
                auto neighbouring_candidates = Util::intersection(neighbours, candidates);
                if (neighbouring_candidates.empty()) {
                    if (Util::are_disjoint(neighbours, excluded)) {
                        reporter.record(VertexPile(v, clique).collect());
                    }
                } else {
                    auto neighbouring_excluded = Util::intersection(neighbours, excluded);
                    auto newclique = VertexPile(v, clique);
                    visit(
                        graph,
                        reporter,
                        further_pivot_choice,
                        further_pivot_choice,
                        std::move(neighbouring_candidates),
                        std::move(neighbouring_excluded),
                        &newclique
                    );
                }
                excluded.insert(v);
            }
        }

    private:
        template <typename VertexSet>
        static Vertex choose(
            PivotChoice pivot_choice,
            std::vector<Vertex> const& candidates,
            UndirectedGraph<VertexSet> const& graph) {
            switch (pivot_choice) {
                case PivotChoice::Arbitrary: return *candidates.begin();
                case PivotChoice::MaxDegree: return *std::max_element(candidates.begin(),
                                                                      candidates.end(),
                                                                      [&graph](Vertex v, Vertex w) {
                                                                          return graph.degree(v) < graph.degree(w);
                                                                      });
                case PivotChoice::MaxDegreeLocal:
                case PivotChoice::MaxDegreeLocalX: break;
            }
            throw std::logic_error("unreachable");
        }
    };
}
