// Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

using BronKerbosch;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;
using Vertex = System.UInt32;

public class BronKerbosch2GPX
{
    static public void Explore(UndirectedGraph graph, Reporter reporter)
    {
        var candidates = new HashSet<Vertex>(graph.ConnectedVertices());
        if (candidates.Any())
        {
            Pivot.Visit(
                graph,
                reporter,
                Pivot.Choice.MaxDegree,
                Pivot.Choice.MaxDegreeLocalX,
                candidates,
                new HashSet<Vertex>(capacity: candidates.Count),
                ImmutableArray.Create<Vertex>());
        }
    }
}
