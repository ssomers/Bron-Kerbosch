// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot.

namespace BronKerbosch

open System.Diagnostics

module DegeneracyBased =
    let explore (pivot_choice: PivotChoice) (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        for v, neighboursPicked in Degeneracy.iter graph do
            let mutable neighbouring_excluded = neighboursPicked
            let neighbours = graph.neighbours v
            Debug.Assert neighbours.Any

            let mutable neighbouring_candidates =
                VertexSet.difference neighbours neighbouring_excluded

            if neighbouring_candidates.Any then
                PivotBased.visit pivot_choice graph consumer (neighbouring_candidates, &neighbouring_excluded, [ v ])
