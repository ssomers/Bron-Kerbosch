using System.Collections.Generic;
using System.Collections.Immutable;

namespace BronKerbosch
{
    public sealed class CliqueCollector : ICliqueAccumulator<CliqueCollector>
    {
        public List<ImmutableArray<Vertex>> Cliques { get; private set; } = [];

        public void Add(ImmutableArray<Vertex> clique) => Cliques.Add(clique);
        public void Absorb(CliqueCollector spawned) => Cliques.AddRange(spawned.Cliques);
    }
}
