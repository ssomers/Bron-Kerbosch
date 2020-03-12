using System;
using System.Collections.Generic;
using System.Diagnostics;
using Vertex = System.UInt32;

namespace BronKerbosch
{
    public class UndirectedGraph
    {
        private readonly List<HashSet<Vertex>> itsAdjacencies;

        public UndirectedGraph(List<HashSet<Vertex>> adjacencies)
        {
            for (Vertex v = 0; v < adjacencies.Count; ++v)
            {
                foreach (Vertex w in adjacencies[(int)v])
                {
                    Debug.Assert(v != w);
                    Debug.Assert(adjacencies[(int)w].Contains(v));
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
                for (Vertex v = 0; v < Order; ++v)
                    total += Degree(v);
                Debug.Assert(total % 2 == 0);
                return total / 2;
            }
        }

        public HashSet<Vertex> Neighbours(Vertex node)
        {
            return itsAdjacencies[(int)node]; // .AsReadOnly()
        }

        public int Degree(Vertex node)
        {
            return itsAdjacencies[(int)node].Count;
        }

        public HashSet<Vertex> ConnectedVertices()
        {
            var result = new HashSet<Vertex>();
            for (Vertex v = 0; v < Order; ++v)
                if (Degree(v) > 0)
                    result.Add(v);
            return result;
        }
    }
}
