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
        void Close();
    }

    public sealed class SimpleReporter : IReporter
    {
        public List<ImmutableArray<Vertex>> Cliques { get; } = new List<ImmutableArray<Vertex>>();
        private bool closed = false;

        public void Record(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(clique.Length > 1);
            if (closed) throw new System.Exception("Record after CLose");
            lock (this)
            {
                Cliques.Add(clique);
            }
        }

        public void Close()
        {
            if (closed) throw new System.Exception("CLose after CLose");
            closed = true;
        }
    }

    public sealed class CountingReporter : IReporter
    {
        private int count;
        private bool closed;
        public int Cliques { get => count; }

        public void Record(ImmutableArray<Vertex> clique)
        {
            if (closed) throw new System.Exception("CLose after CLose");
            System.Threading.Interlocked.Increment(ref count);
        }

        public void Close()
        {
            if (closed) throw new System.Exception("CLose after CLose");
            closed = true;
        }
    }
}
