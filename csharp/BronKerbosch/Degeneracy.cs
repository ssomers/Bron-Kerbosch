using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

using Priority = int;

namespace BronKerbosch
{
    internal static class Degeneracy<VertexSet, VertexSetMgr>
        where VertexSet : ISet<Vertex>
        where VertexSetMgr : IVertexSetMgr<VertexSet>
    {
        // Enumerate connected vertices in degeneracy order, skipping vertices
        // whose neighbours have all been enumerated already.
        public static IEnumerable<Vertex> Iter(UndirectedGraph<VertexSet, VertexSetMgr> graph)
        {
            // Possible values of priorityPerVertex (after initialization):
            //   0: never queued because not connected (degree 0),
            //      or no longer queued because it has been yielded itself,
            //      or no longer queued because all neighbours have been yielded
            //   1..maxPriority: candidates queued with priority (degree - #of yielded neighbours)
            var priorityPerVertex = new Priority[graph.Order];
            Priority maxPriority = 0;
            foreach (var i in Enumerable.Range(0, graph.Order))
            {
                Vertex v = Vertex.Nth(i);
                var degree = graph.Degree(v);
                priorityPerVertex[i] = degree;
                maxPriority = Math.Max(maxPriority, degree);
            }

            var q = new PriorityQueue<Vertex>(maxPriority);
            var numLeftToPick = 0;
            foreach ((Vertex v, Priority priority) in priorityPerVertex.Select((p, i) => (Vertex.Nth(i), p)))
            {
                if (priority > 0)
                {
                    q.Put(priority: priority, element: v);
                    numLeftToPick += 1;
                }
            }

            while (numLeftToPick > 0)
            {
                Vertex pick = q.Pop();
                Priority priority = priorityPerVertex[pick.Index()];
                if (priority > 0)
                {
                    // In contrast to most languages, C# continuations allow spawning ASAP,
                    // before we adjust the data. Not that we know it makes a difference.
                    yield return pick;
                    priorityPerVertex[pick.Index()] = 0;
                    numLeftToPick -= 1;
                    foreach (Vertex v in graph.Neighbours(pick))
                    {
                        var oldPriority = priorityPerVertex[v.Index()];
                        if (oldPriority != 0)
                        {
                            // Requeue with a more urgent priority or dequeue.
                            // Don't bother to remove the original entry from the queue,
                            // since the vertex will be skipped when popped, and thanks to
                            // numLeftToPick we might not need to pop it at all.
                            var newPriority = oldPriority - 1;
                            priorityPerVertex[v.Index()] = newPriority;
                            if (newPriority > 0)
                            {
                                q.Put(priority: newPriority, element: v);
                            }
                            else
                            {
                                Debug.Assert(numLeftToPick > 0);
                                numLeftToPick -= 1;
                            }
                        }
                    }
                }
            }
        }
    }

    internal sealed class PriorityQueue<T>(int maxPriority)
    {
        private readonly List<T>[] itsQueuePerPriority = [.. Enumerable.Repeat(true, maxPriority).Select(_ => new List<T>())];

        public void Put(Priority priority, T element)
        {
            Debug.Assert(priority > 0);
            itsQueuePerPriority[priority - 1].Add(element);
        }

        // We may return an element already popped earlier, in case its priority was promoted.
        public T Pop()
        {
            foreach (List<T> stack in itsQueuePerPriority)
            {
                if (stack.Count > 0)
                {
                    var last = stack.Count - 1;
                    T element = stack[last];
                    stack.RemoveAt(last);
                    return element;
                }
            }
            throw new ArgumentException("Cannot pop more than has been put");
        }
    }
}
