using BronKerbosch;
using System.Collections.Immutable;
using System.Diagnostics;

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
            {
                return int.Parse(orderstr.Remove(orderstr.Length - 1)) * 1_000_000;
            }
            else if (orderstr.EndsWith("k"))
            {
                return int.Parse(orderstr.Remove(orderstr.Length - 1)) * 1_000;
            }
            else
            {
                return int.Parse(orderstr);
            }
        }

        public static RandomUndirectedGraph Read(string orderstr, int size)
        {
            var order = ParsePositiveInt(orderstr);
            var fullyMeshedSize = (long)order * (order - 1) / 2;
            if (size > fullyMeshedSize)
                throw new ArgumentException($"{order} nodes accommodate at most {fullyMeshedSize} edges");

            var edgesPath = $"..\\data\\random_edges_order_{orderstr}.txt";
            const string statsPath = "..\\data\\random_stats.txt";
            var adjacencies = ReadEdges(edgesPath, orderstr, size);
            var cliqueCount = ReadStats(statsPath, orderstr, size);
            var g = new UndirectedGraph(adjacencies);
            Debug.Assert(g.Order == order);
            Debug.Assert(g.Size == size);
            return new RandomUndirectedGraph(g, cliqueCount);
        }

        private static ImmutableArray<HashSet<Vertex>> ReadEdges(string path, string orderstr, int size)
        {
            var order = ParsePositiveInt(orderstr);
            var adjacencies = Enumerable.Range(0, order)
                .Select(_ => new HashSet<Vertex>())
                .ToImmutableArray();
            var linenum = 0;
            using (var file = new StreamReader(path))
            {
                string? line;
                while (linenum < size && (line = file!.ReadLine()) != null)
                {
                    ++linenum;
                    var fields = line.Split(' ');
                    if (!int.TryParse(fields[0], out var v) ||
                        !int.TryParse(fields[1], out var w))
                    {
                        throw new ArgumentException($"File {path} line {linenum} contains bogus text {line}");
                    }
                    var added1 = adjacencies[v].Add(Vertex.Nth(w));
                    var added2 = adjacencies[w].Add(Vertex.Nth(v));
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
            var header = file.ReadLine()!;
            string? line;
            while ((line = file.ReadLine()) != null)
            {
                if (line.StartsWith(prefix))
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
