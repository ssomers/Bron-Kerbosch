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
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, IReporter reporter, PivotChoice pivotChoice)
    {
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        var excluded = VertexSetMgr.EmptyWithCapacity(graph.Order);
        foreach (var v in Degeneracy<VertexSet, VertexSetMgr>.Ordering(graph, drop: 1))
        {
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var neighbouringExcluded = VertexSetMgr.Intersection(excluded, neighbours);
            if (neighbouringExcluded.Count < neighbours.Count)
            {
                var neighbouringCandidates = VertexSetMgr.Difference(neighbours, neighbouringExcluded);
                Pivot<VertexSet, VertexSetMgr>.Visit(graph, reporter, pivotChoice,
                    neighbouringCandidates, neighbouringExcluded,
                    [v]);
            }
            var added = VertexSetMgr.Add(excluded, v);
            Debug.Assert(added);
        }
    }
}
