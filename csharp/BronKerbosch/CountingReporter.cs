using System.Collections.Immutable;

namespace BronKerbosch
{
    public sealed class CountingReporter : IReporter
    {
        public int Cliques { get; private set; }

        public void Record(ImmutableArray<Vertex> clique)
        {
            Cliques += 1;
        }
    }
}
