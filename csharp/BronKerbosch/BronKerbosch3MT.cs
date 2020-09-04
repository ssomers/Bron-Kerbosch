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

internal static class BronKerbosch3MT
{
    public static void Explore(UndirectedGraph graph, IReporter reporter)
    {
        var excluded = new HashSet<Vertex>();

        var startVertices = new BufferBlock<Vertex>(new DataflowBlockOptions { BoundedCapacity = 32 });
        int sent = 0;
        int received = 0;
        Task.Factory.StartNew(async delegate
            {
                foreach (var v in Degeneracy.Ordering(graph, drop: 1))
                {
                    await startVertices.SendAsync(v).ConfigureAwait(false);
                    ++sent;
                }
                startVertices.Complete();
            },
            CancellationToken.None,
            TaskCreationOptions.None,
            TaskScheduler.Default);
        var task = Task.Factory.StartNew(async delegate
            {
                try
                {
                    while (true)
                    {
                        Vertex v = await startVertices.ReceiveAsync().ConfigureAwait(false);
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
                                TaskScheduler.Default);
                        }
                        else
                        {
                            Debug.Assert(!CollectionsUtil.AreDisjoint(neighbours, excluded));
                        }
                        excluded.Add(v);
                        ++received;
                    }
                }
                catch (InvalidOperationException)
                {
                }
            },
            CancellationToken.None,
            TaskCreationOptions.LongRunning,
            TaskScheduler.Default).Unwrap();
        task.Wait();
        if (sent != received)
            throw new Exception($"{sent} sent <> {received} received");
    }
}
