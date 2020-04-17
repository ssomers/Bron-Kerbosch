using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using Vertex = System.UInt32;

namespace BronKerbosch
{
    public sealed class UndirectedGraph
    {
        private readonly ImmutableArray<HashSet<Vertex>> itsAdjacencies;

        public UndirectedGraph(ImmutableArray<HashSet<Vertex>> adjacencies)
        {
            for (Vertex v = 0; v < adjacencies.Length; ++v)
            {
                foreach (var w in adjacencies[(int) v])
                {
                    Debug.Assert(v != w);
                    Debug.Assert(adjacencies[(int) w].Contains(v));
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
                for (Vertex v = 0; v < Order; ++v)
                    total += Degree(v);
                Debug.Assert(total % 2 == 0);
                return total / 2;
            }
        }

        public HashSet<Vertex> Neighbours(Vertex node) => itsAdjacencies[(int) node]; // .AsReadOnly()

        public int Degree(Vertex node) => itsAdjacencies[(int) node].Count;

        public IEnumerable<Vertex> ConnectedVertices()
        {
            for (Vertex v = 0; v < Order; ++v)
                if (Degree(v) > 0)
                    yield return v;
        }
    }
}
