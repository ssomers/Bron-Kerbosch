using BronKerbosch;
using BronKerboschStudy;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Threading;
using static System.Globalization.CultureInfo;

static SampleStatistics[] BronKerboschTimed<VertexSet, VertexSetMgr>(
    bool genuine,
    string orderstr,
    int size,
    int[] funcIndices,
    int timed_samples)
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    const int warning_interval = 3000;
    var sw = Stopwatch.StartNew();
    var graph = RandomUndirectedGraph<VertexSet, VertexSetMgr>.Read(orderstr, size);
    sw.Stop();
    if (genuine)
    {
        Console.WriteLine($"{VertexSetMgr.Name()}-based random graph of order {orderstr},"
            + $" {size} edges, {graph.CliqueCount} cliques: (generating took {sw.ElapsedMilliseconds}ms)");
    }
    var times = new SampleStatistics[Portfolio.FuncNames.Length];

    List<ImmutableArray<Vertex>>? firstResult = null;
    for (var sample = 0; sample <= timed_samples; ++sample)
    {
        foreach (var funcIndex in funcIndices)
        {
            if (sample == 0)
            {
                int warnings = 0;
                var ticker = new Timer((_) =>
                {
                    warnings += 1;
                    int secs = warnings * warning_interval / 1000;
                    Console.WriteLine($"  {secs} seconds in, {Portfolio.FuncNames[funcIndex]} is still busy collecting");
                }, null, warning_interval, warning_interval);
                CliqueCollector consumer = new();
                Portfolio.Explore(funcIndex, graph.Graph, consumer);
                ticker.Dispose();
                var result = consumer.List();
                Portfolio.SortCliques(result);
                if (firstResult == null)
                {
                    if (result.Count != graph.CliqueCount)
                    {
                        throw new InvalidProgramException(
                            $"Expected {graph.CliqueCount} cliques, got {result.Count}");
                    }
                    firstResult = result;
                }
                else
                {
                    Portfolio.AssertSameCliques(firstResult, result);
                }
            }
            else
            {
                CliqueCounter consumer = new();
                sw.Restart();
                Portfolio.Explore(funcIndex, graph.Graph, consumer);
                sw.Stop();
                var secs = sw.ElapsedMilliseconds / 1e3;
                Debug.Assert(consumer.Count() == firstResult!.Count);
                times[funcIndex].Put(secs);
            }
        }
    }
    if (genuine)
    {
        foreach (var funcIndex in funcIndices)
        {
            var funcName = Portfolio.FuncNames[funcIndex];
            var mean = times[funcIndex].Mean;
            var reldev = times[funcIndex].Deviation / mean;
            Console.WriteLine($"  {funcName,-10} {mean,6:N3}s ± {reldev:P0}");
        }
    }
    return times;
}

#pragma warning disable IDE0061 // Use block body for local function
SampleStatistics[] Bk_core(bool genuine, SetType setType, string orderstr, int size, int[] funcIndices, int timed_samples) => setType switch
{
    SetType.HashSet => BronKerboschTimed<HashSet<Vertex>, HashSetMgr>(genuine, orderstr, size, funcIndices, timed_samples),
    SetType.SortedSet => BronKerboschTimed<SortedSet<Vertex>, SortedSetMgr>(genuine, orderstr, size, funcIndices, timed_samples),
    _ => throw new ArgumentOutOfRangeException(nameof(setType)),
};

string SetTypeName(SetType setType) => setType switch
{
    SetType.HashSet => HashSetMgr.Name(),
    SetType.SortedSet => SortedSetMgr.Name(),
    _ => throw new ArgumentOutOfRangeException(nameof(setType)),
};
#pragma warning restore IDE0061 // Use block body for local function

void Bk(
    bool genuine,
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
            foreach (SetType setType in Enum.GetValues<SetType>())
            {
                var funcIndices = includedFuncs(setType, size).ToArray();
                if (funcIndices.Length > 0)
                {
                    stats.Add(setType, Bk_core(genuine, setType, orderstr, size, funcIndices, timed_samples));
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
                SampleStatistics[] times = stats.GetValueOrDefault(setType, new SampleStatistics[Portfolio.FuncNames.Length]);
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

    var path = $"..\\bron_kerbosch_csharp_order_{(genuine ? orderstr : "warmup")}.csv";
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
Bk(false, "100", [2000], (_, _) => allFuncIndices, 3); // warm up
Thread.Sleep(321);
Bk(true, "100", Range(2_000, 3_001, 50), (_, size) => allFuncIndices, 5); // max 4_950
Bk(true, "10k", Range(10_000, 100_000, 10_000).Concat(Range(100_000, 200_001, 25_000)),
    (_, size) => allFuncIndices.Skip(1), 3);
Bk(true, "1M", Range(500_000, 2_000_000, 250_000)
        .Concat(Range(2_000_000, 5_000_001, 1_000_000)),
    (setType, size) => setType switch
    {
        SetType.HashSet => allFuncIndices.Skip(size > 2_000_000 ? 2 : 1),
        SetType.SortedSet => [],
        _ => throw new ArgumentOutOfRangeException(nameof(setType)),
    }, 3);


internal enum SetType
{
    HashSet,
    SortedSet
};
