// Bron-Kerbosch algorithm with pivot picked arbitrarily

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using Vertex = System.UInt32;

namespace BronKerbosch
{
    public class Pivot
    {
        public enum Choice
        {
            MaxDegree, MaxDegreeLocal, MaxDegreeLocalX
        };

        public static void Visit(UndirectedGraph graph, Reporter reporter,
                                 Choice initialChoice, Choice furtherChoice,
                                 ISet<Vertex> candidates, ISet<Vertex> excluded, List<Vertex> clique)
        {
            Debug.Assert(candidates.Any());
            Debug.Assert(candidates.All(v => graph.Degree(v) > 0));
            Debug.Assert(excluded.All(v => graph.Degree(v) > 0));
            Debug.Assert(!candidates.Overlaps(excluded));

            if (candidates.Count == 1)
            {
                // Same logic as below, stripped down
                Vertex v = candidates.First();
                var neighbours = graph.Neighbours(v);
                if (Util.AreDisjoint(excluded, neighbours))
                    reporter.Record(new List<Vertex>(clique) { v });
                return;
            }

            Vertex pivot;
            var remainingCandidates = new Vertex[candidates.Count];
            var remainingCandidateCount = 0;
            if (initialChoice >= Choice.MaxDegreeLocal)
            {
                // Quickly handle locally unconnected candidates while finding pivot
                pivot = Vertex.MaxValue;
                var seenLocalDegree = 0;
                foreach (Vertex v in candidates)
                {
                    var neighbours = graph.Neighbours(v);
                    var localDegree = Util.IntersectCount(neighbours, candidates);
                    if (localDegree == 0)
                    {
                        // Same logic as below, stripped down
                        if (Util.AreDisjoint(excluded, neighbours))
                            reporter.Record(new List<Vertex>(clique) { v });
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
                Debug.Assert(pivot != Vertex.MaxValue);
                if (initialChoice == Choice.MaxDegreeLocalX)
                {
                    foreach (Vertex v in excluded)
                    {
                        var neighbours = graph.Neighbours(v);
                        var localDegree = Util.IntersectCount(neighbours, candidates);
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
            for (int i = 0; i < remainingCandidateCount; ++i)
            {
                var v = remainingCandidates[i];
                var neighbours = graph.Neighbours(v);
                if (neighbours.Contains(pivot))
                    continue;
                candidates.Remove(v);
                var neighbouringCandidates = Util.Intersect(candidates, neighbours);
                if (neighbouringCandidates.Any())
                {
                    var neighbouringExcluded = Util.Intersect(excluded, neighbours);
                    Visit(graph, reporter, furtherChoice, furtherChoice,
                          neighbouringCandidates, neighbouringExcluded,
                          new List<Vertex>(clique) { v });
                }
                else
                {
                    if (Util.AreDisjoint(excluded, neighbours))
                        reporter.Record(new List<Vertex>(clique) { v });
                }
                excluded.Add(v);
            }
        }

        private static Vertex Choose(Choice choice, ISet<Vertex> candidates, UndirectedGraph graph)
        {
            switch (choice)
            {
                case Choice.MaxDegree: return candidates.OrderByDescending(v => graph.Degree(v)).First();
                default: throw new ArgumentException("implemented differently");
            }
        }
    }
}