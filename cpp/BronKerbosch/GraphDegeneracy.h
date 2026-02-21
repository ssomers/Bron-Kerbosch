#pragma once

#include "UndirectedGraph.h"
#include <stdexcept>
#include <vector>

namespace BronKerbosch {
    // Enumerate connected vertices in degeneracy order, skipping vertices
    // whose neighbours have all been enumerated already.
    template <typename VertexSet>
    class DegeneracyIter {
      private:
        using Priority = unsigned;

        template <typename T>
        struct PriorityQueue {
            std::vector<std::vector<T>> stack_per_priority;
            size_t num_left_to_pick = 0;

          public:
            explicit PriorityQueue(Priority max_priority)
                : stack_per_priority(max_priority), num_left_to_pick(0) {
            }

            bool empty() const {
                return num_left_to_pick == 0;
            }

            void insert(T element, Priority priority) {
                if (priority > 0) {
                    stack_per_priority[priority - 1].push_back(element);
                    num_left_to_pick += 1;
                }
            }

            // Requeue with a more urgent priority or dequeue.
            // Don't bother to remove the original entry from the queue,
            // since the vertex will be skipped when popped, and thanks to
            // num_left_to_pick we might not need to pop it at all.
            void promote(T element, Priority priority) {
                if (priority > 0) {
                    stack_per_priority[priority - 1].push_back(element);
                } else {
                    forget(element);
                }
            }

            void forget(T) {
                assert(num_left_to_pick > 0);
                num_left_to_pick -= 1;
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
                assert(priority > 0);
#ifdef NDEBUG
                throw std::logic_error("not suitable for use in release code");
#endif
                auto const& stack = stack_per_priority[priority - 1];
                return std::count(stack.begin(), stack.end(), element) > 0;
            }
        };

        UndirectedGraph<VertexSet> const& graph;
        // Possible values of priority_per_vertex (after initialization):
        //   0: never queued because not connected (degree 0),
        //      or no longer queued because it has been yielded itself,
        //      or no longer queued because all neighbours have been yielded
        //   1..maxPriority: candidates queued with priority (degree - #of yielded neighbours)
        std::vector<Priority> priority_per_vertex;
        PriorityQueue<Vertex> queue;

      public:
        explicit DegeneracyIter(UndirectedGraph<VertexSet> const& graph)
            : graph(graph), priority_per_vertex(graph.order(), 0), queue(graph.max_degree()) {
            for (Vertex v : graph.vertices()) {
                auto degree = graph.degree(v);
                priority_per_vertex[v.index()] = degree;
                queue.insert(v, degree);
            }
        }

        bool invariant() const {
            auto order = priority_per_vertex.size();
            for (size_t v = 0; v < order; ++v) {
                Priority priority = priority_per_vertex[v];
                if (priority > 0 && !queue.contains(priority, Vertex(v))) {
                    return false;
                }
            }
            return true;
        }

        bool has_next() const {
            return !queue.empty();
        }

        std::optional<Vertex> next() {
            while (has_next()) {
                Vertex pick = queue.pop();
                Priority& picked_priority = priority_per_vertex[pick.index()];
                if (picked_priority > 0) {
                    picked_priority = 0;
                    queue.forget(pick);
                    reassess(graph.neighbours(pick));
                    return std::make_optional(pick);
                }
            }
            return std::optional<Vertex>{};
        }

      private:
        void reassess(VertexSet const& neighbours) {
            for (Vertex v : neighbours) {
                Priority& priority = priority_per_vertex[v.index()];
                if (priority > 0) {
                    priority -= 1;
                    queue.promote(v, priority);
                }
            }
        }
    };
}
