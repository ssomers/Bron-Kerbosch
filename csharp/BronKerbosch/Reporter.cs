using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using Vertex = System.UInt32;

namespace BronKerbosch
{
    public interface Reporter
    {
        void Record(ImmutableArray<Vertex> clique);
    }

    public sealed class SimpleReporter : Reporter
    {
        public List<ImmutableArray<Vertex>> Cliques { get; } = new List<ImmutableArray<Vertex>>();

        public void Record(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(clique.Length > 1);
            Cliques.Add(clique);
        }
    }

    public sealed class CountingReporter : Reporter
    {
        public int Cliques { get; private set; } = 0;

        public void Record(ImmutableArray<Vertex> clique)
        {
            Cliques += 1;
        }
    }
}
