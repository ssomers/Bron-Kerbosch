#pragma once

#include "UndirectedGraph.h"
#include <stdexcept>
#include <vector>

namespace BronKerbosch {
    template <typename VertexSet>
    class DegeneracyOrderIter {
    private:
        using Priority = unsigned;
        static Priority const PRIORITY_NONE = 0;

        template <typename T>
        struct PriorityQueue {
            std::vector<std::vector<T>> stack_per_priority;

            PriorityQueue(Priority max_priority) : stack_per_priority(
                std::vector<std::vector<T>>(max_priority)) {
            }

            void put(Priority priority, T element) {
                assert(priority != PRIORITY_NONE);
                stack_per_priority[priority - 1].push_back(element);
            }

            T pop() {
                for (auto& stack : stack_per_priority) {
                    if (!stack.empty()) {
                        T last = *stack.rbegin();
                        stack.pop_back();
                        return last;
                    }
                }
                throw std::logic_error("cannot pop more than has been put");
            }

            bool contains(Priority priority, T element) const {
                assert(priority != PRIORITY_NONE);
#ifdef NDEBUG
                throw std::logic_error("not suitable for use in release code");
#endif
                auto const& stack = stack_per_priority[priority - 1];
                return std::find(stack.begin(), stack.end(), element) != stack.end();
            }
        };

        UndirectedGraph<VertexSet> const& graph;
        std::vector<Priority> priority_per_vertex;
        // If priority is PRIORITY_NONE, vertex was already picked or was always irrelevant (unconnected);
        // otherwise, vertex is still queued and priority = degree - number of picked neighbours +1.
        // +1 because we want the priority number to be NonZero to allow free wrapping inside Option.
        PriorityQueue<Vertex> queue;
        int num_left_to_pick;

        DegeneracyOrderIter(UndirectedGraph<VertexSet> const& graph,
                            std::vector<Priority>&& priority_per_vertex,
                            PriorityQueue<Vertex>&& queue,
                            int num_left_to_pick) :
            graph(graph),
            priority_per_vertex(priority_per_vertex),
            queue(queue),
            num_left_to_pick(num_left_to_pick) {
        }

    public:
        static DegeneracyOrderIter degeneracy_ordering(UndirectedGraph<VertexSet> const& graph, int drop = 0) {
            assert(drop <= 0);
            auto order = graph.order();
            std::vector<Priority> priority_per_vertex(size_t(order), PRIORITY_NONE);
            Priority max_priority = PRIORITY_NONE;
            int num_candidates = 0;
            for (Vertex c = 0; c < order; ++c) {
                auto degree = graph.degree(c);
                if (degree > 0) {
                    Priority priority = degree + 1;
                    priority_per_vertex[c] = priority;
                    if (max_priority < priority) {
                        max_priority = priority;
                    }
                    assert(max_priority != PRIORITY_NONE);
                    num_candidates += 1;
                }
            }
            PriorityQueue<Vertex> queue{ max_priority };
            for (Vertex c = 0; c < order; ++c) {
                Priority priority = priority_per_vertex[c];
                if (priority != PRIORITY_NONE) {
                    queue.put(priority, c);
                }
            }

            return DegeneracyOrderIter{
                graph,
                std::move(priority_per_vertex),
                std::move(queue),
                std::max(0, num_candidates + drop)
            };
        }

        bool invariant() const {
            auto order = priority_per_vertex.size();
            for (Vertex v = 0; v < order; ++v) {
                Priority p = priority_per_vertex[v];
                if (p == PRIORITY_NONE) {
                    // might still be in some stack
                } else if (!queue.contains(p, v)) {
                    return false;
                }
            }
            return true;
        }

        Vertex pick_with_lowest_degree() {
            assert(invariant());
            for (;;) {
                Vertex v = queue.pop();
                if (priority_per_vertex[v] != PRIORITY_NONE) {
                    priority_per_vertex[v] = PRIORITY_NONE;
                    return v;
                }
                // else v was requeued with a more urgent priority and therefore already picked
            }
        }

        bool has_next() const {
            return num_left_to_pick > 0;
        }

        std::optional<Vertex> next() {
            if (num_left_to_pick > 0) {
                num_left_to_pick -= 1;
                Vertex i = pick_with_lowest_degree();
                for (Vertex v : graph.neighbours(i)) {
                    Priority old_priority = priority_per_vertex[v];
                    if (old_priority != PRIORITY_NONE) {
                    // Since this is an unvisited neighbour of a vertex just being picked,
                    // its priority can't be down to the minimum.
                        Priority new_priority = old_priority - 1;
                        assert(new_priority != PRIORITY_NONE);
                        // Requeue with a more urgent priority, but don't bother to remove
                        // the original entry - it will be skipped if it's reached at all.
                        priority_per_vertex[v] = new_priority;
                        queue.put(new_priority, v);
                    }
                }
                return std::make_optional(i);
            } else {
                return std::optional<Vertex>{};
            }
        }
    };
}
