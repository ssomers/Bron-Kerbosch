using System.Collections.Generic;

#pragma warning disable CA1715 // Identifiers should have correct prefix
#pragma warning disable CA1000 // Do not declare static members on generic types

namespace BronKerbosch
{
    public sealed class Degeneracy<VertexSet, VertexSetMgr>
        where VertexSet : ISet<Vertex>
        where VertexSetMgr : IVertexSetMgr<VertexSet>
    {
        private readonly UndirectedGraph<VertexSet, VertexSetMgr> graph;
        private readonly int[] priorityPerVertex;
        private readonly PriorityQueue<Vertex> q;
        private readonly FortifiedCounter<Vertex> leftToPick = new();

        public Degeneracy(UndirectedGraph<VertexSet, VertexSetMgr> graph)
        {
            // Possible values of priorityPerVertex (after initialization):
            //   0: never queued because not connected (degree 0),
            //      or no longer queued because it has been yielded itself,
            //      or no longer queued because all neighbours have been yielded
            //   1 or more: candidates queued with pickedPriority (degree - #of yielded neighbours)
            this.graph = graph;
            priorityPerVertex = new int[graph.Order];
            q = new PriorityQueue<Vertex>(graph.MaxDegree);
            leftToPick = new FortifiedCounter<Vertex>();
            foreach (Vertex v in graph.ConnectedVertices())
            {
                var priority = graph.Degree(v);
                priorityPerVertex[v.Index()] = priority;
                q.Put(priority: priority, element: v);
                leftToPick.Add(v);
            }
        }

        // Enumerate connected vertices in degeneracy order, skipping vertices
        // whose neighbours have all been enumerated already.
        public IEnumerable<Vertex> Iter()
        {
            while (leftToPick.Count > 0)
            {
                Vertex pick = q.Pop();
                ref var pickedPriority = ref priorityPerVertex[pick.Index()];
                if (pickedPriority > 0)
                {
                    pickedPriority = 0;
                    yield return pick;
                    leftToPick.Remove(pick);
                    foreach (Vertex v in graph.Neighbours(pick))
                    {
                        ref var neighbourPriority = ref priorityPerVertex[v.Index()];
                        if (neighbourPriority > 0)
                        {
                            // Requeue with a more urgent priority or dequeue.
                            // Don't bother to remove the original entry from the queue,
                            // since the vertex will be skipped when popped, and thanks to
                            // leftToPick we might not need to pop it at all.
                            neighbourPriority -= 1;
                            if (neighbourPriority > 0)
                            {
                                q.Put(priority: neighbourPriority, element: v);
                            }
                            else
                            {
                                leftToPick.Remove(v);
                            }
                        }
                    }
                }
            }
        }

        public bool IsCandidate(Vertex v) => priorityPerVertex[v.Index()] > 0;
    }
}
