// Naïve Bron-Kerbosch algorithm

using BronKerbosch;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;

internal static class BronKerbosch1<VertexSet, VertexSetMgr>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, IReporter reporter)
    {
        var candidates = VertexSetMgr.From(graph.ConnectedVertices());
        if (candidates.Count > 0)
        {
            Visit(
                graph,
                reporter,
                candidates,
                VertexSetMgr.EmptyWithCapacity(candidates.Count),
                []);
        }
    }


    private static void Visit(UndirectedGraph<VertexSet, VertexSetMgr> graph, IReporter reporter,
        VertexSet candidates, VertexSet excluded, ImmutableArray<Vertex> cliqueInProgress)
    {
        Debug.Assert(candidates.All(v => graph.Degree(v) > 0));
        Debug.Assert(excluded.All(v => graph.Degree(v) > 0));
        Debug.Assert(!VertexSetMgr.Overlaps(candidates, excluded));
        Debug.Assert(candidates.Any());
        while (candidates.Any())
        {
            var v = VertexSetMgr.PopArbitrary(candidates);
            var neighbours = graph.Neighbours(v);
            var neighbouringCandidates = VertexSetMgr.Intersection(candidates, neighbours);
            if (neighbouringCandidates.Any())
            {
                var neighbouringExcluded = VertexSetMgr.Intersection(excluded, neighbours);
                Visit(graph, reporter,
                    neighbouringCandidates, neighbouringExcluded,
                    [.. cliqueInProgress, v]);
            }
            else if (!VertexSetMgr.Overlaps(excluded, neighbours))
            {
                reporter.Record([.. cliqueInProgress, v]);
            }
            var added = VertexSetMgr.Add(excluded, v);
            Debug.Assert(added);
        }
    }
}
