using BronKerbosch;
using BronKerboschStudy;
using System.Collections.Immutable;
using System.Diagnostics;
using static System.Globalization.CultureInfo;

static SampleStatistics[] BronKerboschTimed<VertexSet, VertexSetMgr>(
    RandomUndirectedGraph<VertexSet,
    VertexSetMgr> graph,
    int[] funcIndices,
    int samples)
    where VertexSet : IEnumerable<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    List<ImmutableArray<Vertex>>? firstResult = null;
    SampleStatistics[] times = Enumerable.Range(0, Portfolio.FuncNames.Length)
        .Select(funcIndex => new SampleStatistics()).ToArray();
    for (var sample = samples == 1 ? 1 : 0; sample <= samples; ++sample)
    {
        foreach (var funcIndex in funcIndices)
        {
            if (sample == 0)
            {
                var reporter = new SimpleReporter();
                var sw = Stopwatch.StartNew();
                Portfolio.Explore(funcIndex, graph.Graph, reporter);
                sw.Stop();
                var secs = sw.ElapsedMilliseconds / 1e3;
                if (secs >= 3.0)
                    Console.WriteLine($"  {Portfolio.FuncNames[funcIndex],8}: {secs,6:N2}s");
                Portfolio.SortCliques(reporter.Cliques);
                if (firstResult == null)
                {
                    if (reporter.Cliques.Count != graph.CliqueCount)
                    {
                        throw new InvalidProgramException(
                            $"Expected {graph.CliqueCount} cliques, got {reporter.Cliques.Count}");
                    }
                    firstResult = reporter.Cliques;
                }
                else
                {
                    Portfolio.AssertSameCliques(firstResult, reporter.Cliques);
                }
            }
            else
            {
                var reporter = new CountingReporter();
                var sw = Stopwatch.StartNew();
                Portfolio.Explore(funcIndex, graph.Graph, reporter);
                sw.Stop();
                var secs = sw.ElapsedMilliseconds / 1e3;
                times[funcIndex].Put(secs);
            }
        }
    }
    return times;
}

void Bk<VertexSet, VertexSetMgr>(
    string orderstr,
    IEnumerable<int> sizes, Func<int, IEnumerable<int>> includedFuncs,
    int samples)
    where VertexSet : IEnumerable<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    const string tmpfname = "tmp.csv";
    using (var fo = new StreamWriter(tmpfname,
                                     new System.Text.UTF8Encoding(encoderShouldEmitUTF8Identifier: false),
                                     new FileStreamOptions { Mode = FileMode.Create, Access = FileAccess.Write }))
    {
        fo.Write("Size");
        foreach (var name in Portfolio.FuncNames)
        {
            fo.Write(",{0} min,{0} mean,{0} max", name);
        }
        fo.WriteLine();
        foreach (var size in sizes)
        {
            var funcIndices = includedFuncs(size).ToArray();
            var g = RandomUndirectedGraph<VertexSet, VertexSetMgr>.Read(orderstr, size);
            var stats = BronKerboschTimed(g, funcIndices, samples);
            fo.Write($"{size}");
            foreach ((var funcIndex, var funcName) in Portfolio.FuncNames.Select((n, i) => (i, n)))
            {
                var max = stats[funcIndex].Max;
                var min = stats[funcIndex].Min;
                var mean = stats[funcIndex].Mean;
                fo.Write(string.Format(InvariantCulture, ",{0},{1},{2}", min, mean, max));
                if (!double.IsNaN(mean))
                {
                    var reldev = stats[funcIndex].Deviation / mean;
                    Console.WriteLine(
                        $"order {orderstr,4:D} size {size,7:D} {funcName,-8}: {mean,6:N3}s ± {reldev:P0}");
                }
            }
            fo.WriteLine();
        }
    }

    var path = $"..\\bron_kerbosch_csharp_order_{orderstr}.csv";
    if (File.Exists(path))
        File.Delete(path);
    File.Move(tmpfname, path);
}

IEnumerable<int> Range(int start, int stop, int step)
{
    var current = start;
    while (current < stop)
    {
        yield return current;
        current += step;
    }
}

var allFuncIndices = Enumerable.Range(0, Portfolio.FuncNames.Length);
var mostFuncIndices = Enumerable.Range(1, Portfolio.FuncNames.Length - 1);
Debug.Fail("Run Release build for meaningful measurements");
//Bk<List<Vertex>, ListMgr>("100", Range(2_000, 3_001, 50), size => allFuncIndices, 5); // max 4_950
Bk<HashSet<Vertex>, HashSetMgr>("100", Range(2_000, 3_001, 50), size => allFuncIndices, 5); // max 4_950
Bk<HashSet<Vertex>, HashSetMgr>("10k", Range(10_000, 100_000, 10_000).Concat(Range(100_000, 200_001, 25_000)),
    size => mostFuncIndices, 3);
Bk<HashSet<Vertex>, HashSetMgr>("1M", Range(500_000, 2_000_000, 250_000)
        .Concat(Range(2_000_000, 5_000_001, 1_000_000)),
    size => size > 3_000_000 ? new[] { 2, 4, 5, 6 } : mostFuncIndices, 3);
