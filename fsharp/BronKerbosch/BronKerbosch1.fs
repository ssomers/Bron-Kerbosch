// Naïve Bron-Kerbosch algorithm
module BronKerbosch1

open BronKerbosch
open System.Diagnostics

let rec visit
    (graph: UndirectedGraph)
    (consumer: CliqueConsumer)
    (candidates: byref<VertexSet>, excluded: byref<VertexSet>, clique_in_progress: Vertex list)
    : Unit =
    Debug.Assert(candidates |> Seq.forall graph.hasNeighbours)
    Debug.Assert(excluded |> Seq.forall graph.hasNeighbours)
    Debug.Assert(VertexSet.is_disjoint candidates excluded)

    match VertexSet.pop_arbitrary_mutably &candidates with
    | None -> ()
    | Some v ->
        let neighbours = graph.neighbours v
        let mutable neighbouring_candidates = VertexSet.intersect neighbours candidates

        if neighbouring_candidates.Any then
            let mutable neighbouring_excluded = VertexSet.intersect neighbours excluded

            visit graph consumer (&neighbouring_candidates, &neighbouring_excluded, v :: clique_in_progress)
        elif
            1 + clique_in_progress.Length >= consumer.MinSize
            && VertexSet.is_disjoint neighbours excluded
        then
            let clique = Array.ofList (v :: clique_in_progress)
            consumer.accept clique

        Debug.Assert(not (excluded.Contains v))
        VertexSet.insert_mutably (&excluded, v)
        visit graph consumer (&candidates, &excluded, clique_in_progress)

let public explore (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
    let mutable candidates = graph.ConnectedVertices() |> VertexSet.ofSeq
    let mutable excluded = VertexSet.new_mutable (VertexSet.count candidates)
    visit graph consumer (&candidates, &excluded, [])

let algorithm: Algorithm = { name = "Ver1½"; exec = explore }
