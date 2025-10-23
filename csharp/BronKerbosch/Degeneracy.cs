using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    internal static class Degeneracy<VertexSet, VertexSetMgr>
        where VertexSet : IEnumerable<Vertex>
        where VertexSetMgr : IVertexSetMgr<VertexSet>
    {
        // Iterate connected vertices, lowest degree first.
        // drop=N: omit last N vertices
        public static IEnumerable<Vertex> Ordering(UndirectedGraph<VertexSet, VertexSetMgr> graph, int drop)
        {
            Debug.Assert(drop >= 0);
            const int NO_PRIORITY = -1;
            var priorityPerVertex = new int[graph.Order];
            var maxPriority = 0;
            var numLeftToPick = 0;
            foreach (var i in Enumerable.Range(0, graph.Order))
            {
                var c = Vertex.Nth(i);
                var degree = graph.Degree(c);
                if (degree > 0)
                {
                    var priority = degree;
                    maxPriority = Math.Max(maxPriority, priority);
                    priorityPerVertex[i] = degree;
                    numLeftToPick += 1;
                }
            }

            // Possible values of priority_per_vertex:
            //   no_priority: when yielded or if unconnected
            //   0..maxPriority: candidates still queued with priority (degree - #of yielded neighbours)
            var q = new PriorityQueue(maxPriority);
            foreach (var (c, p) in priorityPerVertex.Select((p, i) => (Vertex.Nth(i), p)))
            {
                if (p > 0)
                {
                    q.Put(priority: p, element: c);
                }
            }

            numLeftToPick -= drop;
            while (numLeftToPick >= 0)
            {
                numLeftToPick -= 1;
                var i = q.Pop();
                while (priorityPerVertex[i.Index()] == NO_PRIORITY)
                {
                    // was requeued with a more urgent priority and therefore already picked
                    i = q.Pop();
                }
                Debug.Assert(priorityPerVertex[i.Index()] >= 0);
                priorityPerVertex[i.Index()] = NO_PRIORITY;
                yield return i;
                foreach (var v in graph.Neighbours(i))
                {
                    var oldPriority = priorityPerVertex[v.Index()];
                    if (oldPriority != NO_PRIORITY)
                    {
                        // Since this is an unvisited neighbour of a vertex just being picked,
                        // its priority can't be down to the minimum.
                        Debug.Assert(oldPriority > 0);
                        // Requeue with a more urgent priority, but don't bother to remove
                        // the original entry - it will be skipped if it's reached at all.
                        priorityPerVertex[v.Index()] = oldPriority - 1;
                        q.Put(priority: oldPriority - 1, element: v);
                    }
                }
            }
        }
    }

    internal sealed class PriorityQueue(int maxPriority)
    {
        private readonly List<Vertex>[] itsQueuePerPriority = new List<Vertex>[maxPriority + 1];

        public void Put(int priority, Vertex element)
        {
            Debug.Assert(priority >= 0);
            itsQueuePerPriority[priority] ??= [];
            itsQueuePerPriority[priority].Add(element);
        }

        public Vertex Pop()
        {
            foreach (var queue in itsQueuePerPriority)
            {
                if (queue != null)
                {
                    var last = queue.Count - 1;
                    if (last >= 0)
                    {
                        var v = queue[last];
                        queue.RemoveAt(last);
                        return v;
                    }
                }
            }
            throw new ArgumentException("Cannot pop more than has been put");
        }
    }
}
