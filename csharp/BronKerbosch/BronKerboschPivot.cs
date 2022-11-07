// Bron-Kerbosch algorithm with pivot picked arbitrarily

using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    internal static class Pivot
    {
        public enum Choice
        {
            MaxDegreeLocal,
            MaxDegreeLocalX
        }

        public static void Explore(UndirectedGraph graph, IReporter reporter, Pivot.Choice pivotChoice)
        {
            var order = graph.Order;
            if (order == 0)
            {
                return;
            }
            var pivot = graph.MaxDegreeVertex();
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            var excluded = new HashSet<Vertex>(capacity: order);
            foreach (var v in Enumerable.Range(0, order).Select(Vertex.Nth))
            {
                var neighbours = graph.Neighbours(v);
                if (neighbours.Any() && !neighbours.Contains(pivot))
                {
                    var neighbouringExcluded = CollectionsUtil.Intersection(neighbours, excluded);
                    if (neighbouringExcluded.Count < neighbours.Count)
                    {
                        var neighbouringCandidates = CollectionsUtil.Difference(neighbours, neighbouringExcluded);
                        Visit(graph, reporter, pivotChoice,
                              neighbouringCandidates, neighbouringExcluded,
                              ImmutableArray.Create<Vertex>(v));
                    }
                    var added = excluded.Add(v);
                    Debug.Assert(added);
                }
            }
        }

        public static void Visit(UndirectedGraph graph, IReporter reporter, Choice choice,
                                 ISet<Vertex> candidates, ISet<Vertex> excluded,
                                 ImmutableArray<Vertex> cliqueInProgress)
        {
            Debug.Assert(candidates.All(v => graph.Degree(v) > 0));
            Debug.Assert(excluded.All(v => graph.Degree(v) > 0));
            Debug.Assert(!candidates.Overlaps(excluded));
            Debug.Assert(candidates.Count >= 1);
            if (candidates.Count == 1)
            {
                // Same logic as below, stripped down
                var v = candidates.First();
                var neighbours = graph.Neighbours(v);
                if (CollectionsUtil.AreDisjoint(neighbours, excluded))
                {
                    reporter.Record(CollectionsUtil.Append(cliqueInProgress, v));
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
                var localDegree = CollectionsUtil.IntersectionSize(neighbours, candidates);
                if (localDegree == 0)
                {
                    // Same logic as below, stripped down
                    if (CollectionsUtil.AreDisjoint(neighbours, excluded))
                    {
                        reporter.Record(CollectionsUtil.Append(cliqueInProgress, v));
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
            if (choice == Choice.MaxDegreeLocalX)
            {
                foreach (var v in excluded)
                {
                    var neighbours = graph.Neighbours(v);
                    var localDegree = CollectionsUtil.IntersectionSize(neighbours, candidates);
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
                    var removed = candidates.Remove(v);
                    Debug.Assert(removed);
                    var neighbouringCandidates = CollectionsUtil.Intersection(neighbours, candidates);
                    if (neighbouringCandidates.Any())
                    {
                        var neighbouringExcluded = CollectionsUtil.Intersection(neighbours, excluded);
                        Visit(graph, reporter, choice,
                              neighbouringCandidates, neighbouringExcluded,
                              CollectionsUtil.Append(cliqueInProgress, v));
                    }
                    else if (CollectionsUtil.AreDisjoint(neighbours, excluded))
                    {
                        reporter.Record(CollectionsUtil.Append(cliqueInProgress, v));
                    }
                    var added = excluded.Add(v);
                    Debug.Assert(added);
                }
            }
        }
    }
}
