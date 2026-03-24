// Bron-Kerbosch algorithm with pivot picked arbitrarily


namespace BronKerbosch

open System.Diagnostics

type PivotChoice =
    | MaxDegreeLocal
    | MaxDegreeLocalX

module PivotBased =
    let harvest
        (
            neighbours: VertexSet,
            excluded: VertexSet,
            v: Vertex,
            clique_in_progress: Vertex list,
            consumer: CliqueConsumer
        ) =
        if
            1 + clique_in_progress.Length >= consumer.MinSize
            && VertexSet.is_disjoint neighbours excluded
        then
            let clique = Array.ofList (v :: clique_in_progress)
            consumer.accept clique

    let rec visit
        (pivot_choice: PivotChoice)
        (graph: UndirectedGraph)
        (consumer: CliqueConsumer)
        (candidates: VertexSet, excluded: byref<VertexSet>, clique_in_progress: Vertex list)
        : Unit =
        Debug.Assert(candidates |> Seq.forall graph.hasNeighbours)
        Debug.Assert(excluded |> Seq.forall graph.hasNeighbours)
        Debug.Assert(VertexSet.is_disjoint candidates excluded)
        Debug.Assert(candidates.Any)

        match candidates |> Seq.tryExactlyOne with
        | Some(v) ->
            // Same logic as below, stripped down
            let neighbours = graph.neighbours v
            harvest (neighbours, excluded, v, clique_in_progress, consumer)
        | None ->
            let mutable pivot: Vertex option = None
            let mutable remaining_candidates: Vertex list = []
            // Quickly handle locally unconnected candidates while finding pivot
            let mutable seen_local_degree = 0

            for v in candidates do
                let neighbours = graph.neighbours v
                let local_degree = VertexSet.overlap neighbours candidates

                if local_degree = 0 then
                    // Same logic as below, stripped down
                    harvest (neighbours, excluded, v, clique_in_progress, consumer)
                else
                    if seen_local_degree < local_degree then
                        seen_local_degree <- local_degree
                        pivot <- Some v

                    remaining_candidates <- v :: remaining_candidates

            if seen_local_degree > 0 then
                Debug.Assert(pivot.IsSome)

                if pivot_choice = PivotChoice.MaxDegreeLocalX then
                    for v in excluded do
                        let neighbours = graph.neighbours v
                        let local_degree = VertexSet.overlap neighbours candidates

                        if seen_local_degree < local_degree then
                            seen_local_degree <- local_degree
                            pivot <- Some v

                let mutable candidates = candidates
                let mutable excluded = excluded

                for v in remaining_candidates do
                    let neighbours = graph.neighbours v
                    Debug.Assert neighbours.Any

                    if not (neighbours.Contains pivot.Value) then
                        Debug.Assert(candidates.Contains(v))
                        VertexSet.remove_mutably (&candidates, v)
                        let neighbouringCandidates = VertexSet.intersect neighbours candidates

                        if neighbouringCandidates.Any then
                            let mutable neighbouringExcluded = VertexSet.intersect neighbours excluded

                            visit
                                pivot_choice
                                graph
                                consumer
                                (neighbouringCandidates, &neighbouringExcluded, v :: clique_in_progress)
                        else
                            harvest (neighbours, excluded, v, clique_in_progress, consumer)

                        VertexSet.insert_mutably (&excluded, v)

    let explore (pivot_choice: PivotChoice) (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
        match graph.MaxDegreeVertices() |> Seq.tryHead with
        | None -> ()
        | Some(pivot) ->
            // In this initial iteration, we don't need to represent the set of candidates
            // because all neighbours are candidates until excluded.
            let mutable excluded = VertexSet.new_mutable (graph.Order)

            for v in graph.ConnectedVertices() do
                let neighbours = graph.neighbours v

                if not (neighbours.Contains(pivot)) then
                    let mutable neighbouringExcluded = VertexSet.intersect neighbours excluded
                    let neighbouringCandidates = VertexSet.difference neighbours neighbouringExcluded

                    if neighbouringCandidates.Any then
                        visit pivot_choice graph consumer (neighbouringCandidates, &neighbouringExcluded, [ v ])

                    VertexSet.insert_mutably (&excluded, v)
