// Bron-Kerbosch algorithm with degeneracy ordering,
// choosing a pivot from both candidates and excluded vertices (IK_GPX).

using BronKerbosch;

internal static class BronKerbosch3GPX
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
    {
        BronKerboschDegeneracy.Explore(graph, reporter, Pivot.Choice.MaxDegreeLocalX);
    }
}
