namespace BronKerbosch

open System.Diagnostics

// Counts the coming and going of elements and, in debug builds only, checks their identity.
type FortifiedCounter<'T when 'T: equality> =
    { mutable count: int
      mutable tracker: DebugOnlyTracker<'T> }

    static member init() : FortifiedCounter<'T> =
        { count = 0
          tracker = DebugOnlyTracker.init () }

    member this.Contains(element: 'T) = this.tracker.Contains(element)

    static member Add(this: byref<FortifiedCounter<'T>>, element: 'T) : Unit =
        Debug.Assert(this.count = this.tracker.Count)
        Debug.Assert(this.Contains element |> not)
        this.count <- this.count + 1
        DebugOnlyTracker.Add(&this.tracker, element)
        Debug.Assert(this.count = this.tracker.Count)

    static member Remove(this: byref<FortifiedCounter<'T>>, element: 'T) =
        Debug.Assert(this.count = this.tracker.Count)
        Debug.Assert(this.Contains element)
        this.count <- this.count - 1
        DebugOnlyTracker.Remove(&this.tracker, element)
        Debug.Assert(this.count = this.tracker.Count)
