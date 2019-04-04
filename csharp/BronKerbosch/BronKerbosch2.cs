// Bron-Kerbosch algorithm with pivot picked arbitrarily

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;

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

    static void Visit(UndirectedGraph graph, Reporter reporter, ISet<Vertex> candidates,
          ISet<Vertex> excluded, List<Vertex> clique)
    {
        if (candidates.Count == 0)
        {
            if (excluded.Count == 0)
                reporter.Record(clique);
            return;
        }

        var pivot = GetArbitrary(candidates);
        ISet<Vertex> far_candidates = new HashSet<Vertex>(candidates);
        far_candidates.ExceptWith(graph.Neighbours(pivot));
        foreach (Vertex v in far_candidates)
        {
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Count > 0);
            candidates.Remove(v);
            ISet<Vertex> neighbouring_candidates = new HashSet<Vertex>(candidates);
            neighbouring_candidates.IntersectWith(neighbours);
            ISet<Vertex> neighbouring_excluded = new HashSet<Vertex>(excluded);
            neighbouring_excluded.IntersectWith(neighbours);
            Visit(graph, reporter, neighbouring_candidates, neighbouring_excluded, new List<Vertex>(clique) { v });
            excluded.Add(v);
        }
    }

    private static Vertex GetArbitrary(ISet<Vertex> candidates)
    {
        var en = candidates.GetEnumerator();
        var ok = en.MoveNext();
        Debug.Assert(ok);
        return en.Current;
    }
}