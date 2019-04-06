// Bron-Kerbosch algorithm with pivot picked arbitrarily

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

public class BronKerbosch2
{
    static public void Explore(UndirectedGraph graph, Reporter reporter)
    {
        var candidates = graph.ConnectedVertices();
        if (candidates.Count > 0)
        {
            Visit(
                graph,
                reporter,
                candidates,
                new HashSet<Vertex>(),
                new List<Vertex>());
        }
    }

    static void Visit(UndirectedGraph graph, Reporter reporter,
                      ISet<Vertex> candidates, ISet<Vertex> excluded, List<Vertex> clique)
    {
        Debug.Assert(candidates.Count > 0);
        var pivot = Util.GetArbitrary(candidates);
        var far_candidates = candidates.Except(graph.Neighbours(pivot)).ToArray();
        foreach (Vertex v in far_candidates)
        {
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Count > 0);
            candidates.Remove(v);
            var neighbouring_candidates = Util.Intersect(candidates, neighbours);
            if (neighbouring_candidates.Any())
            {
                var neighbouring_excluded = Util.Intersect(excluded, neighbours);
                Visit(graph, reporter, neighbouring_candidates, neighbouring_excluded,
                      new List<Vertex>(clique) { v });
            }
            else
            {
                if (Util.AreDisjoint(excluded, neighbours))
                    reporter.Record(new List<Vertex>(clique) { v });
            }
            excluded.Add(v);
        }
    }
}