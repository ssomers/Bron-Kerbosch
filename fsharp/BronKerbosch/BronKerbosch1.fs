// Naïve Bron-Kerbosch algorithm
module BronKerbosch1

open BronKerbosch

let rec visit
    (graph: UndirectedGraph)
    (consumer: CliqueConsumer)
    (candidates: Set<Vertex>, excluded: Set<Vertex>, cliqueInProgress: Vertex array)
    : Unit =
    assert (candidates |> Set.forall graph.HasNeighbours)
    assert (excluded |> Set.forall graph.HasNeighbours)
    assert (Set.intersect candidates excluded).IsEmpty

    if not (candidates.IsEmpty) then
        let v, candidates = VertexSet.pop_arbitrary (candidates)
        let neighbours = graph.Neighbours(v)
        let neighbouringCandidates = Set.intersect candidates neighbours

        if not (neighbouringCandidates.IsEmpty) then
            let neighbouringExcluded: Set<Vertex> = Set.intersect excluded neighbours
            let cliqueInProgress = Array.append cliqueInProgress [| v |]

            visit graph consumer (neighbouringCandidates, neighbouringExcluded, cliqueInProgress)
        else if not (VertexSet.overlaps (excluded, neighbours)) then
            let clique = Array.append cliqueInProgress [| v |]
            consumer clique

        assert not (excluded.Contains(v))
        let excluded = excluded.Add(v)
        visit graph consumer (candidates, excluded, cliqueInProgress)

let public explore (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
    let candidates = graph.ConnectedVertices() |> Set.ofSeq
    let excluded = Set.empty // EmptyWithCapacity(candidates.Count)
    visit graph consumer (candidates, excluded, [||])
