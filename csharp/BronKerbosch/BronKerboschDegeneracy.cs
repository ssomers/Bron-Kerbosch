// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot.

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

internal static class BronKerboschDegeneracy<VertexSet, VertexSetMgr>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, ICliqueConsumer consumer, PivotChoice pivotChoice)
    {
        var degeneracy = new Degeneracy<VertexSet, VertexSetMgr>(graph);
        foreach (Vertex v in degeneracy.Iter())
        {
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var (neighbouringCandidates, neighbouringExcluded) = VertexSetMgr.Partition(neighbours, degeneracy.IsCandidate);
            Debug.Assert(neighbouringCandidates.Any());
            Pivot<VertexSet, VertexSetMgr>.Visit(graph, consumer, pivotChoice, neighbouringCandidates, neighbouringExcluded, [v]);
        }
    }
}
