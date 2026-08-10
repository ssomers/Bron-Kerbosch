// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot.

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

internal static class DegeneracyBased<VertexSet, VertexSetMgr, TAccumulator>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
    where TAccumulator : ICliqueAccumulator<TAccumulator>, new()
{
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph,
                               CliqueConsumer<TAccumulator> consumer,
                               PivotChoice pivotChoice)
    {
        var degeneracy = new Degeneracy<VertexSet, VertexSetMgr>(graph);
        foreach (Vertex v in degeneracy.Iter())
        {
            VertexSet neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            (VertexSet neighbouringCandidates, VertexSet neighbouringExcluded) =
                VertexSetMgr.Partition(neighbours, degeneracy.IsCandidate);
            Debug.Assert(neighbouringCandidates.Any());
            Pivot<VertexSet, VertexSetMgr, TAccumulator>.Visit(graph, consumer, pivotChoice,
                                                               neighbouringCandidates,
                                                               neighbouringExcluded, [v]);
        }
    }
}
