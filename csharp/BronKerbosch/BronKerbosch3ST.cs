// Bron-Kerbosch algorithm with degeneracy ordering,
// choosing a pivot from both candidates and excluded vertices (IK_GPX).

using BronKerbosch;
using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using System.Threading.Tasks.Dataflow;

internal static class BronKerbosch3ST<VertexSet, VertexSetMgr>
    where VertexSet : IEnumerable<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
{
    internal sealed class NestedReporter(ITargetBlock<ImmutableArray<Vertex>>? target) : IReporter
    {
        public int useAfterClose;
        public int postFailed;

        public void Record(ImmutableArray<Vertex> clique)
        {
            if (target is null)
            {
                ++useAfterClose; // breakpoint candidate
            }
            else if (!target.Post(clique))
            {
                ++postFailed; // breakpoint candidate
            }
        }

        public void Close()
        {
            if (target is null)
            {
                ++useAfterClose; // breakpoint candidate
            }
            target = null;
        }
    }

    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, IReporter finalReporter)
    {
        var scheduler = TaskScheduler.Default;
        int sent = 0;
        int received = 0;

        // Step 3: collect results.
        var collect = new ActionBlock<ImmutableArray<Vertex>>(finalReporter.Record);

        // Step 2: visit vertices.
        var reporter = new NestedReporter(collect);
        int waitGroup = 1;
        void completion(Task _)
        {
            if (Interlocked.Decrement(ref waitGroup) == 0)
            {
                reporter.Close();
                collect.Complete();
            }
        }
        var excluded = VertexSetMgr.Empty();
        var visit = new ActionBlock<Vertex>(v =>
            {
                var neighbours = graph.Neighbours(v);
                Debug.Assert(neighbours.Any());
                var neighbouringCandidates = VertexSetMgr.Difference(neighbours, excluded);
                if (neighbouringCandidates.Any())
                {
                    var neighbouringExcluded = VertexSetMgr.Intersection(excluded, neighbours);
                    _ = Interlocked.Increment(ref waitGroup);
                    _ = Task.Run(delegate
                        {
                            Pivot<VertexSet, VertexSetMgr>.Visit(graph, reporter, PivotChoice.MaxDegreeLocal,
                                        neighbouringCandidates, neighbouringExcluded,
                                        [v]);
                        }).ContinueWith(completion, scheduler);
                }
                else
                {
                    Debug.Assert(VertexSetMgr.Overlaps(neighbours, excluded));
                }
                var added = VertexSetMgr.Add(excluded, v);
                Debug.Assert(added);
                ++received;
            });
        _ = visit.Completion.ContinueWith(completion, scheduler);

        // Step 1: feed vertices in order.
        _ = Task.Run(delegate
            {
                foreach (var v in Degeneracy<VertexSet, VertexSetMgr>.Ordering(graph, drop: 1))
                {
                    while (!visit.Post(v))
                    {
                        throw new InvalidOperationException("Post failed");
                    }
                    ++sent;
                }
                visit.Complete();
            });

        collect.Completion.Wait();
        if (sent != received)
        {
            throw new InvalidOperationException($"{sent} sent <> {received} received");
        }
        if (reporter.useAfterClose != 0)
        {
            throw new InvalidOperationException($"Reporter use after Close ({reporter.useAfterClose} times)");
        }
        if (reporter.postFailed != 0)
        {
            throw new InvalidOperationException($"Record failed  ({reporter.postFailed} times)");
        }
    }
}
