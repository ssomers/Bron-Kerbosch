using System;
using System.Collections.Generic;
using System.Diagnostics;

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
            for (var v = 0; v < adjacencies.Count; ++v)
            {
                foreach (Vertex w in adjacencies[v])
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
                var total = 0;
                for (var v = 0; v < Order; ++v)
                    total += Degree(v);
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
            var result = new HashSet<Vertex>();
            for (var v = 0; v < Order; ++v)
                if (Degree(v) > 0)
                    result.Add(v);
            return result;
        }
    }
}
