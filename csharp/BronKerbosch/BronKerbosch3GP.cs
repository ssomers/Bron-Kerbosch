// Bron-Kerbosch algorithm with degeneracy ordering,
// with nested searches choosing a pivot from candidates only (IK_GP).

using BronKerbosch;
using System.Collections.Generic;

internal static class BronKerbosch3GP<VertexSet, VertexSetMgr>
    where VertexSet : IEnumerable<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, IReporter reporter)
    {
#       pragma warning disable IDE0022 // Use expression body for methods
        BronKerboschDegeneracy<VertexSet, VertexSetMgr>.Explore(graph, reporter, PivotChoice.MaxDegreeLocal);
    }
}
