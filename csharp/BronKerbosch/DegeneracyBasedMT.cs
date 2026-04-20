// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot,
// implemented by multiple threads.

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Threading.Tasks.Dataflow;

internal static class DegeneracyBasedMT<VertexSet, VertexSetMgr>
    where VertexSet : ISet<Vertex>
    where VertexSetMgr : IVertexSetMgr<VertexSet>
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
            var neighbours = graph.Neighbours(v);
            Debug.Assert(neighbours.Any());
            var (neighbouringCandidates, neighbouringExcluded) =
                VertexSetMgr.Partition(neighbours, degeneracy.IsCandidate);
            Debug.Assert(neighbouringCandidates.Any());
            yield return new VisitJob(v, neighbouringCandidates, neighbouringExcluded);
        }
    }

    // Step 2: visit vertices.
    private static ICliqueConsumer Step2(UndirectedGraph<VertexSet, VertexSetMgr> graph,
                                         ICliqueConsumer threadConsumer,
                                         PivotChoice pivotChoice,
                                         VisitJob job)
    {
        Pivot<VertexSet, VertexSetMgr>.Visit(graph, threadConsumer, pivotChoice,
                                             job.NeighbouringCandidates,
                                             job.NeighbouringExcluded,
                                             [job.StartVtx]);
        return threadConsumer;
    }


    public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph,
                               ICliqueConsumer mainConsumer,
                               PivotChoice pivotChoice,
                               int numVisitingThreads)
    {
        var starter = new TransformManyBlock<UndirectedGraph<VertexSet, VertexSetMgr>, VisitJob>(Step1);
        var spawner = new TransformBlock<VisitJob, ICliqueConsumer>(
            job => Step2(graph, mainConsumer.StartNew(), pivotChoice, job),
            new ExecutionDataflowBlockOptions() { MaxDegreeOfParallelism = numVisitingThreads });
        var gatherer = new ActionBlock<ICliqueConsumer>(mainConsumer.Absorb);
        var linkOptions = new DataflowLinkOptions { PropagateCompletion = true };
        _ = starter.LinkTo(spawner, linkOptions);
        _ = spawner.LinkTo(gatherer, linkOptions);

        var posted = starter.Post(graph);
        Trace.Assert(posted);
        starter.Complete();
        gatherer.Completion.Wait();
    }
}
