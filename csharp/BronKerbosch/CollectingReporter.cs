using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;

namespace BronKerbosch
{
    public sealed class CollectingReporter : IReporter
    {
        public List<ImmutableArray<Vertex>> Cliques { get; } = [];

        public void Record(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(clique.Length > 1);
            Cliques.Add(clique);
        }
    }
}
