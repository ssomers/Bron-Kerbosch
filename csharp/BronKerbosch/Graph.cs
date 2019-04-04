using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    [DebuggerDisplay("{Index}")]
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
                    Debug.Assert(v != w);
                    Debug.Assert(adjacencies[w].Contains(v));
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
            get
            {
                var total = (from adjacent in itsAdjacencies select adjacent.Count).Sum();
                Debug.Assert(total % 2 == 0);
                return total / 2;
            }
        }

        public HashSet<Vertex> Neighbours(Vertex node)
        {
            return itsAdjacencies[node]; // .AsReadOnly()
        }

        public int Degree(Vertex node)
        {
            return itsAdjacencies[node].Count;
        }

        public HashSet<Vertex> ConnectedVertices()
        {
            return itsAdjacencies.Select((neighbours, i) => (new Vertex(i), neighbours.Any())).Where(p => p.Item2).Select(p => p.Item1).ToHashSet();
        }
    }
}
