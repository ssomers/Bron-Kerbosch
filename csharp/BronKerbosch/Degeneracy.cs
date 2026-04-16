using System.Collections.Generic;
using System.Diagnostics;

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
            this.graph = graph;

            // Possible values of priorityPerVertex (after initialization):
            //   0: never queued because not connected (degree 0),
            //      or no longer queued because it has been yielded itself,
            //      or no longer queued because all neighbours have been yielded
            //   1 or more: candidates queued with pickedPriority (degree - #of yielded neighbours)
            priorityPerVertex = new int[graph.Order];

            q = new PriorityQueue<Vertex>(graph.MaxDegree);

            // We keep a count not just to verify ourselves, but also to avoid at the end
            // individually popping many vertices that have been queued multiple times.
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
                    AdjustNeighbours(pick);
                }
            }
        }

        private void AdjustNeighbours(Vertex pick)
        {
            foreach (Vertex v in graph.Neighbours(pick))
            {
                ref var priority = ref priorityPerVertex[v.Index()];
                if (priority > 0)
                {
                    Debug.Assert(q.Contains(priority, v));
                    Debug.Assert(leftToPick.Contains(v));
                    // Either queue again with a more urgent priority or dequeue.
                    // Don't bother to remove the original entry from the queue,
                    // since the vertex will be skipped when popped, and thanks to
                    // leftToPick we might not need to pop it at all.
                    priority -= 1;
                    if (priority > 0)
                    {
                        q.Put(priority, v);
                    }
                    else
                    {
                        leftToPick.Remove(v);
                    }
                }
            }
        }

        public bool IsCandidate(Vertex v) => priorityPerVertex[v.Index()] > 0;
    }
}
