// Bron-Kerbosch algorithm with pivot picked arbitrarily


namespace BronKerbosch

open System.Diagnostics

type PivotChoice =
    | MaxDegreeLocal
    | MaxDegreeLocalX

module PivotBased =
    let rec visit
        (pivot_choice: PivotChoice)
        (graph: UndirectedGraph)
        (consume: CliqueConsumer)
        (candidates: Set<Vertex>, excluded: Set<Vertex>, clique_in_progress: Clique)
        : Unit =
        Debug.Assert(Set.forall graph.hasNeighbours candidates)
        Debug.Assert(Set.forall graph.hasNeighbours excluded)
        Debug.Assert(VertexSet.is_disjoint candidates excluded)
        Debug.Assert(candidates.Count >= 1)

        match candidates |> Seq.tryExactlyOne with
        | Some(v) ->
            // Same logic as below, stripped down
            let neighbours = graph.neighbours v

            if VertexSet.is_disjoint neighbours excluded then
                let clique = v :: clique_in_progress
                consume clique
        | None ->
            let mutable pivot: Vertex option = None
            let mutable remaining_candidates: Vertex list = []
            // Quickly handle locally unconnected candidates while finding pivot
            let mutable seen_local_degree = 0

            candidates
            |> Set.iter (fun v ->
                let neighbours = graph.neighbours v
                let local_degree = VertexSet.intersection_size neighbours candidates

                if local_degree = 0 then
                    // Same logic as below, stripped down
                    if VertexSet.is_disjoint neighbours excluded then
                        let clique = v :: clique_in_progress
                        consume clique
                else
                    if seen_local_degree < local_degree then
                        seen_local_degree <- local_degree
                        pivot <- Some v

                    remaining_candidates <- v :: remaining_candidates)

            if seen_local_degree > 0 then
                Debug.Assert(pivot.IsSome)

                if pivot_choice = PivotChoice.MaxDegreeLocalX then
                    excluded
                    |> Set.iter (fun v ->
                        let neighbours = graph.neighbours v
                        let local_degree = VertexSet.intersection_size neighbours candidates

                        if seen_local_degree < local_degree then
                            seen_local_degree <- local_degree
                            pivot <- Some v)

                let mutable candidates = candidates
                let mutable excluded = excluded

                remaining_candidates
                |> List.iter (fun v ->
                    let neighbours = graph.neighbours v
                    Debug.Assert(not (neighbours.IsEmpty))

                    if not (neighbours.Contains pivot.Value) then
                        Debug.Assert(candidates.Contains(v))
                        candidates <- candidates.Remove(v)
                        let neighbouringCandidates = VertexSet.intersect neighbours candidates

                        if not (neighbouringCandidates.IsEmpty) then
                            let neighbouringExcluded = VertexSet.intersect neighbours excluded

                            visit
                                pivot_choice
                                graph
                                consume
                                (neighbouringCandidates, neighbouringExcluded, v :: clique_in_progress)
                        elif VertexSet.is_disjoint neighbours excluded then
                            let clique = v :: clique_in_progress
                            consume clique

                        excluded <- excluded.Add v)

    let explore (pivot_choice: PivotChoice) (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
        match graph.MaxDegreeVertices() |> Seq.tryHead with
        | None -> ()
        | Some(pivot) ->
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            let mutable excluded = Set.empty // EmptyWithCapacity(graph.order)

            graph.Vertices()
            |> Seq.iter (fun v ->
                let neighbours = graph.neighbours v

                if not (neighbours.IsEmpty || neighbours.Contains(pivot)) then
                    let neighbouringExcluded = VertexSet.intersect neighbours excluded

                    if neighbouringExcluded.Count < neighbours.Count then
                        let neighbouringCandidates = VertexSet.difference neighbours neighbouringExcluded
                        visit pivot_choice graph consumer (neighbouringCandidates, neighbouringExcluded, [ v ])

                    excluded <- excluded.Add(v))
