// Bron-Kerbosch algorithm with degeneracy ordering,
// with nested searches choosing a pivot from candidates only (IK_GP),
// implemented by multiple threads.

using BronKerbosch;
using System.Collections.Generic;

internal static class BronKerbosch3MT<VertexSet, VertexSetMgr>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
#   pragma warning disable IDE0022 // Use expression body for method
    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph,
                               ICliqueConsumer consumer, int maxDegreeOfParallelism)
    {
        DegeneracyBasedMT<VertexSet, VertexSetMgr>.Explore(graph, consumer,
                                                           PivotChoice.MaxDegreeLocal,
                                                           maxDegreeOfParallelism);
    }
}
