using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    internal static class Degeneracy<VertexSet, VertexSetMgr>
        where VertexSet : ISet<Vertex>
        where VertexSetMgr : IVertexSetMgr<VertexSet>
    {
        // Enumerate connected vertices in degeneracy order, skipping vertices
        // whose neighbours have all been enumerated already.
        public static IEnumerable<(Vertex, VertexSet)> Iter(UndirectedGraph<VertexSet, VertexSetMgr> graph)
        {
            // Possible values of priorityPerVertex (after initialization):
            //   0: never queued because not connected (degree 0),
            //      or no longer queued because it has been yielded itself,
            //      or no longer queued because all neighbours have been yielded
            //   1 or more: candidates queued with priority (degree - #of yielded neighbours)
            var priorityPerVertex = new int[graph.Order];
            var q = new PriorityQueue<Vertex>(graph.MaxDegree);
            var leftToPick = new FortifiedCounter<Vertex>();
            foreach (var i in Enumerable.Range(0, graph.Order))
            {
                Vertex v = Vertex.Nth(i);
                var priority = graph.Degree(v);
                if (priority > 0)
                {
                    priorityPerVertex[i] = priority;
                    q.Put(priority: priority, element: v);
                    leftToPick.Add(v);
                }
            }

            while (leftToPick.Count > 0)
            {
                Vertex pick = q.Pop();
                if (priorityPerVertex[pick.Index()] > 0)
                {
                    priorityPerVertex[pick.Index()] = 0;
                    var pickedNeighbours = VertexSetMgr.Empty();
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
                                // We discount this neighbour already, but logically it will
                                // be (silently) picked only after we yield the current pick.
                                // So it does not belong in the current pickedNeighbours.
                                leftToPick.Remove(v);
                            }
                        }
                        else
                        {
                            bool added = pickedNeighbours.Add(v);
                            Debug.Assert(added);
                        }
                    }
                    Debug.Assert(pickedNeighbours.Count < graph.Degree(pick));
                    yield return (pick, pickedNeighbours);
                    leftToPick.Remove(pick);
                }
            }
        }
    }
}
