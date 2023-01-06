//! Core of Bron-Kerbosch algorithms with pivot

#pragma once

#include <cassert>
#include <stdexcept>
#include "CliqueList.h"
#include "UndirectedGraph.h"
#include "VertexPile.h"

namespace BronKerbosch {
    enum class PivotChoice {
        MaxDegreeLocal,
        MaxDegreeLocalX,
    };

    class BronKerboschPivot {
       public:
        template <typename Reporter, typename VertexSet>
        static Reporter::Result explore(UndirectedGraph<VertexSet> const& graph,
                                        PivotChoice pivot_choice) {
            auto cliques = Reporter::empty();
            if (auto const order = graph.order()) {
                // In this initial iteration, we don't need to represent the set of candidates
                // because all neighbours are candidates until excluded.
                auto excluded = Util::with_capacity<VertexSet>(order);
                Vertex const pivot = graph.max_degree_vertex();
                for (Vertex v = 0; v < order; ++v) {
                    auto const& neighbours = graph.neighbours(v);
                    if (!neighbours.empty() && neighbours.count(pivot) == 0) {
                        auto neighbouring_excluded = Util::intersection(neighbours, excluded);
                        if (neighbouring_excluded.size() < neighbours.size()) {
                            auto neighbouring_candidates =
                                Util::difference(neighbours, neighbouring_excluded);
                            auto newclique = VertexPile(v);
                            Reporter::add_all(
                                cliques,
                                visit<Reporter>(graph, pivot_choice, pivot_choice,
                                                std::move(neighbouring_candidates),
                                                std::move(neighbouring_excluded), &newclique));
                        }
                        excluded.insert(v);
                    }
                }
            }
            return cliques;
        }

        template <typename Reporter, typename VertexSet>
        static Reporter::Result visit(UndirectedGraph<VertexSet> const& graph,
                                      PivotChoice initial_pivot_choice,
                                      PivotChoice further_pivot_choice,
                                      VertexSet&& candidates,
                                      VertexSet&& excluded,
                                      VertexPile* clique) {
            assert(!candidates.empty());
            assert(Util::are_disjoint(candidates, excluded));

            auto cliques = Reporter::empty();

            if (candidates.size() == 1) {
                // Same logic as below, but stripped down for this common case
                for (Vertex v : candidates) {
                    auto const& neighbours = graph.neighbours(v);
                    if (Util::are_disjoint(neighbours, excluded)) {
                        Reporter::add_one(cliques, VertexPile(v, clique));
                    }
                }
                return cliques;
            }

            auto pivot = std::numeric_limits<Vertex>::max();
            std::vector<Vertex> remaining_candidates;
            remaining_candidates.reserve(candidates.size());
            // Quickly handle locally unconnected candidates while finding pivot
            size_t seen_local_degree = 0;
            for (Vertex v : candidates) {
                auto const& neighbours = graph.neighbours(v);
                auto local_degree = Util::intersection_size(neighbours, candidates);
                if (local_degree == 0) {
                    // Same logic as below, but stripped down
                    if (Util::are_disjoint(neighbours, excluded)) {
                        Reporter::add_one(cliques, VertexPile(v, clique));
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
                return cliques;
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

            assert(!remaining_candidates.empty());
            for (Vertex v : remaining_candidates) {
                auto const& neighbours = graph.neighbours(v);
                if (neighbours.count(pivot) == 0) {
                    candidates.erase(v);
                    auto neighbouring_candidates = Util::intersection(neighbours, candidates);
                    if (neighbouring_candidates.empty()) {
                        if (Util::are_disjoint(neighbours, excluded)) {
                            Reporter::add_one(cliques, VertexPile(v, clique));
                        }
                    } else {
                        auto neighbouring_excluded = Util::intersection(neighbours, excluded);
                        auto newclique = VertexPile(v, clique);
                        Reporter::add_all(
                            cliques,
                            visit<Reporter>(graph, further_pivot_choice, further_pivot_choice,
                                            std::move(neighbouring_candidates),
                                            std::move(neighbouring_excluded), &newclique));
                    }
                    excluded.insert(v);
                }
            }
            return cliques;
        }
    };
}
