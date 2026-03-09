namespace BronKerbosch

open System.Diagnostics

type Priority = int

type PriorityQueue< ^T when ^T: comparison> =
    { stackPerPriority: ^T list array
      mutable dequeued: Set< ^T >
      mutable numLeftToPick: int
#if DEBUG
      mutable leftToPick: Set< ^T >
#endif
    }

    member this.Empty = this.numLeftToPick = 0

    member this.Insert(element: ^T, priority: Priority) =
        Debug.Assert(priority > 0)
        Debug.Assert(this.dequeued.IsEmpty)
#if DEBUG
        Debug.Assert(not (this.leftToPick.Contains(element)))
#endif
        this.push (element, priority)
        this.numLeftToPick <- this.numLeftToPick + 1
#if DEBUG
        this.leftToPick <- this.leftToPick.Add(element)
        Debug.Assert(this.numLeftToPick = this.leftToPick.Count)
#endif

    // Requeue with a more urgent priority or dequeue.
    member this.Promote(element: ^T, old_priority: Priority) : Priority =
        Debug.Assert(this.numLeftToPick > 0)
        Debug.Assert(old_priority > 0)
        Debug.Assert(this.stackPerPriority[old_priority - 1] |> List.contains element)
#if DEBUG
        Debug.Assert(this.leftToPick.Contains(element))
#endif
        match old_priority - 1 with
        | 0 ->
            this.dequeue element
            // Don't bother to find and remove the original entry in the queue,
            // since the vertex will be skipped when popped, and thanks to
            // numLeftToPick we might not need to pop it at all.
            0
        | new_priority ->
            this.push (element, new_priority)
            new_priority

    member private this.push(element: ^T, priority: Priority) =
        this.stackPerPriority[priority - 1] <- element :: this.stackPerPriority[priority - 1]

    member this.Pop() : ^T =
        Debug.Assert(this.numLeftToPick > 0)

        let popped = this.raw_pop ()

        if this.dequeued.Contains(popped) then
            // this.Promote did not bother to clean up this one, so try the next one
            this.Pop()
        else
            this.dequeue popped
            popped


    member private this.raw_pop() : ^T =
        match
            this.stackPerPriority
            |> Array.indexed
            |> Array.choose (fun (i: int, stack: ^T list) ->
                match stack with
                | head :: tail -> Some(i, head, tail)
                | [] -> None)
            |> Array.tryHead
        with
        | Some(i, head, tail) ->
            this.stackPerPriority[i] <- tail
            head
        | None -> failwith "Cannot pop more than has been put"

    member private this.dequeue(element: ^T) =
        Debug.Assert(this.numLeftToPick > 0)
        this.numLeftToPick <- this.numLeftToPick - 1
        this.dequeued <- this.dequeued.Add(element)
#if DEBUG
        Debug.Assert(this.leftToPick.Contains(element))
        this.leftToPick <- this.leftToPick.Remove(element)
        Debug.Assert(this.numLeftToPick = this.leftToPick.Count)
#endif

module PriorityQueue =
    let create<'T when ^T: comparison> (maxPriority: int) : PriorityQueue<'T> =
        { stackPerPriority = Array.create maxPriority []
          dequeued = Set.empty
          numLeftToPick = 0
#if DEBUG
          leftToPick = Set.empty
#endif
        }
