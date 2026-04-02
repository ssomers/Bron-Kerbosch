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
        // Step 3: further explore visited vertices.
        ActionBlock<Action> spawner = new(action => action(), spawnerOptions);

        // Step 2: visit vertices.
        VertexSet excluded = VertexSetMgr.Empty();
        var visitor = new ActionBlock<(Vertex, VertexSet)>(pair =>
            {
                (Vertex v, VertexSet neighbouringExcluded) = pair;
                VertexSet neighbours = graph.Neighbours(v);
                Debug.Assert(neighbours.Any());
                if (neighbouringExcluded.Count < neighbours.Count)
                {
                    var neighbouringCandidates = VertexSetMgr.Difference(neighbours, neighbouringExcluded);
                    var posted = spawner.Post(() =>
                        Pivot<VertexSet, VertexSetMgr>.Visit(graph, consumer, PivotChoice.MaxDegreeLocal,
                                                                neighbouringCandidates, neighbouringExcluded,
                                                                [v])
                        );
                    Trace.Assert(posted);
                }
                else
                {
                    Debug.Assert(VertexSetMgr.Overlaps(neighbours, excluded));
                }
                var added = VertexSetMgr.Add(excluded, v);
                Trace.Assert(added);
            });

        // Step 1: order vertices.
        foreach (var pair in Degeneracy<VertexSet, VertexSetMgr>.Iter(graph))
        {
            var posted = visitor.Post(pair);
            Trace.Assert(posted);
        }
        visitor.Complete();
        visitor.Completion.Wait();
        spawner.Complete();
        spawner.Completion.Wait();
    }
}
