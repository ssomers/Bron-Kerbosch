// Bron-Kerbosch algorithm with degeneracy ordering,
// parametrized by the way nested searches choose a pivot.

namespace BronKerbosch

open System.Diagnostics

module DegeneracyBased =
    let explore (pivot_choice: PivotChoice) (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
        // In this initial iteration, we don't need to represent the set of candidates
        // because all neighbours are candidates until excluded.
        Degeneracy.iter graph
        |> Seq.iter (fun (v, excluded) ->
            let neighbours = graph.neighbours v
            Debug.Assert(not (neighbours.IsEmpty))
            let neighbouring_excluded = VertexSet.intersect excluded neighbours

            if neighbouring_excluded.Count < neighbours.Count then
                let neighbouring_candidates = VertexSet.difference neighbours neighbouring_excluded
                PivotBased.visit pivot_choice graph consumer (neighbouring_candidates, neighbouring_excluded, [ v ]))
