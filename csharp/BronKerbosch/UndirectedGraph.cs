using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    public sealed class UndirectedGraph
    {
        private readonly ImmutableArray<HashSet<Vertex>> itsAdjacencies;

        public UndirectedGraph(ImmutableArray<HashSet<Vertex>> adjacencies)
        {
            for (int i = 0; i < adjacencies.Length; ++i)
            {
                var v = Vertex.Nth(i);
                foreach (var w in adjacencies[v.Index()])
                {
                    Debug.Assert(v != w);
                    Debug.Assert(adjacencies[w.Index()].Contains(v));
                }
            }
            itsAdjacencies = adjacencies;
        }

        public int Order => itsAdjacencies.Length;

        public int Size
        {
            get
            {
                var total = Enumerable.Range(0, Order).Select(Vertex.Nth).Sum(Degree);
                Debug.Assert(total % 2 == 0);
                return total / 2;
            }
        }

        public HashSet<Vertex> Neighbours(Vertex node) => itsAdjacencies[node.Index()];

        public int Degree(Vertex node) => itsAdjacencies[node.Index()].Count;

        public IEnumerable<Vertex> Vertices() => Enumerable.Range(0, Order).Select(Vertex.Nth);

        public IEnumerable<Vertex> ConnectedVertices() => Vertices().Where(v => Degree(v) > 0);

        public Vertex MaxDegreeVertex() => Vertices().MaxBy(Degree);
    }
}
