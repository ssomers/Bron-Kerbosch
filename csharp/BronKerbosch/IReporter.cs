using System.Collections;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using Vertex = System.UInt32;

namespace BronKerbosch
{
    public interface IReporter
    {
        void Record(ImmutableArray<Vertex> clique);
    }

    public sealed class SimpleReporter : IReporter
    {
        public List<ImmutableArray<Vertex>> Cliques { get; } = new List<ImmutableArray<Vertex>>();

        public void Record(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(clique.Length > 1);
            lock (this)
            {
                Cliques.Add(clique);
            }
        }
    }

    public sealed class CountingReporter : IReporter
    {
        private int count;
        public int Cliques { get => count; }

        public void Record(ImmutableArray<Vertex> clique)
        {
            System.Threading.Interlocked.Increment(ref count);
        }
    }
}
