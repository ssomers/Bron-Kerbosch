using BronKerbosch;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;

namespace BronKerboschStudy
{
    class Benchmark
    {
        private static SampleStatistics[] BronKerboschTimed(UndirectedGraph graph, int[] func_indices, int samples)
        {
            List<List<Vertex>> first = null;
            SampleStatistics[] times = Enumerable.Range(0, Portfolio.FUNC_NAMES.Length).Select(func_index => new SampleStatistics()).ToArray();
            for (int sample = 0; sample < samples; ++sample)
            {
                foreach (int func_index in func_indices)
                {
                    var reporter = new SimpleReporter();
                    var sw = Stopwatch.StartNew();
                    Portfolio.Explore(func_index, graph, reporter);
                    sw.Stop();
                    var secs = sw.ElapsedMilliseconds / 1e3;
                    if (secs >= 3.0)
                        Console.WriteLine($"  {Portfolio.FUNC_NAMES[func_index],8}: {secs,6:N2}s");
                    if (sample < 2)
                    {
                        Portfolio.SortCliques(reporter.Cliques);
                        if (first == null)
                            first = reporter.Cliques;
                        else
                            Portfolio.AssertSameCliques(first, reporter.Cliques);
                    }
                    times[func_index].Put(secs);
                }
            }
            return times;
        }

        private static void bk(string orderstr, IEnumerable<int> sizes, Func<int, IEnumerable<int>> includedFuncs, int samples)
        {
            var tmpfname = "tmp.csv";
            using (StreamWriter fo = new StreamWriter(tmpfname))
            {
                fo.Write("Size");
                foreach (string name in Portfolio.FUNC_NAMES)
                {
                    fo.Write(",{0} min,{0} mean,{0} max", name);
                }
                fo.Write("\n");
                foreach (int size in sizes)
                {
                    var func_indices = includedFuncs(size).ToArray();
                    var g = RandomUndirectedGraph.Read(orderstr, size);
                    var stats = BronKerboschTimed(g, func_indices, samples);
                    fo.Write($"{size}");
                    foreach ((int func_index, string func_name) in Portfolio.FUNC_NAMES.Select((n, i) => (i, n)))
                    {
                        var max = stats[func_index].Max;
                        var min = stats[func_index].Min;
                        var mean = stats[func_index].Mean;
                        fo.Write($",{min},{mean},{max}");
                        if (!Double.IsNaN(mean))
                        {
                            var reldev = stats[func_index].Deviation / mean;
                            Console.WriteLine($"order {orderstr,4:D} size {size,7:D} {func_name,-8}: {mean,6:N3}s ± {reldev:P0}");
                        }
                    }
                    fo.WriteLine();
                }
            }
            var path = @"..\bron_kerbosch_c#_order_" + orderstr + ".csv";
            if (File.Exists(path))
                File.Delete(path);
            File.Move(tmpfname, path);
        }

        private static IEnumerable<int> Range(int start, int stop, int step)
        {
            var current = start;
            while (current < stop)
            {
                yield return current;
                current += step;
            }
        }

        static void Main(string[] args)
        {
            var all_func_indices = Enumerable.Range(0, Portfolio.FUNC_NAMES.Length);
            var most_func_indices = Enumerable.Range(1, Portfolio.FUNC_NAMES.Length - 1);
            Debug.Fail("Run Release build for meaningful measurements");
            bk("100", Range(2_000, 3_001, 50), (size) => all_func_indices, 5); // max 4_950
            bk("10k", Range(10_000, 100_000, 10_000).Concat(Range(100_000, 200_001, 25_000)), (size) => most_func_indices, 3);
            bk("1M", Range(20_000, 200_000, 20_000).Concat(Range(200_000, 1_000_001, 200_000)), (size) => most_func_indices, 3);
        }
    }
}
