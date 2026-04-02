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

          public:
            explicit PriorityQueue(Priority max_priority) : stack_per_priority(max_priority) {
            }

            void put(T element, Priority priority) {
                if (priority > 0) {
                    stack_per_priority[priority - 1].push_back(element);
                }
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
        size_t num_left_to_pick;

      public:
        explicit DegeneracyIter(UndirectedGraph<VertexSet> const& graph)
            : graph(graph),
              priority_per_vertex(graph.order(), 0),
              queue(graph.max_degree()),
              num_left_to_pick(0) {
            for (Vertex v : graph.vertices()) {
                auto priority = graph.degree(v);
                if (priority > 0) {
                    priority_per_vertex[v.index()] = priority;
                    queue.put(v, priority);
                    num_left_to_pick += 1;
                }
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
            return num_left_to_pick != 0;
        }

        std::optional<std::pair<Vertex, VertexSet>> next() {
            while (has_next()) {
                Vertex pick = queue.pop();
                Priority& picked_priority = priority_per_vertex[pick.index()];
                if (picked_priority > 0) {
                    picked_priority = 0;
                    num_left_to_pick -= 1;
                    auto neighbouring_picked = evaluate_neighbours(pick);
                    return std::make_optional(std::make_pair(pick, std::move(neighbouring_picked)));
                }
            }
            return std::nullopt;
        }

      private:
        VertexSet evaluate_neighbours(Vertex pick) {
            VertexSet result;
            for (Vertex v : graph.neighbours(pick)) {
                Priority& priority = priority_per_vertex[v.index()];
                if (priority > 0) {
                    // Requeue with a more urgent priority or dequeue.
                    // Don't bother to remove the original entry from the queue,
                    // since the vertex will be skipped when popped, and thanks to
                    // num_left_to_pick we might not need to pop it at all.
                    priority -= 1;
                    if (priority > 0) {
                        queue.put(v, priority);
                    } else {
                        assert(num_left_to_pick > 0);
                        num_left_to_pick -= 1;
                    }
                } else {
                    result.insert(v);
                }
            }
            return result;
        }
    };
}
