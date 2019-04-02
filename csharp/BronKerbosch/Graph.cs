using System;
using System.Collections.Generic;
using System.Diagnostics.Contracts;
using System.Linq;

namespace Graph
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

        [Pure]
        public int Size()
        {
            var total = (from adjacent in itsAdjacencies select adjacent.Count).Sum();
            Contract.Assume(total % 2 == 0);
            return total / 2;
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

        public static UndirectedGraph GenerateRandom(int order, int size)
        {
            var random = new Random();
            var fully_meshed_size = order * (order - 1) / 2;
            Contract.Requires(size <= fully_meshed_size);
            List<Vertex> unsaturated_vertices = Enumerable.Range(0, order).Select(index => new Vertex(index)).ToList();
            List<HashSet<Vertex>> adjacency_sets = Enumerable.Range(0, order).Select(_ => new HashSet<Vertex>()).ToList();
            List<HashSet<Vertex>> adjacency_complements = Enumerable.Range(0, order).Select(_ => new HashSet<Vertex>()).ToList(); 
            for (int i = 0; i < size; ++i)
            {
                Vertex v = unsaturated_vertices[random.Next(unsaturated_vertices.Count)];
                Contract.Assume(adjacency_sets[v].Count < order - 1);
                Vertex w;
                if (adjacency_complements[v].Any())
                {
                    var next = random.Next(adjacency_complements[v].Count);
                    w = adjacency_complements[v].Skip(next).Take(1).First();
                }
                else
                {
                    w = v;
                    while (w == v || adjacency_sets[v].Contains(w))
                        w = unsaturated_vertices[random.Next(unsaturated_vertices.Count)];
                }
                Contract.Assume(v != w);
                Contract.Assume(!adjacency_sets[v].Contains(w));
                Contract.Assume(!adjacency_sets[w].Contains(v));
                foreach ((Vertex, Vertex) p in new[] { (v, w), (w, v) })
                {
                    var x = p.Item1;
                    var y = p.Item2;
                    adjacency_sets[x].Add(y);
                    var neighbours = adjacency_sets[x].Count;
                    if (neighbours == order - 1)
                    {
                        unsaturated_vertices.Remove(x);
                    }
                    else if (neighbours == order / 2)
                    {
                        // start using adjacency complement
                        Contract.Assume(adjacency_complements[x].Count == 0);
                        adjacency_complements[x] = unsaturated_vertices.ToHashSet()
                            .Except(new[] { x }.ToHashSet()).ToHashSet()
                            .Except(adjacency_sets[x]).ToHashSet();
                    }
                    else if (neighbours > order / 2)
                    {
                        adjacency_complements[x].Remove(y);
                    }
                }
            }
            var g = new UndirectedGraph(adjacency_sets);
                    Contract.Requires(g.Order == order);
                    Contract.Requires(g.Size() == size);
                    return g;
        }
    }
}
