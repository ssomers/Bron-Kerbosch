// Bron-Kerbosch algorithm with degeneracy ordering,
// with nested searches choosing a pivot from candidates only (IK_GP).

using BronKerbosch;

internal static class BronKerbosch3GP
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
    {
#       pragma warning disable IDE0022 // Use expression body for methods
        BronKerboschDegeneracy.Explore(graph, reporter, Pivot.Choice.MaxDegreeLocal);
    }
}
