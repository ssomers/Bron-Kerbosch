// Naive Bron-Kerbosch algorithm

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

class BronKerbosch1
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
        if (!candidates.Any() && !excluded.Any())
            reporter.Record(clique);

        while (candidates.Any())
        {
            Vertex v = candidates.First();
            candidates.Remove(v);
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            Visit(graph, reporter,
                  candidates.Intersect(neighbours).ToHashSet(),
                  excluded.Intersect(neighbours).ToHashSet(),
                  clique.Concat(new[] { v }).ToList());
            excluded.Add(v);
        }
    }
}