namespace BronKerbosch

open System.Diagnostics

type Priority = int

type PriorityQueue< ^T> =
    { stack_per_priority: ^T list array }

    // If the same element is pushed again, its priority must be higher i.e. a lower value.
    member this.Push(element: ^T, priority: Priority) =
        Debug.Assert(priority > 0)
        this.stack_per_priority[priority - 1] <- element :: this.stack_per_priority[priority - 1]

    // May pop an element already popped earlier, in case it was pushed again.
    member this.Pop() : ^T option =
        match
            this.stack_per_priority
            |> Array.indexed
            |> Array.choose (fun (i: int, stack: ^T list) ->
                match stack with
                | head :: tail -> Some(i, head, tail)
                | [] -> None)
            |> Array.tryHead
        with
        | Some(i, head, tail) ->
            this.stack_per_priority[i] <- tail
            Some(head)
        | None -> None

module PriorityQueue =
    let create<'T> (maxPriority: int) : PriorityQueue<'T> =
        { stack_per_priority = Array.create maxPriority [] }
