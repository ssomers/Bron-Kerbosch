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
        true
#endif

    member this.Contains(element: 'T) =
        this.individuals |> List.contains element

    member this.Add(element: 'T) =
        assert this.invariant
        this.count <- this.count + 1
#if DEBUG
        assert not (this.individuals |> List.contains element)
        this.individuals <- element :: this.individuals
#endif
        assert this.invariant

    member this.Remove(element: 'T) =
        assert this.invariant
        this.count <- this.count - 1
#if DEBUG
        assert (this.Contains element)
        this.individuals <- this.individuals |> List.except [ element ]
#endif
        assert this.invariant

module FortifiedCounter =
    let create () : FortifiedCounter<'T> =
        { count = 0
#if DEBUG
          individuals = []
#endif
        }
