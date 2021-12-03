using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;

namespace BronKerbosch
{
    public sealed class UndirectedGraph
    {
        private readonly ImmutableArray<HashSet<Vertex>> itsAdjacencies;

        public UndirectedGraph(ImmutableArray<HashSet<Vertex>> adjacencies)
        {
            for (int i = 0; i < adjacencies.Length; ++i)
            {
                var v = Vertex.nth(i);
                foreach (var w in adjacencies[v.index])
                {
                    Debug.Assert(v != w);
                    Debug.Assert(adjacencies[w.index].Contains(v));
                }
            }
            itsAdjacencies = adjacencies;
        }

        public int Order => itsAdjacencies.Length;

        public int Size
        {
            get
            {
                var total = 0;
                for (int i = 0; i < Order; ++i)
                    total += Degree(Vertex.nth(i));
                Debug.Assert(total % 2 == 0);
                return total / 2;
            }
        }

        public HashSet<Vertex> Neighbours(Vertex node) => itsAdjacencies[node.index];

        public int Degree(Vertex node) => itsAdjacencies[node.index].Count;

        public IEnumerable<Vertex> ConnectedVertices()
        {
            for (int i = 0; i < Order; ++i)
            {
                var v = Vertex.nth(i);
                if (Degree(v) > 0)
                    yield return v;
            }
        }
    }
}
