// Naive Bron-Kerbosch algorithm

using BronKerbosch;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;
using Vertex = System.UInt32;

class BronKerbosch1
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
    {
        var candidates = new HashSet<Vertex>(graph.ConnectedVertices());
        if (candidates.Any())
        {
            Visit(
                graph,
                reporter,
                candidates,
                new HashSet<Vertex>(capacity: candidates.Count),
                ImmutableArray.Create<Vertex>());
        }
    }


    private static void Visit(UndirectedGraph graph, IReporter reporter,
        ISet<Vertex> candidates, ISet<Vertex> excluded, ImmutableArray<Vertex> cliqueInProgress)
    {
        while (true)
        {
            var v = CollectionsUtil.PopArbitrary(candidates);
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var neighbouringCandidates = CollectionsUtil.Intersection(candidates, neighbours);
            if (neighbouringCandidates.Any())
            {
                var neighbouringExcluded = CollectionsUtil.Intersection(excluded, neighbours);
                Visit(graph, reporter, neighbouringCandidates, neighbouringExcluded,
                    CollectionsUtil.Append(cliqueInProgress, v));
            }
            else
            {
                if (CollectionsUtil.AreDisjoint(excluded, neighbours))
                    reporter.Record(CollectionsUtil.Append(cliqueInProgress, v));
                if (!candidates.Any())
                    break;
            }
            excluded.Add(v);
        }
    }
}
