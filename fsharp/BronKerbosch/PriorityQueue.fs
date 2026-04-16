namespace BronKerbosch

open System.Diagnostics

// 1 = top priority, 2 or more = lower priority
type Priority = int

type PriorityQueue<'T when 'T: equality> =
    { stackPerPriority: 'T list array
      mutable lowestPopulatedIndex: Priority }

    static member init(maxPriority: int) : PriorityQueue<'T> =
        { stackPerPriority = Array.create maxPriority []
          lowestPopulatedIndex = maxPriority }

    member this.Contains(priority: Priority, element: 'T) =
        Debug.Assert(priority > 0)
#if DEBUG
        this.stackPerPriority[priority - 1] |> List.contains element
#else
        failwith "Debug build only, please"
#endif

    // Putting the same element again does not replace the previous entry.
    static member Put(this: byref<PriorityQueue<'T>>, priority: Priority, element: 'T) =
        Debug.Assert(priority > 0)
        let index = priority - 1
        this.stackPerPriority[index] <- element :: this.stackPerPriority[index]
        this.lowestPopulatedIndex <- min this.lowestPopulatedIndex index

    // May pop an element already popped earlier, in case it was put multiple times.
    static member Pop(this: byref<PriorityQueue<'T>>) : 'T option =
        if this.lowestPopulatedIndex < this.stackPerPriority.Length then
            let stack = &this.stackPerPriority[this.lowestPopulatedIndex]

            match stack with
            | head :: tail ->
                stack <- tail

                // Not a necessary adjustment, but a ~5% performance boost.
                if tail.IsEmpty then
                    this.lowestPopulatedIndex <- this.lowestPopulatedIndex + 1

                Some(head)
            | [] ->
                this.lowestPopulatedIndex <- this.lowestPopulatedIndex + 1
                PriorityQueue.Pop &this
        else
            None
