// Bron-Kerbosch algorithm with pivot picked arbitrarily

using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    internal enum PivotChoice
    {
        MaxDegreeLocal,
        MaxDegreeLocalX
    }

    internal static class Pivot<VertexSet, VertexSetMgr>
        where VertexSet : ISet<Vertex>
        where VertexSetMgr : IVertexSetMgr<VertexSet>
    {
        public static void Explore(UndirectedGraph<VertexSet, VertexSetMgr> graph, ICliqueConsumer consumer, PivotChoice pivotChoice)
        {
            var order = graph.Order;
            if (order == 0)
            {
                return;
            }
            var pivot = graph.MaxDegreeVertex();
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            var excluded = VertexSetMgr.EmptyWithCapacity(order);
            foreach (var v in Enumerable.Range(0, order).Select(Vertex.Nth))
            {
                var neighbours = graph.Neighbours(v);
                if (neighbours.Any() && !neighbours.Contains(pivot))
                {
                    var neighbouringExcluded = VertexSetMgr.Intersection(neighbours, excluded);
                    if (neighbouringExcluded.Count < neighbours.Count)
                    {
                        var neighbouringCandidates = VertexSetMgr.Difference(neighbours, neighbouringExcluded);
                        Visit(graph, consumer, pivotChoice,
                              neighbouringCandidates, neighbouringExcluded,
                              [v]);
                    }
                    var added = VertexSetMgr.Add(excluded, v);
                    Debug.Assert(added);
                }
            }
        }

        public static void Visit(UndirectedGraph<VertexSet, VertexSetMgr> graph, ICliqueConsumer consumer, PivotChoice choice,
                                 VertexSet candidates, VertexSet excluded,
                                 ImmutableArray<Vertex> cliqueInProgress)
        {
            Debug.Assert(candidates.All(graph.HasNeighbours));
            Debug.Assert(excluded.All(graph.HasNeighbours));
            Debug.Assert(!VertexSetMgr.Overlaps(candidates, excluded));
            Debug.Assert(candidates.Count >= 1);
            if (candidates.Count == 1)
            {
                // Same logic as below, stripped down
                var v = candidates.First();
                var neighbours = graph.Neighbours(v);
                if (!VertexSetMgr.Overlaps(neighbours, excluded))
                {
                    consumer.Accept([.. cliqueInProgress, v]);
                }
                return;
            }

            Vertex pivot;
            var remainingCandidates = new Vertex[candidates.Count];
            var remainingCandidateCount = 0;
            // Quickly handle locally unconnected candidates while finding pivot
            const int INVALID = int.MaxValue;
            pivot = Vertex.Nth(INVALID);
            var seenLocalDegree = 0;
            foreach (var v in candidates)
            {
                var neighbours = graph.Neighbours(v);
                var localDegree = VertexSetMgr.IntersectionSize(neighbours, candidates);
                if (localDegree == 0)
                {
                    // Same logic as below, stripped down
                    if (!VertexSetMgr.Overlaps(neighbours, excluded))
                    {
                        consumer.Accept([.. cliqueInProgress, v]);
                    }
                }
                else
                {
                    if (seenLocalDegree < localDegree)
                    {
                        seenLocalDegree = localDegree;
                        pivot = v;
                    }
                    remainingCandidates[remainingCandidateCount] = v;
                    remainingCandidateCount += 1;
                }
            }
            if (seenLocalDegree == 0)
            {
                return;
            }

            Debug.Assert(pivot.Index() != INVALID);
            if (choice == PivotChoice.MaxDegreeLocalX)
            {
                foreach (var v in excluded)
                {
                    var neighbours = graph.Neighbours(v);
                    var localDegree = VertexSetMgr.IntersectionSize(neighbours, candidates);
                    if (seenLocalDegree < localDegree)
                    {
                        seenLocalDegree = localDegree;
                        pivot = v;
                    }
                }
            }

            for (var i = 0; i < remainingCandidateCount; ++i)
            {
                var v = remainingCandidates[i];
                var neighbours = graph.Neighbours(v);
                Debug.Assert(neighbours.Any());
                if (!neighbours.Contains(pivot))
                {
                    var removed = VertexSetMgr.Remove(candidates, v);
                    Debug.Assert(removed);
                    var neighbouringCandidates = VertexSetMgr.Intersection(neighbours, candidates);
                    if (neighbouringCandidates.Any())
                    {
                        var neighbouringExcluded = VertexSetMgr.Intersection(neighbours, excluded);
                        Visit(graph, consumer, choice,
                              neighbouringCandidates, neighbouringExcluded,
                              [.. cliqueInProgress, v]);
                    }
                    else if (!VertexSetMgr.Overlaps(neighbours, excluded))
                    {
                        consumer.Accept([.. cliqueInProgress, v]);
                    }
                    var added = VertexSetMgr.Add(excluded, v);
                    Debug.Assert(added);
                }
            }
        }
    }
}
