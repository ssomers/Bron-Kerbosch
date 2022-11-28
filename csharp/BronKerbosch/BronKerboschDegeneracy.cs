// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot.

using BronKerbosch;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;

internal static class BronKerboschDegeneracy
{
    public static void Explore(UndirectedGraph graph, IReporter reporter, PivotChoice pivotChoice)
    {
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        var excluded = new HashSet<Vertex>(capacity: graph.Order);
        foreach (var v in Degeneracy.Ordering(graph, drop: 1))
        {
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var neighbouringExcluded = CollectionsUtil.Intersection(excluded, neighbours);
            if (neighbouringExcluded.Count < neighbours.Count)
            {
                var neighbouringCandidates = CollectionsUtil.Difference(neighbours, neighbouringExcluded);
                Pivot.Visit(graph, reporter, pivotChoice,
                    neighbouringCandidates, neighbouringExcluded,
                    ImmutableArray.Create(v));
            }
            var added = excluded.Add(v);
            Debug.Assert(added);
        }
    }
}
