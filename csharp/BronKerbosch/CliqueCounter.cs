using System.Collections.Immutable;
using System.Diagnostics;

namespace BronKerbosch
{
    public sealed class CliqueCounter(int min_size) : ICliqueConsumer
    {
        public int Cliques { get; private set; }

        public bool IsAcceptedSize(int size) => size >= min_size;
        public void Accept(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(clique.Length >= min_size);
            Cliques += 1;
        }
        public ICliqueConsumer StartNew() => new CliqueCounter(min_size);
        public void Absorb(ICliqueConsumer other) => Cliques += ((CliqueCounter)other).Cliques;
    }
}
