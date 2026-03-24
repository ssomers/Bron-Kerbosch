namespace BronKerbosch

open System.Diagnostics

// 1 = top priority, 2 or more = lower priority
type Priority = int

type PriorityQueue<'T when 'T: equality> =
    { stackPerPriority: 'T list array }

    static member init(maxPriority: int) : PriorityQueue<'T> =
        { stackPerPriority = Array.create maxPriority [] }

    member this.Contains(priority: Priority, element: 'T) =
        Debug.Assert(priority > 0)
        this.stackPerPriority[priority - 1] |> List.contains element


    // Putting the same element again only makes sense if it is with a more urgent priority, i.e. closer to 1.
    static member Put(this: byref<PriorityQueue<'T>>, priority: Priority, element: 'T) =
        Debug.Assert(priority > 0)
        this.stackPerPriority[priority - 1] <- element :: this.stackPerPriority[priority - 1]

    // May pop an element already popped earlier, in case it was put multiple times.
    static member Pop(this: byref<PriorityQueue<'T>>) : 'T option = this.PopAtOrAbove(0)

    member private this.PopAtOrAbove(index: int) : 'T option =
        if index < this.stackPerPriority.Length then
            match this.stackPerPriority[index] with
            | [] -> this.PopAtOrAbove(index + 1)
            | head :: tail ->
                this.stackPerPriority[index] <- tail
                Some(head)
        else
            None
