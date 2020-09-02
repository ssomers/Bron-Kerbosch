// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot.

using BronKerbosch;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;
using Vertex = System.UInt32;

internal static class BronKerboschDegeneracy
{
    public static void Explore(UndirectedGraph graph, IReporter reporter, Pivot.Choice pivotChoice)
    {
        var excluded = new HashSet<Vertex>();
        foreach (var v in Degeneracy.Ordering(graph, drop: 1))
        {
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var neighbouringCandidates = CollectionsUtil.Difference(neighbours, excluded);
            if (neighbouringCandidates.Any())
            {
                var neighbouringExcluded = CollectionsUtil.Intersection(excluded, neighbours);
                Pivot.Visit(graph, reporter,
                    pivotChoice, pivotChoice,
                    neighbouringCandidates, neighbouringExcluded,
                    ImmutableArray.Create(v));
            }
            else
            {
                Debug.Assert(!CollectionsUtil.AreDisjoint(neighbours, excluded));
            }
            excluded.Add(v);
        }
    }
}
