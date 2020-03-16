// Naive Bron-Kerbosch algorithm

using BronKerbosch;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;
using Vertex = System.UInt32;

class BronKerbosch1
{
    static public void Explore(UndirectedGraph graph, Reporter reporter)
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


    static void Visit(UndirectedGraph graph, Reporter reporter,
                      ISet<Vertex> candidates, ISet<Vertex> excluded, ImmutableArray<Vertex> cliqueInProgress)
    {
        while (true)
        {
            Vertex v = Util.PopArbitrary(candidates);
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var neighbouring_candidates = Util.Intersection(candidates, neighbours);
            if (neighbouring_candidates.Any())
            {
                var neighbouring_excluded = Util.Intersection(excluded, neighbours);
                Visit(graph, reporter, neighbouring_candidates, neighbouring_excluded,
                      Util.Append(cliqueInProgress, v));
            }
            else
            {
                if (Util.AreDisjoint(excluded, neighbours))
                    reporter.Record(Util.Append(cliqueInProgress, v));
                if (!candidates.Any())
                    break;
            }
            excluded.Add(v);
        }
    }
}
