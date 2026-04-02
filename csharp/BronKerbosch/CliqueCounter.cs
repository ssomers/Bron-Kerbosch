using System.Collections.Immutable;
using System.Diagnostics;
using System.Threading;

namespace BronKerbosch
{
    public sealed class CliqueCounter(int min_size) : ICliqueConsumer
    {
        private int cliques;

        public bool IsAcceptedSize(int size) => size >= min_size;
        public void Accept(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(clique.Length >= min_size);
            _ = Interlocked.Increment(ref cliques);
        }

        public int Count() => cliques;
    }
}
