using System;
using System.Collections.Generic;
using Vertex = System.UInt32;

namespace BronKerbosch
{
    public class Portfolio
    {
        public static readonly string[] FUNC_NAMES = new string[] { "Ver1+", "Ver2+G", "Ver2+GP", "Ver2+GPX", "Ver3+GP", "Ver3+GPX" };

        public static void Explore(int func_index, UndirectedGraph graph, Reporter reporter)
        {
            switch (func_index)
            {
                case 0: BronKerbosch1.Explore(graph, reporter); break;
                case 1: BronKerbosch2G.Explore(graph, reporter); break;
                case 2: BronKerbosch2GP.Explore(graph, reporter); break;
                case 3: BronKerbosch2GPX.Explore(graph, reporter); break;
                case 4: BronKerbosch3GP.Explore(graph, reporter); break;
                case 5: BronKerbosch3GPX.Explore(graph, reporter); break;
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
                var d = (int)lhs[i] - (int)rhs[i];
                if (d != 0)
                {
                    return d;
                }
            }
            throw new ArgumentException($"got overlapping or equal cliques (length {lhs.Count} <> length {rhs.Count})");
        }
    }
}
