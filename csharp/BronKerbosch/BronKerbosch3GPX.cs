// Bron-Kerbosch algorithm with degeneracy ordering,
// choosing a pivot from both candidates and excluded vertices (IK_GPX).

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

public class BronKerbosch3GPX
{
    static public void Explore(UndirectedGraph graph, Reporter reporter)
    {
        var excluded = new HashSet<Vertex>();
        foreach (Vertex v in Degeneracy.Ordering(graph, drop: 1))
        {
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var neighbouring_candidates = Util.Difference(neighbours, excluded);
            if (neighbouring_candidates.Any())
            {
                var neighbouring_excluded = Util.Intersect(excluded, neighbours);
                Pivot.Visit(graph, reporter,
                            Pivot.Choice.MaxDegreeLocalX, Pivot.Choice.MaxDegreeLocalX,
                            neighbouring_candidates, neighbouring_excluded,
                            new List<Vertex>() { v });
            }
            else
            {
                Debug.Assert(!Util.AreDisjoint(neighbours, excluded));
            }
            excluded.Add(v);
        }
    }
}