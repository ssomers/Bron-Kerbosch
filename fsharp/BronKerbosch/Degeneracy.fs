namespace BronKerbosch

open System.Diagnostics

type public Degeneracy =
    { graph: UndirectedGraph
      // Possible values of priorityPerVertex (after initialization):
      //   0: never queued because not connected (degree 0),
      //      or no longer queued because it has been yielded itself,
      //      or no longer queued because all neighbours have been yielded
      //   1 or more: candidates queued with priority (degree - #of yielded neighbours)
      mutable priorityPerVertex: Priority array
      mutable q: PriorityQueue<Vertex>
      mutable leftToPick: FortifiedCounter<Vertex> }

    static member New(graph: UndirectedGraph) : Degeneracy =
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

        { graph = graph
          priorityPerVertex = priorityPerVertex
          q = q
          leftToPick = leftToPick }


    // Enumerate connected vertices in degeneracy order, skipping vertices
    // whose neighbours have all been enumerated already.
    member this.iter() : Vertex seq =
        seq {
            while this.leftToPick.count > 0 do
                let pick =
                    match PriorityQueue.Pop &this.q with
                    | None -> failwith "Cannot pop more than was put"
                    | Some(p) -> p

                if this.priorityPerVertex[pick.index] > 0 then
                    this.priorityPerVertex[pick.index] <- 0
                    FortifiedCounter.Remove(&this.leftToPick, pick)
                    yield pick

                    let neighbours = this.graph.neighbours pick

                    for v in neighbours do
                        match this.priorityPerVertex[v.index] with
                        | 0 -> ()
                        | old_priority ->
                            Debug.Assert(this.q.Contains(old_priority, v))
                            Debug.Assert(this.leftToPick.Contains(v))
                            let new_priority = old_priority - 1
                            this.priorityPerVertex[v.index] <- new_priority

                            if new_priority > 0 then
                                // Requeue with a more urgent priority.
                                PriorityQueue.Put(&this.q, new_priority, v)
                            else
                                // We discount this neighbour already, but logically it will
                                // be (silently) picked only after we yield the current pick.
                                // So it does not belong in the current pickedNeighbours.
                                FortifiedCounter.Remove(&this.leftToPick, v)
        }

    member this.isCandidate(v: Vertex) : bool = this.priorityPerVertex[v.index] > 0
