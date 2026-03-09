namespace BronKerbosch

open System.Diagnostics

type PriorityQueueWithDequeued< ^T when ^T: comparison> =
    { queue: PriorityQueue< ^T >
      mutable dequeued: Set< ^T > }

    member this.IsQueued(element: ^T, priority: Priority) =
#if DEBUG
        this.queue.stack_per_priority[priority - 1] |> List.contains element
#else
        failwith "Debug only, please"
#endif

    member this.Push(element: ^T, priority: Priority) =
        Debug.Assert(priority > 0)
        Debug.Assert(not (this.IsQueued(element, priority)))
        Debug.Assert(not (this.dequeued.Contains element))
        this.queue.Push(element, priority)

    // Picked element still needs a this.Dequeue,
    // even though it has already been removed from the queue.
    member this.Pick() : ^T =
        match this.queue.Pop() with
        | None -> failwith "Cannot pop more than has been pushed"
        | Some(popped) when this.dequeued.Contains popped ->
            // queue.Pop just cleaned up a requeued item already picked, so try the next one.
            this.Pick()
        | Some(popped) -> popped

    // Don't bother to find and remove the original entry in the queue,
    // if it still exists, since we will skip the vertex when popped.
    member this.Dequeue(element: ^T) =
        Debug.Assert(not (this.dequeued.Contains element))
        this.dequeued <- this.dequeued.Add element

module PriorityQueueWithDequeued =
    let create<'T when ^T: comparison> (maxPriority: int) : PriorityQueueWithDequeued<'T> =
        { queue = PriorityQueue.create maxPriority
          dequeued = Set.empty }
