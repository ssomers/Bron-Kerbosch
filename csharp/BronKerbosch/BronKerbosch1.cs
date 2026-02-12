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
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, ICliqueConsumer consumer)
    {
        VertexSet candidates = VertexSetMgr.From(graph.ConnectedVertices());
        if (candidates.Count > 0)
        {
            Visit(
                graph,
                consumer,
                candidates,
                VertexSetMgr.EmptyWithCapacity(candidates.Count),
                []);
        }
    }


    private static void Visit(UndirectedGraph<VertexSet, VertexSetMgr> graph, ICliqueConsumer consumer,
        VertexSet candidates, VertexSet excluded, ImmutableArray<Vertex> cliqueInProgress)
    {
        Debug.Assert(candidates.All(graph.HasNeighbours));
        Debug.Assert(excluded.All(graph.HasNeighbours));
        Debug.Assert(!VertexSetMgr.Overlaps(candidates, excluded));
        Debug.Assert(candidates.Any());
        while (candidates.Any())
        {
            Vertex v = candidates.PopArbitrary();
            VertexSet neighbours = graph.Neighbours(v);
            VertexSet neighbouringCandidates = VertexSetMgr.Intersection(candidates, neighbours);
            if (neighbouringCandidates.Any())
            {
                VertexSet neighbouringExcluded = VertexSetMgr.Intersection(excluded, neighbours);
                Visit(graph, consumer,
                    neighbouringCandidates, neighbouringExcluded,
                    [.. cliqueInProgress, v]);
            }
            else if (!VertexSetMgr.Overlaps(excluded, neighbours))
            {
                consumer.Accept([.. cliqueInProgress, v]);
            }
            var added = VertexSetMgr.Add(excluded, v);
            Debug.Assert(added);
        }
    }
}
