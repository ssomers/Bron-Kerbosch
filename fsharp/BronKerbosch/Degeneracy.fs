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
      mutable queue: PriorityQueue<Vertex>
      mutable leftToPick: FortifiedCounter<Vertex> }

    static member New(graph: UndirectedGraph) : Degeneracy =
        let mutable priorityPerVertex: Priority array = Array.create graph.Order 0
        let mutable queue = PriorityQueue.init graph.MaxDegree
        let mutable leftToPick = FortifiedCounter.init ()

        for v in graph.ConnectedVertices() do
            match graph.degree v with
            | 0 -> ()
            | priority ->
                priorityPerVertex[v.index] <- priority
                PriorityQueue.Put(&queue, priority, v)
                FortifiedCounter.Add(&leftToPick, v)

        { graph = graph
          priorityPerVertex = priorityPerVertex
          queue = queue
          leftToPick = leftToPick }


    // Enumerate connected vertices in degeneracy order, skipping vertices
    // whose neighbours have all been enumerated already.
    member this.iter() : Vertex seq =
        seq {
            while this.leftToPick.count > 0 do
                let pick =
                    match PriorityQueue.Pop &this.queue with
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
                            Debug.Assert(this.queue.Contains(old_priority, v))
                            Debug.Assert(this.leftToPick.Contains(v))
                            let new_priority = old_priority - 1
                            this.priorityPerVertex[v.index] <- new_priority

                            if new_priority > 0 then
                                // Requeue with a more urgent priority.
                                PriorityQueue.Put(&this.queue, new_priority, v)
                            else
                                // Dequeue.
                                FortifiedCounter.Remove(&this.leftToPick, v)
        }

    member this.isCandidate(v: Vertex) : bool = this.priorityPerVertex[v.index] > 0
