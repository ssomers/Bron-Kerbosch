// Bron-Kerbosch algorithm with degeneracy ordering,
// with nested searches choosing a pivot from candidates only (IK_GP),
// implemented by multiple threads.

using BronKerbosch;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks;
using System.Threading.Tasks.Dataflow;

internal static class BronKerbosch3MT<VertexSet, VertexSetMgr>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, ICliqueConsumer consumer)
    {
        // Step 2: visit vertices.
        List<Task> tasks = [];
        VertexSet excluded = VertexSetMgr.Empty();
        var visit = new ActionBlock<Vertex>(v =>
            {
                VertexSet neighbours = graph.Neighbours(v);
                Debug.Assert(neighbours.Any());
                VertexSet neighbouringCandidates = VertexSetMgr.Difference(neighbours, excluded);
                if (neighbouringCandidates.Any())
                {
                    VertexSet neighbouringExcluded = VertexSetMgr.Intersection(excluded, neighbours);
                    var task = Task.Run(() =>
                            Pivot<VertexSet, VertexSetMgr>.Visit(graph, consumer, PivotChoice.MaxDegreeLocal,
                                                                 neighbouringCandidates, neighbouringExcluded,
                                                                 [v])
                            );
                    tasks.Add(task);
                }
                else
                {
                    Debug.Assert(VertexSetMgr.Overlaps(neighbours, excluded));
                }
                var added = VertexSetMgr.Add(excluded, v);
                if (!added)
                    throw new InvalidOperationException("Add failed");
            });

        // Step 1: feed vertices in order.
        foreach (Vertex v in Degeneracy<VertexSet, VertexSetMgr>.Ordering(graph, drop: 1))
        {
            if (!visit.Post(v))
                throw new InvalidOperationException("Post failed");
        }
        visit.Complete();
        visit.Completion.Wait();
        Task.WaitAll([.. tasks]);
    }
}
