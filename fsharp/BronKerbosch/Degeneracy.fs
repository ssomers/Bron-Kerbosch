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
            let q = PriorityQueue.empty graph.MaxDegree
            let mutable leftToPick = FortifiedCounter.empty ()

            for v in graph.ConnectedVertices() do
                match graph.degree v with
                | 0 -> ()
                | priority ->
                    priorityPerVertex[v.index] <- priority
                    q.Put(priority, v)
                    leftToPick.Add v

            while leftToPick.count > 0 do
                let pick =
                    match q.Pop() with
                    | None -> failwith "Cannot pop more than was put"
                    | Some(p) -> p

                if priorityPerVertex[pick.index] > 0 then
                    priorityPerVertex[pick.index] <- 0
                    leftToPick.Remove pick

                    let mutable neighboursPicked = Set.empty

                    for v in graph.neighbours pick do
                        match priorityPerVertex[v.index] with
                        | 0 -> neighboursPicked <- neighboursPicked.Add(v)
                        | old_priority ->
                            Debug.Assert(q.Contains(old_priority, v))
                            Debug.Assert(leftToPick.Contains(v))
                            let new_priority = old_priority - 1
                            priorityPerVertex[v.index] <- new_priority

                            if new_priority > 0 then
                                // Requeue with a more urgent priority.
                                q.Put(new_priority, v)
                            else
                                leftToPick.Remove v

                    yield (pick, neighboursPicked)
        }
