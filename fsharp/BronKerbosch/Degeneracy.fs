namespace BronKerbosch

open System.Diagnostics

module Degeneracy =
    // Enumerate connected vertices in degeneracy order, skipping vertices
    // whose neighbours have all been enumerated already.
    let iter (graph: UndirectedGraph) : (Vertex * Set<Vertex>) seq =
        seq {
            // Possible values of priorityPerVertex (after initialization):
            //   0: never queued because not connected (degree 0),
            //      or no longer queued because it has been yielded itself,
            //      or no longer queued because all neighbours have been yielded
            //   1 or more: candidates queued with priority (degree - #of yielded neighbours)
            let mutable priorityPerVertex: Priority array = Array.create graph.Order 0
            let q = PriorityQueueWithDequeued.create graph.MaxDegree
            let mutable left_to_pick = FortifiedCounter.create ()

            graph.ConnectedVertices()
            |> Seq.iter (fun v ->
                match graph.degree v with
                | 0 -> ()
                | priority ->
                    priorityPerVertex[v.index] <- priority
                    q.Push(v, priority)
                    left_to_pick.Add v)

            while left_to_pick.count > 0 do
                let pick = q.Pick()
                let picked_earlier = q.dequeued
                yield (pick, picked_earlier)
                priorityPerVertex[pick.index] <- 0
                q.Dequeue(pick)
                left_to_pick.Remove pick

                graph.neighbours pick
                |> Seq.iter (fun v ->
                    match priorityPerVertex[v.index] with
                    | 0 -> ()
                    | old_priority ->
                        Debug.Assert(q.IsQueued(v, old_priority))
                        Debug.Assert(left_to_pick.Contains(v))
                        let new_priority = old_priority - 1
                        priorityPerVertex[v.index] <- new_priority

                        if new_priority > 0 then
                            // Requeue with a more urgent priority.
                            q.Push(v, new_priority)
                        else
                            q.Dequeue v
                            left_to_pick.Remove v)

        }
