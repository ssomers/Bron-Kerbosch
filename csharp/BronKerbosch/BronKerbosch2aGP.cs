// Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

using BronKerbosch;
using System.Collections.Generic;
using System.Linq;

internal static class BronKerbosch2aGP<VertexSet, VertexSetMgr, TAccumulator>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
    where TAccumulator : ICliqueAccumulator<TAccumulator>, new()
{
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph,
                               CliqueConsumer<TAccumulator> consumer)
    {
        VertexSet candidates = VertexSetMgr.From(graph.ConnectedVertices());
        if (candidates.Any())
        {
            Pivot<VertexSet, VertexSetMgr, TAccumulator>.Visit(
                graph,
                consumer,
                PivotChoice.MaxDegreeLocal,
                candidates,
                VertexSetMgr.Empty(),
                []);
        }
    }
}
