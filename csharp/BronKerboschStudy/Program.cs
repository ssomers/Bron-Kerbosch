using BronKerbosch;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;

namespace BronKerboschStudy
{
    class Program
    {
        private static readonly string[] FUNC_NAMES = new string[] { "Ver1", "Ver2" };
        private static SampleStatistics[] BronKerboschTimed(UndirectedGraph graph, int[] func_indices, int samples)
        {
            List<List<Vertex>> first = null;
            SampleStatistics[] times = Enumerable.Range(0, Portfolio.NUM_FUNCS).Select(func_index => new SampleStatistics()).ToArray();
            for (int sample = 0; sample < samples; ++sample)
            {
                foreach (int func_index in func_indices)
                {
                    var reporter = new SimpleReporter();
                    DateTime begin = DateTime.Now;
                    Portfolio.Explore(func_index, graph, reporter);
                    var secs = new TimeSpan(DateTime.Now.Ticks - begin.Ticks).TotalSeconds;
                    if (secs >= 3.0)
                        Console.WriteLine($"  {FUNC_NAMES[func_index],8}: {secs,5:N2}s");
                    if (sample < 2)
                    {
                        Portfolio.SortCliques(reporter.Cliques);
                        if (first == null)
                        {
                            first = reporter.Cliques;
                        }
                        else if (first.SequenceEqual(reporter.Cliques))
                            Console.WriteLine($"  {FUNC_NAMES[func_index],8}: expected {first.Count} cliques, obtained {reporter.Cliques.Count} cliques");
                    }
                    times[func_index].Put(secs);
                }
            }
            return times;
        }

        private static void bk(string orderstr, IEnumerable<int> sizes, int[] func_indices, int samples)
        {
            int order;
            if (orderstr.EndsWith("M"))
                order = Int32.Parse(orderstr.Substring(0, orderstr.Length - 1)) * 1_000_000;
            else if (orderstr.EndsWith("k"))
                order = Int32.Parse(orderstr.Substring(0, orderstr.Length - 1)) * 1_000;
            else
                order = Int32.Parse(orderstr);

            var tmpfname = "tmp.csv";
            using (StreamWriter fo = File.AppendText(tmpfname))
            {
                fo.Write("Size");
                foreach (string name in FUNC_NAMES)
                {
                    fo.Write(",{0} min,{0} mean,{0} max", name);
                }
                fo.Write("\n");
                foreach (int size in sizes)
                {
                    var random = new Random(19680516);
                    var g = RandomUndirectedGraph.Generate(random, order, size);
                    var stats = BronKerboschTimed(g, func_indices, samples);
                    fo.Write($"{size}");
                    foreach ((int func_index, string func_name) in FUNC_NAMES.Select((n, i) => (i, n)))
                    {
                        var max = stats[func_index].Max;
                        var min = stats[func_index].Min;
                        var mean = stats[func_index].Mean;
                        var dev = stats[func_index].Deviation;
                        fo.Write($",{min},{mean},{max}");
                        Console.WriteLine($"order {order,7:D} size {size,7:D} {func_name,8}: {mean,5:N2}s ±{dev,5:N2}s");
                    }
                    fo.WriteLine();
                }
            }
            var path = "..\\..\\..\\..\\bron_kerbosch_c#_order_" + orderstr + ".csv";
            File.Move(tmpfname, path);
        }

        private static IEnumerable<int> Range(int start, int stop, int step)
        {
            return RangeIterator(start, stop, step);
        }

        private static IEnumerable<int> RangeIterator(int start, int stop, int step)
        {
            int x = start;
            while (x < stop)
            {
                yield return x;
                x += step;
            }
        }

        static void Main(string[] args)
        {
            var all_func_indices = new[] { 0, 1 };
            Debug.Fail("Run Release build for meaningful measurements");
            bk("100", Range(2_000, 3_001, 50), // max 4_950
               all_func_indices, 5);
            bk("10k",
               Range(1_000, 10_000, 1_000).Concat(Range(10_000, 200_001, 10_000)),
               all_func_indices,
               5);
            bk("1M",
               Range(0, 1_000_000, 250_000).Concat(Range(1_000_000, 3_000_001, 500_000)),
               all_func_indices, 3);
        }
    }
}
