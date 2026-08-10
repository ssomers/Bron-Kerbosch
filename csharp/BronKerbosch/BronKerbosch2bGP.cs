// Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

using BronKerbosch;
using System.Collections.Generic;

internal static class BronKerbosch2bGP<VertexSet, VertexSetMgr, TAccumulator>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
    where TAccumulator : ICliqueAccumulator<TAccumulator>, new()
{
#   pragma warning disable IDE0022 // Use expression body for method
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph,
                               CliqueConsumer<TAccumulator> consumer)
    {
        Pivot<VertexSet, VertexSetMgr, TAccumulator>.Explore(graph, consumer,
                                                             PivotChoice.MaxDegreeLocal);
    }
}
