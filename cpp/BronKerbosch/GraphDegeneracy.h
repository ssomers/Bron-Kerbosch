#pragma once

#include "UndirectedGraph.h"
#include <stdexcept>
#include <vector>

namespace BronKerbosch {
    template <typename VertexSet>
    class DegeneracyOrderIter {
      private:
        using Priority = unsigned;
        static constexpr Priority PRIORITY_NONE = 0;

        template <typename T>
        struct PriorityQueue {
            std::vector<std::vector<T>> stack_per_priority;

            PriorityQueue(Priority max_priority)
                : stack_per_priority(std::vector<std::vector<T>>(max_priority)) {
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
        // If priority is PRIORITY_NONE, the vertex:
        // - was always irrelevant (unconnected);
        // - was already picked itself;
        // - had all its neighbours picked.
        // Otherwise, vertex is still queued and priority = degree - number of picked neighbours.
        PriorityQueue<Vertex> queue;
        int num_left_to_pick;

        DegeneracyOrderIter(UndirectedGraph<VertexSet> const& graph,
                            std::vector<Priority>&& priority_per_vertex,
                            PriorityQueue<Vertex>&& queue,
                            int num_left_to_pick)
            : graph(graph), priority_per_vertex(priority_per_vertex), queue(queue),
              num_left_to_pick(num_left_to_pick) {
        }

      public:
        static DegeneracyOrderIter degeneracy_ordering(UndirectedGraph<VertexSet> const& graph) {
            auto order = graph.order();
            std::vector<Priority> priority_per_vertex(size_t(order), PRIORITY_NONE);
            Priority max_priority = 0;
            for (Vertex c = 0; c < order; ++c) {
                auto degree = graph.degree(c);
                Priority priority = degree;
                priority_per_vertex[c] = priority;
                if (max_priority < priority) {
                    max_priority = priority;
                }
            }

            int num_candidates = 0;
            PriorityQueue<Vertex> queue{max_priority};
            for (Vertex c = 0; c < order; ++c) {
                Priority priority = priority_per_vertex[c];
                if (priority != PRIORITY_NONE) {
                    queue.put(priority, c);
                    num_candidates += 1;
                }
            }

            return DegeneracyOrderIter{graph, std::move(priority_per_vertex), std::move(queue),
                                       num_candidates};
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

        void requeue(VertexSet const& neighbours) {
            for (Vertex v : neighbours) {
                Priority old_priority = priority_per_vertex[v];
                if (old_priority != PRIORITY_NONE) {
                    Priority new_priority = old_priority - 1;
                    priority_per_vertex[v] = new_priority;
                    if (new_priority > 0) {
                        // Requeue with a more urgent priority, but don't bother to remove
                        // the original entry - it will be skipped if it's reached at all.
                        queue.put(new_priority, v);
                    } else {
                        num_left_to_pick -= 1;
                    }
                }
            }
        }

        bool has_next() const {
            return num_left_to_pick > 0;
        }

        std::optional<Vertex> next() {
            while (num_left_to_pick > 0) {
                Vertex pick = queue.pop();
                if (priority_per_vertex[pick] == PRIORITY_NONE) {
                    // v was requeued with a more urgent priority and therefore already picked
                } else {
                    priority_per_vertex[pick] = PRIORITY_NONE;
                    num_left_to_pick -= 1;
                    requeue(graph.neighbours(pick));
                    return std::make_optional(pick);
                }
            }
            return std::optional<Vertex>{};
        }
    };
}
