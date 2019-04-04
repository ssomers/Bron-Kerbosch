// Naive Bron-Kerbosch algorithm

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;

class BronKerbosch1
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
        if (candidates.Count == 0 && excluded.Count == 0)
            reporter.Record(clique);

        while (candidates.Count > 0)
        {
            Vertex v = PopArbitrary(candidates);
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Count > 0);
            ISet<Vertex> neighbouring_candidates = new HashSet<Vertex>(candidates);
            neighbouring_candidates.IntersectWith(neighbours);
            ISet<Vertex> neighbouring_excluded = new HashSet<Vertex>(excluded);
            neighbouring_excluded.IntersectWith(neighbours);
            Visit(graph, reporter, neighbouring_candidates, neighbouring_excluded, new List<Vertex>(clique) { v });
            excluded.Add(v);
        }
    }

    private static Vertex PopArbitrary(ISet<Vertex> candidates)
    {
        var en = candidates.GetEnumerator();
        var ok = en.MoveNext();
        Debug.Assert(ok);
        candidates.Remove(en.Current);
        return en.Current;
    }
}