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
        if (candidates.Count > 0)
        {
            Visit(
                graph,
                reporter,
                candidates,
                new HashSet<Vertex>(),
                new List<Vertex>(capacity: candidates.Count));
        }
    }


    static void Visit(UndirectedGraph graph, Reporter reporter,
                      ISet<Vertex> candidates, ISet<Vertex> excluded, List<Vertex> clique)
    {
        while (true)
        {
            Vertex v = Util.PopArbitrary(candidates);
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Count > 0);
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
                if (candidates.Count == 0)
                    break;
            }
            excluded.Add(v);
        }
    }
}
