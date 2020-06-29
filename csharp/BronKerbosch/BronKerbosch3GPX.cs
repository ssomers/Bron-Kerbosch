// Bron-Kerbosch algorithm with degeneracy ordering,
// choosing a pivot from both candidates and excluded vertices (IK_GPX).

using BronKerbosch;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;
using Vertex = System.UInt32;

internal static class BronKerbosch3GPX
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
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
                    Pivot.Choice.MaxDegreeLocalX, Pivot.Choice.MaxDegreeLocalX,
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
