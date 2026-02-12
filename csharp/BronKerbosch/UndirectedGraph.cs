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

        public bool AreSymmetrical(ImmutableArray<TVertexSet> adjacencies) => adjacencies
            .Select((neighbours, i) => (Vertex.Nth(i), neighbours))
            .All(((Vertex vertex, TVertexSet neighbours) the) =>
                 the.neighbours.All(w => adjacencies[w.Index()].Contains(the.vertex)));

        public bool AreLoopFree(ImmutableArray<TVertexSet> adjacencies) => adjacencies
            .Select((neighbours, i) => (Vertex.Nth(i), neighbours))
            .All(((Vertex vertex, TVertexSet neighbours) the) =>
                 !the.neighbours.Contains(the.vertex));

        public UndirectedGraph(ImmutableArray<TVertexSet> adjacencies)
        {
            Debug.Assert(AreSymmetrical(adjacencies));
            Debug.Assert(AreLoopFree(adjacencies));
            itsAdjacencies = adjacencies;
        }

        public int Order => itsAdjacencies.Length;

        public int Size
        {
            get
            {
                var total = Vertices().Sum(Degree);
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
