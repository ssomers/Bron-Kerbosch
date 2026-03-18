// Naïve Bron-Kerbosch algorithm
module BronKerbosch1

open BronKerbosch
open System.Diagnostics

let rec visit
    (graph: UndirectedGraph)
    (consumer: CliqueConsumer)
    (candidates: VertexSet, excluded: VertexSet, clique_in_progress: Vertex list)
    : Unit =
    Debug.Assert(Set.forall graph.hasNeighbours candidates)
    Debug.Assert(Set.forall graph.hasNeighbours excluded)
    Debug.Assert(VertexSet.is_disjoint candidates excluded)

    match VertexSet.pop_arbitrary (candidates) with
    | (None, _) -> ()
    | (Some v, remaining_candidates) ->
        let neighbours = graph.neighbours v
        let neighbouring_candidates = VertexSet.intersect neighbours remaining_candidates

        if not neighbouring_candidates.IsEmpty then
            let neighbouring_excluded = VertexSet.intersect neighbours excluded

            visit graph consumer (neighbouring_candidates, neighbouring_excluded, v :: clique_in_progress)
        elif
            1 + clique_in_progress.Length >= consumer.MinSize
            && VertexSet.is_disjoint neighbours excluded
        then
            let clique = Array.ofList (v :: clique_in_progress)
            consumer.accept clique

        Debug.Assert(not (excluded.Contains v))
        visit graph consumer (remaining_candidates, excluded.Add(v), clique_in_progress)

let public explore (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
    let candidates = graph.ConnectedVertices() |> Set.ofSeq
    let excluded = Set.empty // EmptyWithCapacity(candidates.Count)
    visit graph consumer (candidates, excluded, [])

let algorithm: Algorithm = { name = "Ver1½"; exec = explore }
