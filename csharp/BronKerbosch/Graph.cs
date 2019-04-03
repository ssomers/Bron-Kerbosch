using System;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;

namespace BronKerbosch
{
    public struct Vertex : IComparable<Vertex>
    {
        private readonly int Index;

        public Vertex(int id)
        {
            Index = id;
        }

        public int CompareTo(Vertex rhs)
        {
            return Index - rhs.Index;
        }

        public static implicit operator Vertex(int index)
        {
            return new Vertex(index);
        }

        public static implicit operator int(Vertex vtx)
        {
            return vtx.Index;
        }
    }

    public sealed class UndirectedGraph
    {
        private readonly List<HashSet<Vertex>> itsAdjacencies;

        public UndirectedGraph(List<HashSet<Vertex>> adjacencies)
        {
            foreach ((HashSet<Vertex> adjacent_to_v, Vertex v) in adjacencies.Select((s, i) => (s, new Vertex(i))))
            {
                foreach (Vertex w in adjacent_to_v)
                {
                    Contract.Requires(v != w);
                    Contract.Requires(adjacencies[w].Contains(v));
                }
            }
            itsAdjacencies = adjacencies;
        }

        public int Order
        {
            get => itsAdjacencies.Count;
        }

        public int Size
        {
            get {
                var total = (from adjacent in itsAdjacencies select adjacent.Count).Sum();
                Contract.Assume(total % 2 == 0);
                return total / 2;
            }
        }

        [Pure]
        public HashSet<Vertex> Neighbours(Vertex node)
        {
            return itsAdjacencies[node]; // .AsReadOnly()
        }

        [Pure]
        public int Degree(Vertex node)
        {
            return itsAdjacencies[node].Count;
        }

        [Pure]
        public HashSet<Vertex> ConnectedVertices()
        {
            return itsAdjacencies.Select((neighbours, i) => (neighbours.Any(), i)).Where(p => p.Item1).Select(p => new Vertex(p.Item2)).ToHashSet();
        }
    }
}
