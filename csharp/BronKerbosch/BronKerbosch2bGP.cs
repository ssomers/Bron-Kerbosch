// Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)

using BronKerbosch;

internal static class BronKerbosch2bGP
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
    {
        Pivot.Explore(graph, reporter, Pivot.Choice.MaxDegreeLocal);
    }
}