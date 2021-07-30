// Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

using BronKerbosch;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

internal static class BronKerbosch2GPX
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
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
