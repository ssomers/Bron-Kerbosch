using System;
using System.Collections.Generic;

namespace BronKerbosch
{
    public class Portfolio
    {
        public const int NUM_FUNCS = 2;

        public static void Explore(int func_index, UndirectedGraph graph, Reporter reporter)
        {
            switch (func_index)
            {
                case 0: BronKerbosch1.Explore(graph, reporter); break;
                case 1: BronKerbosch2.Explore(graph, reporter); break;
                default: throw new ArgumentException("unknown func_index");
            }
        }

        public static void SortCliques(List<List<Vertex>> cliques)
        {
            foreach (List<Vertex> clique in cliques)
                clique.Sort();
            cliques.Sort(comparer);
        }

        public static void AssertSameCliques(List<List<Vertex>> lhs, List<List<Vertex>> rhs)
        {
            if (lhs.Count != rhs.Count)
                throw new Exception($"{lhs.Count} cliques <> {rhs.Count} cliques");
            for (var i = 0; i < lhs.Count; ++i)
            {
                if (lhs[i].Count != rhs[i].Count)
                    throw new Exception($"clique #{i + 1}: length {lhs[i].Count} <> length {rhs[i].Count}");
                for (var j = 0; j < lhs[i].Count; ++j)
                {
                    if (lhs[i][j] != rhs[i][j])
                        throw new Exception($"clique #{i + 1}, vertex #{j + 1}: {lhs[i][j]} <> length {rhs[i][j]}");
                }
            }
        }

        static int comparer(List<Vertex> lhs, List<Vertex> rhs)
        {
            if (Object.Equals(lhs, rhs))
            {   // Seriously, Sort sometimes compares an element with itself
                return 0;
            }
            for (var i = 0; i < lhs.Count && i < rhs.Count; ++i)
            {
                var d = lhs[i] - rhs[i];
                if (d != 0)
                {
                    return d;
                }
            }
            throw new ArgumentException($"got overlapping or equal cliques (length {lhs.Count} <> length {rhs.Count})");
        }
    }
}
