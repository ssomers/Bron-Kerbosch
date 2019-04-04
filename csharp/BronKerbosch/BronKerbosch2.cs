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
        if (candidates.Any())
        {
            Visit(
                graph,
                reporter,
                candidates,
                new HashSet<Vertex>(),
                new List<Vertex>());
        }
    }

    static void Visit(UndirectedGraph graph, Reporter reporter, HashSet<Vertex> candidates,
          HashSet<Vertex> excluded, List<Vertex> clique)
    {
        if (!candidates.Any())
        {
            if (!excluded.Any())
                reporter.Record(clique);
            return;
        }

        var pivot = candidates.First();
        var far_candidates = candidates.Except(graph.Neighbours(pivot)).ToArray();
        foreach (Vertex v in far_candidates)
        {
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            candidates.Remove(v);
            Visit(graph, reporter,
                  candidates.Intersect(neighbours).ToHashSet(),
                  excluded.Intersect(neighbours).ToHashSet(),
                  clique.Concat(new[] { v }).ToList());
            bool ok = excluded.Add(v);
            Debug.Assert(ok);
        }
    }
}