// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot.

namespace BronKerbosch

open System.Diagnostics

module DegeneracyBased =
    let explore (pivot_choice: PivotChoice) (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        let degeneracy = Degeneracy.New graph

        for v in degeneracy.iter () do
            let mutable (neighbouringCandidates, neighbouringExcluded) =
                graph.neighbours v |> VertexSet.partition degeneracy.isCandidate

            PivotBased.visit pivot_choice graph consumer (neighbouringCandidates, &neighbouringExcluded, [ v ])
