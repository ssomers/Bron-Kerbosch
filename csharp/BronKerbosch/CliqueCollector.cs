using System.Collections.Generic;
using System.Collections.Immutable;
using System.Collections.Concurrent;
using System.Diagnostics;

namespace BronKerbosch
{
    public sealed class CliqueCollector : ICliqueConsumer
    {
        private ConcurrentBag<ImmutableArray<Vertex>> Cliques { get; } = [];

        public void Accept(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(clique.Length > 1);
            Cliques.Add(clique);
        }

        public List<ImmutableArray<Vertex>> List() => [.. Cliques];
    }
}
