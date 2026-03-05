namespace BronKerbosch

open System.Diagnostics

type Priority = int

type PriorityQueue< ^T> =
    { stackPerPriority: ^T list array
      mutable numLeftToPick: int }
    (*
#if DEBUG
        private readonly HashSet<^T> itsLeftToPick = [];
#endif
    *)

    member this.Empty = this.numLeftToPick = 0

    member this.Insert(element: ^T, priority: Priority) =
        Debug.Assert(priority > 0)
        this.push (element, priority)
        this.numLeftToPick <- this.numLeftToPick + 1
    (*
#if DEBUG
        bool added = itsLeftToPick.Add(element);
        Debug.Assert(added);
        Debug.Assert(itsNumLeftToPick == itsLeftToPick.Count);
#endif
    *)

    // Requeue with a more urgent priority or dequeue.
    // Don't bother to remove the original entry from the queue,
    // since the vertex will be skipped when popped, and thanks to
    // itsNumLeftToPick we might not need to pop it at all.
    //
    // Assumes the given priority is less than the previous priority
    // that the vertex was assigned.
    member this.Promote(element: ^T, old_priority: Priority) : Priority =
        (*
#if DEBUG
        Debug.Assert(itsLeftToPick.Contains(element));
#endif
        *)

        let new_priority = old_priority - 1

        if new_priority > 0 then
            this.push (element, new_priority)
        else
            this.Forget(element)

        new_priority

    member private this.push(element: ^T, priority: Priority) =
        this.stackPerPriority[priority - 1] <- element :: this.stackPerPriority[priority - 1]


    // We may return an element already popped, even though it was passed to Forget,
    // in case its priority was promoted earlier on. That's why we do not count
    // the element as picked, but wait for the caller to Forget it. The caller must
    // somehow ensure to Forget the same element only once.
    member this.Pop() : ^T =
        match
            this.stackPerPriority
            |> Array.indexed
            |> Array.choose (fun (i: int, stack: ^T list) ->
                match stack with
                | [] -> None
                | head :: tail ->
                    this.stackPerPriority[i] <- tail
                    Some(head))
            |> Array.tryHead
        with
        | Some(head) -> head
        | None -> failwith "Cannot pop more than has been put"

    member this.Forget(element: ^T) =
        Debug.Assert(this.numLeftToPick > 0)
        this.numLeftToPick <- this.numLeftToPick - 1
(*
#if DEBUG
        bool removed = itsLeftToPick.Remove(element);
        Debug.Assert(removed);
        Debug.Assert(itsNumLeftToPick == itsLeftToPick.Count);
#endif
*)

module PriorityQueue =
    let create<'T> (maxPriority: int) : PriorityQueue<'T> =
        { stackPerPriority = Array.create maxPriority []
          numLeftToPick = 0 }
