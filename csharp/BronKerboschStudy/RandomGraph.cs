using BronKerbosch;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace BronKerboschStudy
{
    public sealed class RandomUndirectedGraph
    {
        public static UndirectedGraph Generate(Random random, int order, int size)
        {
            var fully_meshed_size = order * (order - 1) / 2;
            if (size > fully_meshed_size)
                throw new ArgumentException($"{order} nodes accommodate at most {fully_meshed_size} edges", "size");
            List<Vertex> unsaturated_vertices = Enumerable.Range(0, order).Select(index => new Vertex(index)).ToList();
            List<HashSet<Vertex>> adjacency_sets = Enumerable.Range(0, order).Select(_ => new HashSet<Vertex>()).ToList();
            List<HashSet<Vertex>> adjacency_complements = Enumerable.Range(0, order).Select(_ => new HashSet<Vertex>()).ToList();
            for (int i = 0; i < size; ++i)
            {
                Vertex v = unsaturated_vertices[random.Next(unsaturated_vertices.Count)];
                Debug.Assert(adjacency_sets[v].Count < order - 1);
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
                Debug.Assert(v != w);
                Debug.Assert(!adjacency_sets[v].Contains(w));
                Debug.Assert(!adjacency_sets[w].Contains(v));
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
                        Debug.Assert(adjacency_complements[x].Count == 0);
                        adjacency_complements[x] = unsaturated_vertices.ToHashSet()
                            .Except(new[] { x }).ToHashSet()
                            .Except(adjacency_sets[x]).ToHashSet();
                    }
                    else if (neighbours > order / 2)
                    {
                        adjacency_complements[x].Remove(y);
                    }
                }
            }
            var g = new UndirectedGraph(adjacency_sets);
            Debug.Assert(g.Order == order);
            Debug.Assert(g.Size == size);
            return g;
        }
    }
}
