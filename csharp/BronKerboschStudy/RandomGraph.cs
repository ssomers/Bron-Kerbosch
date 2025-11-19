using BronKerbosch;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.IO;
using System.Linq;

namespace BronKerboschStudy
{
    public sealed class RandomUndirectedGraph<TVertexSet, TVertexSetMgr>
        where TVertexSet : IEnumerable<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        public UndirectedGraph<TVertexSet, TVertexSetMgr> Graph { get; }
        public int CliqueCount { get; }

        private RandomUndirectedGraph(UndirectedGraph<TVertexSet, TVertexSetMgr> graph, int cliqueCount)
        {
            Graph = graph;
            CliqueCount = cliqueCount;
        }

        public static RandomUndirectedGraph<TVertexSet, TVertexSetMgr> Read(string orderstr, int size)
        {
            var order = NumbersGame.ParseInt(orderstr);
            var fullyMeshedSize = (long)order * (order - 1) / 2;
            if (size > fullyMeshedSize)
                throw new ArgumentException($"{order} nodes accommodate at most {fullyMeshedSize} edges");

            var edgesPath = $"..\\data\\random_edges_order_{orderstr}.txt";
            const string statsPath = "..\\data\\random_stats.txt";
            var adjacencies = ReadEdges(edgesPath, orderstr, size);
            var cliqueCount = ReadStats(statsPath, orderstr, size);
            var g = new UndirectedGraph<TVertexSet, TVertexSetMgr>(adjacencies);
            Debug.Assert(g.Order == order);
            Debug.Assert(g.Size == size);
            return new RandomUndirectedGraph<TVertexSet, TVertexSetMgr>(g, cliqueCount);
        }

        private static ImmutableArray<TVertexSet> ReadEdges(string path, string orderstr, int size)
        {
            var order = NumbersGame.ParseInt(orderstr);
            var adjacencies = Enumerable.Range(0, order)
                .Select(_ => TVertexSetMgr.Empty())
                .ToImmutableArray();
            var linenum = 0;
            using (var file = new StreamReader(path))
            {
                string? line;
                while (linenum < size && (line = file.ReadLine()) != null)
                {
                    ++linenum;
                    var fields = line.Split(' ');
                    if (!int.TryParse(fields[0], out var v) ||
                        !int.TryParse(fields[1], out var w))
                    {
                        throw new ArgumentException($"File {path} line {linenum} contains bogus text {line}");
                    }
                    var added1 = TVertexSetMgr.Add(adjacencies[v], Vertex.Nth(w));
                    var added2 = TVertexSetMgr.Add(adjacencies[w], Vertex.Nth(v));
                    Debug.Assert(added1);
                    Debug.Assert(added2);
                }
            }
            if (linenum < size)
            {
                throw new ArgumentException($"Exhausted generated list of {linenum} edges in {path}");
            }
            return adjacencies;
        }

        private static int ReadStats(string path, string orderstr, int size)
        {
            var prefix = $"{orderstr}\t{size}\t";
            using var file = new StreamReader(path);
            var header = file.ReadLine();
            string? line;
            while ((line = file.ReadLine()) != null)
            {
                if (line.StartsWith(prefix, StringComparison.Ordinal))
                {
                    if (!int.TryParse(line.AsSpan(prefix.Length), out var c))
                    {
                        throw new ArgumentException($"File {path} has bogus line “{line}”");
                    }
                    return c;
                }
            }

            throw new ArgumentException($"File {path} lacks order {orderstr} size {size}");
        }
    }
}
