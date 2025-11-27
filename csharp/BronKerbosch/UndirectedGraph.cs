using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    public sealed class UndirectedGraph<TVertexSet, TVertexSetMgr>
        where TVertexSet : ISet<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        private readonly ImmutableArray<TVertexSet> itsAdjacencies;

        public bool AreValidAdjacencies(ImmutableArray<TVertexSet> adjacencies)
        {
            foreach (var v in Enumerable.Range(0, adjacencies.Length).Select(Vertex.Nth))
            {
                foreach (var w in adjacencies[v.Index()])
                {
                    if (v == w) return false;
                    if (!adjacencies[w.Index()].Contains(v)) return false;
                }
            }
            return true;
        }

        public UndirectedGraph(ImmutableArray<TVertexSet> adjacencies)
        {
            Debug.Assert(AreValidAdjacencies(adjacencies));
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

        public TVertexSet Neighbours(Vertex node) => itsAdjacencies[node.Index()];

        public bool HasNeighbours(Vertex node) => itsAdjacencies[node.Index()].Count > 0;

        public int Degree(Vertex node) => itsAdjacencies[node.Index()].Count;

        public IEnumerable<Vertex> Vertices() => Enumerable.Range(0, Order).Select(Vertex.Nth);

        public IEnumerable<Vertex> ConnectedVertices() => Vertices().Where(HasNeighbours);

        public Vertex MaxDegreeVertex() => Vertices().MaxBy(Degree);
    }
}
