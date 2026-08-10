using System.Collections.Immutable;

namespace BronKerbosch
{
    public sealed class CliqueCounter : ICliqueAccumulator<CliqueCounter>
    {
        public int Cliques { get; private set; }

        public void Add(ImmutableArray<Vertex> clique) => Cliques += 1;
        public void Absorb(CliqueCounter spawned) => Cliques += spawned.Cliques;
    }
}
