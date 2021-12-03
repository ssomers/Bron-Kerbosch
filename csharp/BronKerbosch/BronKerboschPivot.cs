// Bron-Kerbosch algorithm with pivot picked arbitrarily

using System;
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
            MaxDegree,
            MaxDegreeLocal,
            MaxDegreeLocalX
        }

        public static void Visit(UndirectedGraph graph, IReporter reporter,
                                 Choice initialChoice, Choice furtherChoice,
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
                if (CollectionsUtil.AreDisjoint(excluded, neighbours))
                    reporter.Record(CollectionsUtil.Append(cliqueInProgress, v));
                return;
            }

            Vertex pivot;
            var remainingCandidates = new Vertex[candidates.Count];
            var remainingCandidateCount = 0;
            if (initialChoice >= Choice.MaxDegreeLocal)
            {
                // Quickly handle locally unconnected candidates while finding pivot
                const int INVALID = int.MaxValue;
                pivot = Vertex.nth(INVALID);
                var seenLocalDegree = 0;
                foreach (var v in candidates)
                {
                    var neighbours = graph.Neighbours(v);
                    var localDegree = CollectionsUtil.IntersectionSize(neighbours, candidates);
                    if (localDegree == 0)
                    {
                        // Same logic as below, stripped down
                        if (CollectionsUtil.AreDisjoint(excluded, neighbours))
                            reporter.Record(CollectionsUtil.Append(cliqueInProgress, v));
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
                    return;
                Debug.Assert(pivot.index != INVALID);
                if (initialChoice == Choice.MaxDegreeLocalX)
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
            }
            else
            {
                pivot = Choose(initialChoice, candidates, graph);
                candidates.CopyTo(remainingCandidates, 0);
                remainingCandidateCount = candidates.Count;
            }

            for (var i = 0; i < remainingCandidateCount; ++i)
            {
                var v = remainingCandidates[i];
                var neighbours = graph.Neighbours(v);
                if (neighbours.Contains(pivot))
                    continue;
                candidates.Remove(v);
                var neighbouringCandidates = CollectionsUtil.Intersection(candidates, neighbours);
                if (neighbouringCandidates.Any())
                {
                    var neighbouringExcluded = CollectionsUtil.Intersection(excluded, neighbours);
                    Visit(graph, reporter, furtherChoice, furtherChoice,
                          neighbouringCandidates, neighbouringExcluded,
                          CollectionsUtil.Append(cliqueInProgress, v));
                }
                else if (CollectionsUtil.AreDisjoint(excluded, neighbours))
                {
                    reporter.Record(CollectionsUtil.Append(cliqueInProgress, v));
                }
                excluded.Add(v);
            }
        }

        private static Vertex Choose(Choice choice, ISet<Vertex> candidates, UndirectedGraph graph)
        {
            return choice switch
            {
                Choice.MaxDegree => candidates.OrderByDescending(graph.Degree).First(),
                _ => throw new ArgumentException("implemented differently")
            };
        }
    }
}
