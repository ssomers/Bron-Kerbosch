using BronKerbosch;
using BronKerboschStudy;
using System.Collections.Immutable;
using System.Diagnostics;
using static System.Globalization.CultureInfo;

static SampleStatistics[] BronKerboschTimed<VertexSet, VertexSetMgr>(
    string orderstr,
    int size,
    int[] funcIndices,
    int timed_samples)
    where VertexSet : IEnumerable<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    var sw = Stopwatch.StartNew();
    var graph = RandomUndirectedGraph<VertexSet, VertexSetMgr>.Read(orderstr, size);
    sw.Stop();
    var secs = sw.ElapsedMilliseconds / 1e3;
    Console.WriteLine($"{VertexSetMgr.Name()}-based random graph of order {orderstr},"
        + $" {size} edges, {graph.CliqueCount} cliques: (generating took {secs:.3}s)");
    SampleStatistics[] times = new SampleStatistics[Portfolio.FuncNames.Length];

    List<ImmutableArray<Vertex>>? firstResult = null;
    for (var sample = 0; sample <= timed_samples; ++sample)
    {
        foreach (var funcIndex in funcIndices)
        {
            if (sample == 0)
            {
                CollectingReporter reporter = new();
                sw.Restart();
                Portfolio.Explore(funcIndex, graph.Graph, reporter);
                sw.Stop();
                secs = sw.ElapsedMilliseconds / 1e3;
                if (timed_samples == 0 || secs >= 3.0)
                    Console.WriteLine($"  {Portfolio.FuncNames[funcIndex],10} {secs,6:N2}s");
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
                CountingReporter reporter = new();
                sw.Restart();
                Portfolio.Explore(funcIndex, graph.Graph, reporter);
                sw.Stop();
                secs = sw.ElapsedMilliseconds / 1e3;
                times[funcIndex].Put(secs);
            }
        }
    }
    foreach (var funcIndex in funcIndices)
    {
        var funcName = Portfolio.FuncNames[funcIndex];
        var mean = times[funcIndex].Mean;
        var reldev = times[funcIndex].Deviation / mean;
        Console.WriteLine($"  {funcName,-10} {mean,6:N3}s ± {reldev:P0}");
    }
    return times;
}

SampleStatistics[] Bk_core
(SetType setType, string orderstr, int size, int[] funcIndices, int timed_samples) => setType switch
{
    SetType.HashSet => BronKerboschTimed<HashSet<Vertex>, HashSetMgr>(orderstr, size, funcIndices, timed_samples),
    SetType.SortedSet => BronKerboschTimed<SortedSet<Vertex>, SortedSetMgr>(orderstr, size, funcIndices, timed_samples),
    _ => throw new ArgumentOutOfRangeException(nameof(setType)),
};

string SetTypeName(SetType setType) => setType switch
{
    SetType.HashSet => HashSetMgr.Name(),
    SetType.SortedSet => SortedSetMgr.Name(),
    _ => throw new ArgumentOutOfRangeException(nameof(setType)),
};

void Bk(
    string orderstr,
    IEnumerable<int> sizes, Func<SetType, int, IEnumerable<int>> includedFuncs,
    int timed_samples)
{
    const string tmpfname = "tmp.csv";
    using (var fo = new StreamWriter(tmpfname,
                                     new System.Text.UTF8Encoding(encoderShouldEmitUTF8Identifier: false),
                                     new FileStreamOptions { Mode = FileMode.Create, Access = FileAccess.Write }))
    {
        var setTypesUsed = Array.Empty<SetType>();
        foreach (var size in sizes)
        {
            var stats = new Dictionary<SetType, SampleStatistics[]>();
            foreach (SetType setType in Enum.GetValues(typeof(SetType)))
            {
                var funcIndices = includedFuncs(setType, size).ToArray();
                if (funcIndices.Length > 0)
                {
                    stats.Add(setType, Bk_core(setType, orderstr, size, funcIndices, timed_samples));
                }
            }

            if (setTypesUsed.Length == 0)
            {
                setTypesUsed = [.. stats.Keys];
                if (setTypesUsed.Length == 0)
                    throw new ArgumentException("includedFuncs excludes all set types for smallest size");
                fo.Write("Size");
                foreach (var setType in setTypesUsed)
                {
                    foreach (var func_name in Portfolio.FuncNames)
                    {
                        var name = $"{func_name}@{SetTypeName(setType)}";
                        fo.Write($",{name} min,{name} mean,{name} max");
                    }
                }
                fo.WriteLine();
            }
            fo.Write($"{size}");
            foreach (var setType in setTypesUsed)
            {
                var times = stats.GetValueOrDefault(setType, new SampleStatistics[Portfolio.FuncNames.Length]);
                foreach ((var funcIndex, var funcName) in Portfolio.FuncNames.Select((n, i) => (i, n)))
                {
                    var max = times[funcIndex].Max;
                    var min = times[funcIndex].Min;
                    var mean = times[funcIndex].Mean;
                    if (double.IsNaN(mean))
                        fo.Write(",,,");
                    else
                        fo.Write(string.Format(InvariantCulture, ",{0},{1},{2}", min, mean, max));
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

Debug.Fail("Run Release build for meaningful measurements");

var allFuncIndices = Enumerable.Range(0, Portfolio.FuncNames.Length);
var mostFuncIndices = Enumerable.Range(1, Portfolio.FuncNames.Length - 1);
Bk("100", Range(2_000, 3_001, 50), (_, size) => allFuncIndices, 5); // max 4_950
Bk("10k", Range(10_000, 100_000, 10_000).Concat(Range(100_000, 200_001, 25_000)),
    (_, size) => mostFuncIndices, 3);
Bk("1M", Range(500_000, 2_000_000, 250_000)
        .Concat(Range(2_000_000, 5_000_001, 1_000_000)),
    (setType, size) => setType switch
    {
        SetType.HashSet => size > 2_000_000 ? [4, 5, 6] : mostFuncIndices,
        SetType.SortedSet => [],
        _ => throw new ArgumentOutOfRangeException(nameof(setType)),
    }, 3);


internal enum SetType
{
    HashSet,
    SortedSet
};
