// Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

using BronKerbosch;
using System.Collections.Generic;
using System.Linq;

internal static class BronKerbosch2aGP<VertexSet, VertexSetMgr>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, ICliqueConsumer consumer)
    {
        var candidates = VertexSetMgr.From(graph.ConnectedVertices());
        if (candidates.Any())
        {
            Pivot<VertexSet, VertexSetMgr>.Visit(
                graph,
                consumer,
                PivotChoice.MaxDegreeLocal,
                candidates,
                VertexSetMgr.Empty(),
                []);
        }
    }
}
