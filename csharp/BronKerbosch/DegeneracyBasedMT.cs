// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot,
// implemented by multiple threads.

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks.Dataflow;

internal static class DegeneracyBasedMT<VertexSet, VertexSetMgr, TAccumulator>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
    where TAccumulator : ICliqueAccumulator<TAccumulator>, new()
{
    private sealed record VisitJob(Vertex StartVtx, VertexSet NeighbouringCandidates,
                                                    VertexSet NeighbouringExcluded)
    { }

    // Step 1: order vertices & prepare visit.
    private static IEnumerable<VisitJob> Step1(UndirectedGraph<VertexSet, VertexSetMgr> graph)
    {
        var degeneracy = new Degeneracy<VertexSet, VertexSetMgr>(graph);
        foreach (Vertex v in degeneracy.Iter())
        {
            VertexSet neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            (VertexSet neighbouringCandidates, VertexSet neighbouringExcluded) =
                VertexSetMgr.Partition(neighbours, degeneracy.IsCandidate);
            Debug.Assert(neighbouringCandidates.Any());
            yield return new VisitJob(v, neighbouringCandidates, neighbouringExcluded);
        }
    }

    // Step 2: visit vertices.
    private static TAccumulator Step2(UndirectedGraph<VertexSet, VertexSetMgr> graph,
                                      int minCliqueSize,
                                      PivotChoice pivotChoice,
                                      VisitJob job)
    {
        TAccumulator threadAccumulator = new();
        CliqueConsumer<TAccumulator> threadConsumer = new(minCliqueSize, threadAccumulator);
        Pivot<VertexSet, VertexSetMgr, TAccumulator>.Visit(graph, threadConsumer, pivotChoice,
                                             job.NeighbouringCandidates,
                                             job.NeighbouringExcluded,
                                             [job.StartVtx]);
        return threadAccumulator;
    }


    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph,
                               int minCliqueSize,
                               TAccumulator mainStorage,
                               PivotChoice pivotChoice,
                               int maxDegreeOfParallelism)
    {
        var starter = new TransformManyBlock<UndirectedGraph<VertexSet, VertexSetMgr>, VisitJob>(Step1);
        var spawner = new TransformBlock<VisitJob, TAccumulator>(
            job => Step2(graph, minCliqueSize, pivotChoice, job),
            new ExecutionDataflowBlockOptions() { MaxDegreeOfParallelism = maxDegreeOfParallelism });
        var gatherer = new ActionBlock<TAccumulator>(mainStorage.Absorb);
        var linkOptions = new DataflowLinkOptions { PropagateCompletion = true };
        _ = starter.LinkTo(spawner, linkOptions);
        _ = spawner.LinkTo(gatherer, linkOptions);

        var posted = starter.Post(graph);
        Trace.Assert(posted);
        starter.Complete();
        gatherer.Completion.Wait();
    }
}
