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

internal static class BronKerbosch3ST
{
    internal sealed class NestedReporter : IReporter
    {
        private ITargetBlock<ImmutableArray<Vertex>>? Target;

        public NestedReporter(ITargetBlock<ImmutableArray<Vertex>> target)
        {
            Target = target;
        }

        public void Record(ImmutableArray<Vertex> clique)
        {
            if (Target is null)
                throw new Exception("Record after Close");
            if (!Target.Post(clique))
            {
                throw new Exception("Record failed");
            }
        }

        public void Close()
        {
            if (Target is null)
                throw new Exception("Close after Close");
            Target = null;
        }
    }

    public static void Explore(UndirectedGraph graph, IReporter finalReporter)
    {
        var scheduler = TaskScheduler.Default;
        int sent = 0;
        int received = 0;

        // Step 3: collect results.
        var collect = new ActionBlock<ImmutableArray<Vertex>>(clique => finalReporter.Record(clique));

        // Step 2: visit vertices.
        var reporter = new NestedReporter(collect);
        int waitgroup = 1;
        void completion(Task _)
        {
            if (Interlocked.Decrement(ref waitgroup) == 0)
            {
                reporter.Close();
                collect.Complete();
            }
        }
        var excluded = new HashSet<Vertex>();
        var visit = new ActionBlock<Vertex>(v =>
            {
                var neighbours = graph.Neighbours(v);
                Debug.Assert(neighbours.Any());
                var neighbouringCandidates = CollectionsUtil.Difference(neighbours, excluded);
                if (neighbouringCandidates.Any())
                {
                    var neighbouringExcluded = CollectionsUtil.Intersection(excluded, neighbours);
                    Interlocked.Increment(ref waitgroup);
                    _ = Task.Run(delegate
                        {
                            Pivot.Visit(graph, reporter,
                                        Pivot.Choice.MaxDegreeLocal, Pivot.Choice.MaxDegreeLocal,
                                        neighbouringCandidates, neighbouringExcluded,
                                        ImmutableArray.Create(v));
                        }).ContinueWith(completion, scheduler);
                }
                else
                {
                    Debug.Assert(!CollectionsUtil.AreDisjoint(neighbours, excluded));
                }
                excluded.Add(v);
                ++received;
            });
        visit.Completion.ContinueWith(completion, scheduler);

        // Step 1: feed vertices in order.
        Task.Run(delegate
            {
                foreach (var v in Degeneracy.Ordering(graph, drop: 1))
                {
                    while (!visit.Post(v))
                    {
                        throw new Exception("Post failed");
                    }
                    ++sent;
                }
                visit.Complete();
            });

        collect.Completion.Wait();
        if (sent != received)
            throw new Exception($"{sent} sent <> {received} received");
    }
}
