// Bron-Kerbosch algorithm with pivot picked arbitrarily

using BronKerbosch;
using System.Collections.Generic;
using System.Linq;

public class BronKerbosch2GP
{
    static public void Explore(UndirectedGraph graph, Reporter reporter)
    {
        var candidates = graph.ConnectedVertices();
        if (candidates.Any())
        {
            Pivot.Visit(
                graph,
                reporter,
                Pivot.Choice.MaxDegree,
                Pivot.Choice.MaxDegreeLocal,
                candidates,
                new HashSet<Vertex>(),
                new List<Vertex>(capacity: candidates.Count));
        }
    }
}