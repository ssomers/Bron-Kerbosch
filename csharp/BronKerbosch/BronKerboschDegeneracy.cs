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
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        var excluded = new bool[graph.Order];
        foreach (Vertex v in Degeneracy<VertexSet, VertexSetMgr>.Iter(graph))
        {
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var neighbouringExcluded = VertexSetMgr.Intersection(neighbours, excluded);
            if (neighbouringExcluded.Count < neighbours.Count)
            {
                var neighbouringCandidates = VertexSetMgr.Difference(neighbours, neighbouringExcluded);
                Pivot<VertexSet, VertexSetMgr>.Visit(graph, consumer, pivotChoice,
                    neighbouringCandidates, neighbouringExcluded,
                    [v]);
            }
            Debug.Assert(!excluded[v.Index()]);
            excluded[v.Index()] = true;
        }
    }
}
