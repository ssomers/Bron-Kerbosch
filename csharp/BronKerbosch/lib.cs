using System;
using System.Collections.Generic;

namespace BronKerbosch
{
    public class Portfolio
    {
        public const int NUM_FUNCS = 2;

        static public void Explore(int func_index, UndirectedGraph graph, Reporter reporter)
        {
            switch (func_index)
            {
                case 0: BronKerbosch1.Explore(graph, reporter); break;
                case 1: BronKerbosch2.Explore(graph, reporter); break;
                default: throw new ArgumentException("unknown func_index");
            }
        }

        static public void SortCliques(List<List<Vertex>> cliques)
        {
            foreach (List<Vertex> clique in cliques)
                clique.Sort();
            cliques.Sort(comparer);
        }

        static int comparer(List<Vertex> lhs, List<Vertex> rhs)
        {
            if (Object.Equals(lhs, rhs))
            {   // Seriously, Sort sometimes compares an element with itself
                return 0;
            }
            for (var i = 0; i < lhs.Count && i < rhs.Count; i++)
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
