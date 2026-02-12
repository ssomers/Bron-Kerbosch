using System.Collections.Immutable;
using System.Threading;

namespace BronKerbosch
{
    public sealed class CliqueCounter : ICliqueConsumer
    {
        private int cliques;

        public void Accept(ImmutableArray<Vertex> clique) => Interlocked.Increment(ref cliques);

        public int Count() => cliques;
    }
}
