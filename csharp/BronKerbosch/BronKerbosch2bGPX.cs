// Bron-Kerbosch algorithm with pivot of highest degree towards the remaining candidates (IK_GPX)

using BronKerbosch;

internal static class BronKerbosch2bGPX
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
    {
#       pragma warning disable IDE0022 // Use expression body for methods
        Pivot.Explore(graph, reporter, Pivot.Choice.MaxDegreeLocalX);
    }
}
