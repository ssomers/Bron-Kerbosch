// Naive Bron-Kerbosch algorithm

using BronKerbosch;
using Graph;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;

public class BronKerbosch1
{
    static public void explore(UndirectedGraph graph, Reporter reporter)
    {
        var candidates = graph.connected_nodes();
        if (candidates.Any())
        {
            visit(
                graph,
                reporter,
                candidates,
                new HashSet<Vertex>(),
                new List<Vertex>());
        }
    }


    static void visit(UndirectedGraph graph, Reporter reporter, HashSet<Vertex> candidates,
          HashSet<Vertex> excluded, List<Vertex> clique)
    {
        if (candidates.Count == 0 && excluded.Count == 0)
            reporter.record(clique);

        while (candidates.Any())
        {
            Vertex v = candidates.First();
            var neighbours = graph.Neighbours(v);
            Contract.Assume(neighbours.Any());
            candidates.Remove(v);
            visit(
            graph,
            reporter,
            candidates.Intersect(neighbours).ToHashSet(),
            excluded.Intersect(neighbours).ToHashSet(),
            clique.Concat(new[] { v }.ToList()).ToList());
            excluded.Add(v);
        }

    }
}

static class Extensions
{
    public static T RemoveFirst<T>(this ICollection<T> items)
    {
        T item = items.FirstOrDefault();
        if (item != null)
        {
            items.Remove(item);
        }
        return item;
    }
}