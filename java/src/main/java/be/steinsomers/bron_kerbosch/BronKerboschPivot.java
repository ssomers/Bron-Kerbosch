package be.steinsomers.bron_kerbosch;

import lombok.NonNull;

import java.util.Collection;
import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

class BronKerboschPivot implements BronKerboschAlgorithm {
    private final PivotChoice itsInitialPivotChoice;
    private final PivotChoice itsFurtherPivotChoice;

    BronKerboschPivot(PivotChoice initialPivotChoice,
                      PivotChoice furtherPivotChoice) {
        itsInitialPivotChoice = initialPivotChoice;
        itsFurtherPivotChoice = furtherPivotChoice;
    }

    @Override
    public final Stream<int[]> explore(UndirectedGraph graph) {
        Set<Integer> candidates = graph.connectedVertices()
                .collect(Collectors.toCollection(HashSet::new));
        Set<Integer> excluded = new HashSet<>(candidates.size());
        var worker = new Worker(
                graph,
                itsInitialPivotChoice,
                itsFurtherPivotChoice,
                candidates,
                excluded);
        var spliterator = new BronKerboschSpliterator(-1, worker);
        return StreamSupport.stream(spliterator, true);
    }

    static final class Worker implements BronKerboschSpliterator.Generator {
        private final @NonNull UndirectedGraph graph;
        private final PivotChoice furtherPivotChoice;
        private final Set<Integer> mut_candidates;
        private final Set<Integer> mut_excluded;
        private final int[] vertices;
        private int endIndexCliqueCompleting;
        private int firstCandidateIndex;
        private int pivot = -1;

        Worker(final @NonNull UndirectedGraph graph,
               final PivotChoice initialPivotChoice,
               final PivotChoice furtherPivotChoice,
               @NonNull Set<Integer> candidates,
               @NonNull Set<Integer> excluded) {
            assert candidates.stream().allMatch(v -> graph.degree(v) > 0);
            assert excluded.stream().allMatch(v -> graph.degree(v) > 0);
            assert util.AreDisjoint(candidates, excluded);

            this.graph = graph;
            this.furtherPivotChoice = furtherPivotChoice;
            mut_candidates = candidates;
            mut_excluded = excluded;
            final var numCandidates = candidates.size();
            if (numCandidates <= 1) {
                // Same logic as below, stripped down for this common case
                vertices = candidates.stream().mapToInt(i -> i).toArray();
                firstCandidateIndex = numCandidates;
                if (numCandidates == 0) {
                    endIndexCliqueCompleting = 0;
                } else {
                    var v = vertices[0];
                    var neighbours = graph.neighbours(v);
                    if (util.AreDisjoint(neighbours, excluded)) {
                        endIndexCliqueCompleting = 1;
                    } else {
                        endIndexCliqueCompleting = 0;
                    }
                }
            } else switch (initialPivotChoice) {
                case Arbitrary:
                    vertices = candidates.stream().mapToInt(i -> i).toArray();
                    firstCandidateIndex = 0;
                    endIndexCliqueCompleting = 0;
                    pivot = vertices[0];
                    break;
                case MaxDegree:
                    vertices = candidates.stream().mapToInt(i -> i).toArray();
                    firstCandidateIndex = 0;
                    endIndexCliqueCompleting = 0;
                    pivot = candidates.stream()
                            .max((v, w) -> graph.degree(v) > graph.degree(w) ? v : w)
                            .orElseThrow();
                    break;
                case MaxDegreeLocal:
                case MaxDegreeLocalX: {
                    // Quickly handle locally unconnected candidates while finding pivot
                    vertices = new int[numCandidates];
                    endIndexCliqueCompleting = 0;
                    firstCandidateIndex = numCandidates;
                    long seenLocalDegree = 0;
                    for (var v : candidates) {
                        var neighbours = graph.neighbours(v);
                        long localDegree = util.Intersect(neighbours, candidates).count();
                        if (localDegree == 0) {
                            // Same logic as below, stripped down
                            if (util.AreDisjoint(neighbours, mut_excluded)) {
                                vertices[endIndexCliqueCompleting++] = v;
                            }
                        } else {
                            if (seenLocalDegree < localDegree) {
                                seenLocalDegree = localDegree;
                                pivot = v;
                            }
                            vertices[--firstCandidateIndex] = v;
                        }
                    }
                    if (initialPivotChoice == PivotChoice.MaxDegreeLocalX
                            && firstCandidateIndex < numCandidates) {
                        for (var v : mut_excluded) {
                            var neighbours = graph.neighbours(v);
                            var localDegree = util.Intersect(neighbours, candidates).count();
                            if (seenLocalDegree < localDegree) {
                                seenLocalDegree = localDegree;
                                pivot = v;
                            }
                        }
                    }
                    break;
                }
                default:
                    throw new IndexOutOfBoundsException(initialPivotChoice.toString());
            }
        }

        public boolean findNextVertex(BronKerboschSpliterator.VtxConsumer consumer) {
            if (endIndexCliqueCompleting > 0) {
                var v = vertices[--endIndexCliqueCompleting];
                consumer.acceptClique(v);
                return true;
            }
            while (firstCandidateIndex < vertices.length) {
                var v = vertices[firstCandidateIndex++];
                var neighbours = graph.neighbours(v);
                if (!neighbours.contains(pivot)) {
                    mut_candidates.remove(v);
                    mut_excluded.add(v);
                    var neighbouringCandidates = util.Intersect(mut_candidates, neighbours)
                            .collect(Collectors.toCollection(HashSet::new));
                    if (neighbouringCandidates.isEmpty()) {
                        if (util.AreDisjoint(neighbours, mut_excluded)) {
                            consumer.acceptClique(v);
                            return true;
                        }
                    } else {
                        var neighbouringExcluded = util.Intersect(mut_excluded, neighbours)
                                .collect(Collectors.toCollection(HashSet::new));
                        var subWorker = new Worker(
                                graph,
                                furtherPivotChoice,
                                furtherPivotChoice,
                                neighbouringCandidates,
                                neighbouringExcluded);
                        consumer.diveDeeper(v, subWorker);
                        return true;
                    }
                }
            }
            return false;
        }
    }
}
