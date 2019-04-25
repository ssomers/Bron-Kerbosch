using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    public class Degeneracy
    {
        // Iterate connected vertices, lowest degree first.
        // drop=N: omit last N vertices
        public static IEnumerable<Vertex> Ordering(UndirectedGraph graph, int drop)
        {
            Debug.Assert(drop >= 0);
            const int NO_PRIORITY = -1;
            int[] priorityPerVertex = new int[graph.Order];
            var max_priority = 0;
            var numLeftToPick = 0;
            foreach (var i in Enumerable.Range(0, graph.Order))
            {
                var c = new Vertex(i);
                var degree = graph.Degree(c);
                if (degree > 0)
                {
                    var priority = degree;
                    max_priority = Math.Max(max_priority, priority);
                    priorityPerVertex[i] = degree;
                    numLeftToPick += 1;
                }
            }
            // Possible values of priority_per_vertex:
            //   no_priority: when yielded or if unconnected
            //   0..max_priority: candidates still queued with priority (degree - #of yielded neighbours)
            var q = new PriorityQueue(max_priority);
            foreach (var (c, p) in priorityPerVertex.Select((p, i) => (new Vertex(i), p)))
            {
                if (p > 0)
                    q.Put(priority: p, element: c);
            }

            numLeftToPick -= drop;
            while (numLeftToPick >= 0)
            {
                numLeftToPick -= 1;
                var i = q.Pop();
                while (priorityPerVertex[i] == NO_PRIORITY)
                {
                    // was requeued with a more urgent priority and therefore already picked
                    i = q.Pop();
                }
                Debug.Assert(priorityPerVertex[i] >= 0);
                priorityPerVertex[i] = NO_PRIORITY;
                yield return i;
                foreach (Vertex v in graph.Neighbours(i))
                {
                    var oldPriority = priorityPerVertex[v];
                    if (oldPriority != NO_PRIORITY)
                    {
                        // Since this is an unvisited neighbour of a vertex just being picked,
                        // its priority can't be down to the minimum.
                        Debug.Assert(oldPriority > 0);
                        // Requeue with a more urgent priority, but don't bother to remove
                        // the original entry - it will be skipped if it's reached at all.
                        priorityPerVertex[v] = oldPriority - 1;
                        q.Put(priority: oldPriority - 1, element: v);
                    }
                }
            }
        }
    }

    class PriorityQueue
    {
        private List<Vertex>[] queuePerPriority;

        public PriorityQueue(int max_priority)
        {
            queuePerPriority = new List<Vertex>[max_priority + 1];
        }

        public void Put(int priority, Vertex element)
        {
            Debug.Assert(priority >= 0);
            if (queuePerPriority[priority] == null)
                queuePerPriority[priority] = new List<Vertex>();
            queuePerPriority[priority].Add(element);
        }

        public Vertex Pop()
        {
            foreach (var queue in queuePerPriority)
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