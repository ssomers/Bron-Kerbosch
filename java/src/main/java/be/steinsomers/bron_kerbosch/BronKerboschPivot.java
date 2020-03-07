package be.steinsomers.bron_kerbosch;

import lombok.NonNull;

import java.util.Collection;
import java.util.HashSet;
import java.util.Set;
import java.util.function.Consumer;
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
        var spliterator = new BronKerboschSpliterator(worker);
        return StreamSupport.stream(spliterator, false);
    }

    static final class Worker implements BronKerboschSpliterator.Generator {
        private final @NonNull UndirectedGraph graph;
        private final PivotChoice furtherPivotChoice;
        private final Set<Integer> mut_candidates;
        private final Set<Integer> mut_excluded;
        private final int[] vertices;
        private int endIndexOfCliqueCompleting;
        private int firstIndexOfGenuineCandidate;
        private int pivot = -1;

        Worker(final @NonNull UndirectedGraph graph,
               final PivotChoice initialPivotChoice,
               final PivotChoice furtherPivotChoice,
               @NonNull Set<Integer> mut_candidates,
               @NonNull Set<Integer> mut_excluded) {
            assert mut_candidates.stream().allMatch(v -> graph.degree(v) > 0);
            assert mut_excluded.stream().allMatch(v -> graph.degree(v) > 0);
            assert util.AreDisjoint(mut_candidates, mut_excluded);

            this.graph = graph;
            this.furtherPivotChoice = furtherPivotChoice;
            this.mut_candidates = mut_candidates;
            this.mut_excluded = mut_excluded;
            final var numCandidates = mut_candidates.size();
            if (numCandidates <= 1) {
                // Same logic as below, stripped down for this common case
                vertices = mut_candidates.stream().mapToInt(i -> i).toArray();
                firstIndexOfGenuineCandidate = numCandidates;
                if (numCandidates == 0) {
                    endIndexOfCliqueCompleting = 0;
                } else {
                    var v = vertices[0];
                    var neighbours = graph.neighbours(v);
                    if (util.AreDisjoint(neighbours, mut_excluded)) {
                        endIndexOfCliqueCompleting = 1;
                    } else {
                        endIndexOfCliqueCompleting = 0;
                    }
                }
            } else if (initialPivotChoice == PivotChoice.Arbitrary) {
                vertices = mut_candidates.stream().mapToInt(i -> i).toArray();
                endIndexOfCliqueCompleting = 0;
                firstIndexOfGenuineCandidate = 0;
                pivot = vertices[0];
            } else if (initialPivotChoice == PivotChoice.MaxDegree) {
                vertices = mut_candidates.stream().mapToInt(i -> i).toArray();
                endIndexOfCliqueCompleting = 0;
                firstIndexOfGenuineCandidate = 0;
                pivot = mut_candidates.stream()
                        .max((v, w) -> graph.degree(v) > graph.degree(w) ? v : w)
                        .orElseThrow();
            } else { // PivotChoice.MaxDegreeLocal | PivotChoice.MaxDegreeLocalX
                // Quickly handle locally unconnected candidates while finding pivot
                vertices = new int[numCandidates];
                endIndexOfCliqueCompleting = 0;
                firstIndexOfGenuineCandidate = numCandidates;
                long seenLocalDegree = 0;
                for (var v : mut_candidates) {
                    var neighbours = graph.neighbours(v);
                    long localDegree = util.Intersect(neighbours, mut_candidates).count();
                    if (localDegree == 0) {
                        // Same logic as below, stripped down
                        if (util.AreDisjoint(neighbours, mut_excluded)) {
                            vertices[endIndexOfCliqueCompleting++] = v;
                        }
                    } else {
                        if (seenLocalDegree < localDegree) {
                            seenLocalDegree = localDegree;
                            pivot = v;
                        }
                        vertices[--firstIndexOfGenuineCandidate] = v;
                    }
                }
                if (initialPivotChoice == PivotChoice.MaxDegreeLocalX
                        && firstIndexOfGenuineCandidate < numCandidates) {
                    for (var v : mut_excluded) {
                        var neighbours = graph.neighbours(v);
                        var localDegree = util.Intersect(neighbours, mut_candidates).count();
                        if (seenLocalDegree < localDegree) {
                            seenLocalDegree = localDegree;
                            pivot = v;
                        }
                    }
                }
            }
        }

        public boolean findNextVertex(Consumer<Integer> cliqueConsumer,
                                      BronKerboschSpliterator.VertexVisitQueue recursionQueue) {
            if (endIndexOfCliqueCompleting > 0) {
                assert cliqueConsumer != null;
                var v = vertices[--endIndexOfCliqueCompleting];
                cliqueConsumer.accept(v);
                return true;
            }
            while (firstIndexOfGenuineCandidate < vertices.length) {
                var v = vertices[firstIndexOfGenuineCandidate++];
                var neighbours = graph.neighbours(v);
                if (!neighbours.contains(pivot)) {
                    mut_candidates.remove(v);
                    mut_excluded.add(v);
                    var neighbouringCandidates = util.Intersect(mut_candidates, neighbours)
                            .collect(Collectors.toCollection(HashSet::new));
                    if (neighbouringCandidates.isEmpty()) {
                        if (cliqueConsumer == null) {
                            assert !util.AreDisjoint(neighbours, mut_excluded);
                        } else if (util.AreDisjoint(neighbours, mut_excluded)) {
                            cliqueConsumer.accept(v);
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
                        recursionQueue.offer(v, subWorker);
                    }
                }
            }
            return false;
        }
    }
}
