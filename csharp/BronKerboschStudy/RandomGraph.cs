using BronKerbosch;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.IO;
using System.Linq;
using Vertex = System.UInt32;

namespace BronKerboschStudy
{
    public sealed class RandomUndirectedGraph
    {
        public UndirectedGraph Graph { get; }
        public int CliqueCount { get; }

        private RandomUndirectedGraph(UndirectedGraph graph, int cliqueCount)
        {
            Graph = graph;
            CliqueCount = cliqueCount;
        }

        public static int ParsePositiveInt(string orderstr)
        {
            if (orderstr.EndsWith("M"))
                return Int32.Parse(orderstr.Remove(orderstr.Length - 1)) * 1_000_000;
            else if (orderstr.EndsWith("k"))
                return Int32.Parse(orderstr.Remove(orderstr.Length - 1)) * 1_000;
            else
                return Int32.Parse(orderstr);
        }

        public static RandomUndirectedGraph Read(string orderstr, int size)
        {
            int order = ParsePositiveInt(orderstr);
            long fully_meshed_size = (long)order * (order - 1) / 2;
            if (size > fully_meshed_size)
                throw new ArgumentException($"{order} nodes accommodate at most {fully_meshed_size} edges");

            var edges_path = @"..\random_edges_order_" + orderstr + ".txt";
            var stats_path = @"..\random_stats.txt";
            var adjacencies = ReadEdges(edges_path, orderstr, size);
            var clique_count = ReadStats(stats_path, orderstr, size);
            var g = new UndirectedGraph(adjacencies);
            Debug.Assert(g.Order == order);
            Debug.Assert(g.Size == size);
            return new RandomUndirectedGraph(g, clique_count);
        }

        private static ImmutableArray<HashSet<Vertex>> ReadEdges(string path, string orderstr, int size)
        {
            int order = ParsePositiveInt(orderstr);
            var adjacencies = Enumerable.Range(0, order)
                .Select(_ => new HashSet<Vertex>())
                .ToImmutableArray();
            int linenum = 0;
            using (var file = new StreamReader(path))
            {
                string? line;
                while (linenum < size && (line = file!.ReadLine()) != null)
                {
                    ++linenum;
                    var fields = line.Split(' ');
                    int v, w;
                    if (!int.TryParse(fields[0], out v) ||
                        !int.TryParse(fields[1], out w))
                    {
                        throw new ArgumentException($"File {path} line {linenum} contains bogus text {line}");
                    }
                    var added1 = adjacencies[v].Add((Vertex)w);
                    var added2 = adjacencies[w].Add((Vertex)v);
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
            var prefix = orderstr + "\t" + size.ToString() + "\t";
            using (var file = new StreamReader(path))
            {
                string header = file.ReadLine()!;
                string? line;
                while ((line = file.ReadLine()) != null)
                {
                    if (line.StartsWith(prefix))
                    {
                        if (!int.TryParse(line.Substring(prefix.Length), out int c))
                        {
                            throw new ArgumentException($"File {path} has bogus line “{line}”");
                        }
                        return c;
                    }
                }
            }
            throw new ArgumentException($"File {path} lacks order {orderstr} size {size}");
        }
    }
}
