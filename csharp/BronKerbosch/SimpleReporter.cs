using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;

namespace BronKerbosch
{
    public sealed class SimpleReporter : IReporter
    {
        public List<ImmutableArray<Vertex>> Cliques { get; } = new List<ImmutableArray<Vertex>>();

        public void Record(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(clique.Length > 1);
            Cliques.Add(clique);
        }
    }
}
