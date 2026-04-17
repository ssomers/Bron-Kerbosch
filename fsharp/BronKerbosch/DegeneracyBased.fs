// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot.

namespace BronKerbosch

open System.Diagnostics

module DegeneracyBased =
    let explore (pivot_choice: PivotChoice) (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
        let degeneracy = Degeneracy.Of graph

        for v in degeneracy.iter () do
            let mutable (neighbouringCandidates, neighbouringExcluded) =
                graph.neighbours v |> VertexSet.partition degeneracy.isCandidate

            PivotBased.visit pivot_choice graph consumer (neighbouringCandidates, &neighbouringExcluded, [ v ])
