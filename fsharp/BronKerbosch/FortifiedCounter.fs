namespace BronKerbosch

open System.Diagnostics

// Counts the coming and going of elements and, in debug build only, checks their identity.
type FortifiedCounter<'T when 'T: equality> =
    { mutable count: int
#if DEBUG
      mutable individuals: 'T list  // set-like, but don't force more traits on 'T here
#endif
    }

    member private this.invariant: bool =
#if DEBUG
        this.count = this.individuals.Length
#else
        failwith "debug build only, please"
#endif

    member this.Contains(element: 'T) =
#if DEBUG
        this.individuals |> List.contains element
#else
        failwith "debug build only, please"
#endif

    member this.Add(element: 'T) =
        Debug.Assert this.invariant
        this.count <- this.count + 1
#if DEBUG
        Debug.Assert(not (this.Contains element))
        this.individuals <- element :: this.individuals
#endif
        Debug.Assert this.invariant

    member this.Remove(element: 'T) =
        Debug.Assert this.invariant
        this.count <- this.count - 1
#if DEBUG
        Debug.Assert(this.Contains element)
        this.individuals <- this.individuals |> List.except [ element ]
#endif
        Debug.Assert this.invariant

module FortifiedCounter =
    let empty () : FortifiedCounter<'T> =
        { count = 0
#if DEBUG
          individuals = []
#endif
        }
