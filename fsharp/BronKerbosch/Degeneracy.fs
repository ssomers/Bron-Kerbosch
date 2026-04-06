namespace BronKerbosch

open System.Diagnostics

module Degeneracy =
    // Enumerate connected vertices in degeneracy order, skipping vertices
    // whose neighbours have all been enumerated already.
    let iter (graph: UndirectedGraph) : (Vertex * VertexSet) seq =
        seq {
            // Possible values of priorityPerVertex (after initialization):
            //   0: never queued because not connected (degree 0),
            //      or no longer queued because it has been yielded itself,
            //      or no longer queued because all neighbours have been yielded
            //   1 or more: candidates queued with priority (degree - #of yielded neighbours)
            let mutable priorityPerVertex: Priority array = Array.create graph.Order 0
            let mutable q = PriorityQueue.init graph.MaxDegree
            let mutable leftToPick = FortifiedCounter.init ()

            for v in graph.ConnectedVertices() do
                match graph.degree v with
                | 0 -> ()
                | priority ->
                    priorityPerVertex[v.index] <- priority
                    PriorityQueue.Put(&q, priority, v)
                    FortifiedCounter.Add(&leftToPick, v)

            while leftToPick.count > 0 do
                let pick =
                    match PriorityQueue.Pop &q with
                    | None -> failwith "Cannot pop more than was put"
                    | Some(p) -> p

                if priorityPerVertex[pick.index] > 0 then
                    priorityPerVertex[pick.index] <- 0
                    FortifiedCounter.Remove(&leftToPick, pick)

                    let neighbours = graph.neighbours pick
                    let mutable pickedNeighbours = VertexSet.new_mutable (VertexSet.count neighbours)

                    for v in neighbours do
                        match priorityPerVertex[v.index] with
                        | 0 -> VertexSet.insert_mutably (&pickedNeighbours, v)
                        | old_priority ->
                            Debug.Assert(q.Contains(old_priority, v))
                            Debug.Assert(leftToPick.Contains(v))
                            let new_priority = old_priority - 1
                            priorityPerVertex[v.index] <- new_priority

                            if new_priority > 0 then
                                // Requeue with a more urgent priority.
                                PriorityQueue.Put(&q, new_priority, v)
                            else
                                // We discount this neighbour already, but logically it will
                                // be (silently) picked only after we yield the current pick.
                                // So it does not belong in the current pickedNeighbours.
                                FortifiedCounter.Remove(&leftToPick, v)

                    Debug.Assert(VertexSet.count pickedNeighbours < graph.degree pick)
                    yield (pick, pickedNeighbours)
        }
