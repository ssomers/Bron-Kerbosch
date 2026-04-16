namespace BronKerbosch

open System.Diagnostics

module move =
    let inline through (cell: byref<'T>) (new_value: 'T) : 'T =
        let old_value = cell
        cell <- new_value
        old_value

type public Degeneracy =
    { graph: UndirectedGraph
      // Possible values of priorityPerVertex (after initialization):
      //   0: never queued because not connected (degree 0),
      //      or no longer queued because it has been yielded itself,
      //      or no longer queued because all neighbours have been yielded
      //   1 or more: candidates queued with priority (degree - #of yielded neighbours)
      mutable priorityPerVertex: Priority array

      mutable queue: PriorityQueue<Vertex>

      // We keep a count not just to verify ourselves, but also to avoid at the end
      // individually popping many vertices that have been queued multiple times.
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
                match PriorityQueue.Pop &this.queue with
                | None -> failwith "Cannot pop more than was put"
                | Some(pick) ->
                    let priority = move.through &this.priorityPerVertex[pick.index] 0

                    if priority > 0 then
                        yield pick
                        this.adjust_neighbours pick
                        FortifiedCounter.Remove(&this.leftToPick, pick)
        }

    member private this.adjust_neighbours(pick: Vertex) : Unit =
        for v in this.graph.neighbours pick do
            let priority = &this.priorityPerVertex[v.index]

            if priority > 0 then
                Debug.Assert(this.queue.Contains(priority, v))
                Debug.Assert(this.leftToPick.Contains v)
                // Either queue again with a more urgent priority or dequeue.
                // Don't bother to remove the original entry from the queue,
                // since the vertex will be skipped when popped, and thanks to
                // leftToPick we might not need to pop it at all.
                priority <- priority - 1

                if priority > 0 then
                    PriorityQueue.Put(&this.queue, priority, v)
                else
                    FortifiedCounter.Remove(&this.leftToPick, v)

    member this.isCandidate(v: Vertex) : bool = this.priorityPerVertex[v.index] > 0
