using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;

namespace BronKerbosch
{
    public sealed class CliqueCollector(int min_size) : ICliqueConsumer
    {
        public List<ImmutableArray<Vertex>> Cliques { get; } = [];

        public bool IsAcceptedSize(int size) => size >= min_size;
        public void Accept(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(clique.Length >= min_size);
            Cliques.Add(clique);
        }
        public ICliqueConsumer StartNew() => new CliqueCollector(min_size);
        public void Absorb(ICliqueConsumer other) => Cliques.AddRange(((CliqueCollector)other).Cliques);
    }
}
