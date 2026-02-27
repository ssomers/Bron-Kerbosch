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
            //   1 or more: candidates queued with priority (degree - #of yielded neighbours)
            Priority[] priorityPerVertex = [.. Enumerable.Range(0, graph.Order).Select(Vertex.Nth).Select(graph.Degree)];
            var q = new PriorityQueue<Vertex>(graph.MaxDegree);
            foreach (var i in Enumerable.Range(0, graph.Order))
            {
                Vertex v = Vertex.Nth(i);
                var degree = graph.Degree(v);
                if (degree > 0)
                {
                    priorityPerVertex[i] = degree;
                    q.Insert(element: v, priority: degree);
                }
            }

            while (!q.Empty)
            {
                Vertex pick = q.Pop();
                Priority priority = priorityPerVertex[pick.Index()];
                if (priority > 0)
                {
                    // In contrast to most languages, C# continuations allow spawning ASAP,
                    // before we adjust the data. Not that we know it makes a difference.
                    yield return pick;
                    priorityPerVertex[pick.Index()] = 0;
                    q.Forget(pick);
                    foreach (Vertex v in graph.Neighbours(pick))
                    {
                        var oldPriority = priorityPerVertex[v.Index()];
                        if (oldPriority != 0)
                        {
                            var newPriority = oldPriority - 1;
                            priorityPerVertex[v.Index()] = newPriority;
                            q.Promote(v, newPriority);
                        }
                    }
                }
            }
        }
    }

    internal sealed class PriorityQueue<T>(int maxPriority)
    {
        private readonly List<T>[] itsQueuePerPriority = [.. Enumerable.Repeat(true, maxPriority).Select(_ => new List<T>())];
        private int itsNumLeftToPick;
#if DEBUG
        private readonly HashSet<T> itsLeftToPick = [];
#endif

        public bool Empty => itsNumLeftToPick == 0;

        public void Insert(T element, Priority priority)
        {
            Debug.Assert(priority > 0);
            itsQueuePerPriority[priority - 1].Add(element);
            itsNumLeftToPick += 1;
#if DEBUG
            bool added = itsLeftToPick.Add(element);
            Debug.Assert(added);
            Debug.Assert(itsNumLeftToPick == itsLeftToPick.Count);
#endif
        }

        // Requeue with a more urgent priority or dequeue.
        // Don't bother to remove the original entry from the queue,
        // since the vertex will be skipped when popped, and thanks to
        // itsNumLeftToPick we might not need to pop it at all.
        //
        // Assumes the given priority is less than the previous priority
        // that the vertex was assigned.
        public void Promote(T element, Priority priority)
        {
#if DEBUG
            Debug.Assert(itsLeftToPick.Contains(element));
#endif
            if (priority > 0)
            {
                itsQueuePerPriority[priority - 1].Add(element);
            }
            else
            {
                Forget(element);
            }
        }

        // We may return an element already popped, even though it was passed to Forget,
        // in case its priority was promoted earlier on. That's why we do not count 
        // the element as picked, but wait for the caller to Forget it. The caller must
        // somehow ensure to Forget the same element only once.
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

        public void Forget(T element)
        {
            Debug.Assert(itsNumLeftToPick > 0);
            itsNumLeftToPick -= 1;
#if DEBUG
            bool removed = itsLeftToPick.Remove(element);
            Debug.Assert(removed);
            Debug.Assert(itsNumLeftToPick == itsLeftToPick.Count);
#endif
        }
    }
}
