namespace BronKerbosch

module Degeneracy =

    type Priority = int

    // Enumerate connected vertices in degeneracy order, skipping vertices
    // whose neighbours have all been enumerated already.
    let iter (graph: UndirectedGraph) : Vertex seq =
        seq {
            // Possible values of priorityPerVertex (after initialization):
            //   0: never queued because not connected (degree 0),
            //      or no longer queued because it has been yielded itself,
            //      or no longer queued because all neighbours have been yielded
            //   1 or more: candidates queued with priority (degree - #of yielded neighbours)
            let mutable priorityPerVertex: Priority array = Array.zeroCreate (graph.Order)
            let q = PriorityQueue.create<Vertex> (graph.MaxDegree)

            graph.ConnectedVertices()
            |> Seq.iter (fun v ->
                let degree = graph.degree (v)
                priorityPerVertex[v.index] <- degree
                q.Insert(v, degree))

            while not q.Empty do
                let pick = q.Pop()
                let priority = priorityPerVertex[pick.index]

                if priority > 0 then
                    yield pick

                    priorityPerVertex[pick.index] <- 0
                    q.Forget(pick)

                    graph.neighbours (pick)
                    |> Seq.iter (fun v ->
                        let old_priority = priorityPerVertex[v.index]

                        if old_priority > 0 then
                            let new_priority = q.Promote(v, old_priority)
                            priorityPerVertex[v.index] <- new_priority)

        }
