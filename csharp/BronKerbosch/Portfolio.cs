using System;
using System.Collections.Generic;
using System.Collections.Immutable;

namespace BronKerbosch
{
    public static class Portfolio
    {
        public static readonly string[] FuncNames =
        [
            "Ver1�",
            "Ver2-GP", "Ver2�-GP", "Ver2�-GPX",
            "Ver3�-GP", "Ver3�-GPX",
            "Ver3�=GPc"
        ];

        public static void Explore<TVertexSet, TVertexSetMgr>(int funcIndex, UndirectedGraph<TVertexSet, TVertexSetMgr> graph, IReporter reporter)
            where TVertexSet : IEnumerable<Vertex>
            where TVertexSetMgr : IVertexSetMgr<TVertexSet>
        {
            switch (funcIndex)
            {
                case 0: BronKerbosch1<TVertexSet, TVertexSetMgr>.Explore(graph, reporter); break;
                case 1: BronKerbosch2aGP<TVertexSet, TVertexSetMgr>.Explore(graph, reporter); break;
                case 2: BronKerbosch2bGP<TVertexSet, TVertexSetMgr>.Explore(graph, reporter); break;
                case 3: BronKerbosch2bGPX<TVertexSet, TVertexSetMgr>.Explore(graph, reporter); break;
                case 4: BronKerbosch3GP<TVertexSet, TVertexSetMgr>.Explore(graph, reporter); break;
                case 5: BronKerbosch3GPX<TVertexSet, TVertexSetMgr>.Explore(graph, reporter); break;
                case 6: BronKerbosch3MT<TVertexSet, TVertexSetMgr>.Explore(graph, reporter); break;
                default: throw new ArgumentException("unknown func_index");
            }
        }

        public static void SortCliques(List<ImmutableArray<Vertex>> cliques)
        {
            for (var i = 0; i < cliques.Count; ++i)
            {
                cliques[i] = cliques[i].Sort();
            }
            cliques.Sort(Comparer);
        }

        public static void AssertSameCliques(List<ImmutableArray<Vertex>> lhs, List<ImmutableArray<Vertex>> rhs)
        {
            if (lhs.Count != rhs.Count)
            {
                throw new ArgumentException($"{lhs.Count} cliques <> {rhs.Count} cliques");
            }
            for (var i = 0; i < lhs.Count; ++i)
            {
                if (lhs[i].Length != rhs[i].Length)
                {
                    throw new ArgumentException($"clique #{i + 1}: length {lhs[i].Length} <> length {rhs[i].Length}");
                }
                for (var j = 0; j < lhs[i].Length; ++j)
                {
                    if (lhs[i][j] != rhs[i][j])
                    {
                        throw new ArgumentException($"clique #{i + 1}, vertex #{j + 1}: {lhs[i][j]} <> {rhs[i][j]}");
                    }
                }
            }
        }

        private static int Comparer(ImmutableArray<Vertex> lhs, ImmutableArray<Vertex> rhs)
        {
            if (Equals(lhs, rhs))
            {
                // Seriously, Sort sometimes compares an element with itself
                return 0;
            }

            for (var i = 0; i < lhs.Length && i < rhs.Length; ++i)
            {
                var d = lhs[i].CompareTo(rhs[i]);
                if (d != 0)
                {
                    return d;
                }
            }
            throw new ArgumentException(
                $"got overlapping or equal cliques (length {lhs.Length} <> length {rhs.Length})");
        }
    }
}
