// Bron-Kerbosch algorithm with degeneracy ordering,
// with nested searches choosing a pivot from candidates only (IK_GP),
// implemented by multiple threads.

using BronKerbosch;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks.Dataflow;

internal static class BronKerbosch3MT<VertexSet, VertexSetMgr>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    private static readonly ExecutionDataflowBlockOptions spawnerOptions = new() { MaxDegreeOfParallelism = 5 };

    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, ICliqueConsumer consumer)
    {
        // Step 2: explore visited vertices.
        ActionBlock<Action> spawner = new(action => action(), spawnerOptions);

        // Step 1: order & visit vertices.
        var degeneracy = new Degeneracy<VertexSet, VertexSetMgr>(graph);
        foreach (Vertex v in degeneracy.Iter())
        {
            VertexSet neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var (neighbouringCandidates, neighbouringExcluded) = VertexSetMgr.Partition(neighbours, degeneracy.IsCandidate);
            Debug.Assert(neighbouringCandidates.Any());
            var posted = spawner.Post(() =>
                Pivot<VertexSet, VertexSetMgr>.Visit(graph, consumer, PivotChoice.MaxDegreeLocal,
                                                        neighbouringCandidates, neighbouringExcluded,
                                                        [v])
                );
            Trace.Assert(posted);
        }
        spawner.Complete();
        spawner.Completion.Wait();
    }
}
