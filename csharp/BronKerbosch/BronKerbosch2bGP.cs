// Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

using BronKerbosch;
using System.Collections.Generic;

internal static class BronKerbosch2bGP<VertexSet, VertexSetMgr>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
#   pragma warning disable IDE0022 // Use expression body for method
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, ICliqueConsumer consumer)
    {
        Pivot<VertexSet, VertexSetMgr>.Explore(graph, consumer, PivotChoice.MaxDegreeLocal);
    }
}
