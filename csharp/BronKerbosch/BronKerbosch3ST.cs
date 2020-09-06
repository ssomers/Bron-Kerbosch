// Bron-Kerbosch algorithm with degeneracy ordering,
// choosing a pivot from both candidates and excluded vertices (IK_GPX).

using BronKerbosch;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;
using System.Numerics;
using System.Threading;
using System.Threading.Tasks;
using System.Threading.Tasks.Dataflow;
using Vertex = System.UInt32;

public static class MyExtensions
{
    public static async Task<T?> ReceiveAsyncIfEver<T>(this IReceivableSourceBlock<T> source)
        where T : struct
    {
        try
        {
            return await source.ReceiveAsync().ConfigureAwait(false);
        }
        catch (InvalidOperationException)
        {
            return null;
        }
    }
}

internal static class BronKerbosch3ST
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
    {
        var scheduler = TaskScheduler.Default;
        var task = Task.Factory.StartNew(delegate
            {
                var excluded = new HashSet<Vertex>();
                foreach (var v in Degeneracy.Ordering(graph, drop: 1))
                {
                    var neighbours = graph.Neighbours(v);
                    Debug.Assert(neighbours.Any());
                    var neighbouringCandidates = CollectionsUtil.Difference(neighbours, excluded);
                    if (neighbouringCandidates.Any())
                    {
                        var neighbouringExcluded = CollectionsUtil.Intersection(excluded, neighbours);
                        _ = Task.Factory.StartNew(
                            () => Pivot.Visit(graph, reporter,
                                              Pivot.Choice.MaxDegreeLocal, Pivot.Choice.MaxDegreeLocal,
                                              neighbouringCandidates, neighbouringExcluded,
                                              ImmutableArray.Create(v)),
                            CancellationToken.None,
                            TaskCreationOptions.AttachedToParent,
                            scheduler);
                    }
                    else
                    {
                        Debug.Assert(!CollectionsUtil.AreDisjoint(neighbours, excluded));
                    }
                    excluded.Add(v);
                }
            },
            CancellationToken.None,
            TaskCreationOptions.None,
            scheduler);
        task.Wait();
        reporter.Close();
    }
}
