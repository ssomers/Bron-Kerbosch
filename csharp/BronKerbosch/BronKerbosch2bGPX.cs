// Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

using BronKerbosch;
using System.Collections.Generic;

internal static class BronKerbosch2bGPX<VertexSet, VertexSetMgr>
    where VertexSet : IEnumerable<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
#   pragma warning disable IDE0022 // Use expression body for method
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, IReporter reporter)
    {
        Pivot<VertexSet, VertexSetMgr>.Explore(graph, reporter, PivotChoice.MaxDegreeLocalX);
    }
}
