// Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

using BronKerbosch;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

internal static class BronKerbosch2aGP
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
    {
        var candidates = new HashSet<Vertex>(graph.ConnectedVertices());
        if (candidates.Any())
        {
            Pivot.Visit(
                graph,
                reporter,
                Pivot.Choice.MaxDegreeLocal,
                candidates,
                new HashSet<Vertex>(),
                ImmutableArray.Create<Vertex>());
        }
    }
}