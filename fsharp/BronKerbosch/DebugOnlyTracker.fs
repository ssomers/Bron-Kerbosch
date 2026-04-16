namespace BronKerbosch

open System.Diagnostics

// Tracks the coming and going of elements in debug builds only.
type DebugOnlyTracker<'T when 'T: equality> =
    {
#if DEBUG
      mutable individuals: 'T list  // set-like, but don't force more traits on 'T here
#else
      phantom: Unit
#endif
    }

    static member init() : DebugOnlyTracker<'T> =
        {
#if DEBUG
          individuals = []
#else
          phantom = ()
#endif
        }

    member this.Count: int =
#if DEBUG
        this.individuals.Length
#else
        failwith "Debug build only, please"
#endif

    member this.Contains(element: 'T) =
#if DEBUG
        List.contains element this.individuals
#else
        failwith "debug build only, please"
#endif

    static member Add(this: byref<DebugOnlyTracker<'T>>, element: 'T) : Unit =
        Debug.Assert(this.Contains element |> not)
#if DEBUG
        this.individuals <- element :: this.individuals
#endif

    static member Remove(this: byref<DebugOnlyTracker<'T>>, element: 'T) =
        Debug.Assert(this.Contains element)
#if DEBUG
        this.individuals <- this.individuals |> List.except [ element ]
#endif
