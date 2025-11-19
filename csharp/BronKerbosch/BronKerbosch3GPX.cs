// Bron-Kerbosch algorithm with degeneracy ordering,
// choosing a pivot from both candidates and excluded vertices (IK_GPX).

using BronKerbosch;
using System.Collections.Generic;

internal static class BronKerbosch3GPX<VertexSet, VertexSetMgr>
    where VertexSet : IEnumerable<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
#   pragma warning disable IDE0022 // Use expression body for method
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, IReporter reporter)
    {
        BronKerboschDegeneracy<VertexSet, VertexSetMgr>.Explore(graph, reporter, PivotChoice.MaxDegreeLocalX);
    }
}
