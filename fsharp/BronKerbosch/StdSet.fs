namespace BronKerbosch

open System.Collections
open System.Diagnostics

type StdSet<'T when 'T: comparison> =
    { mutable set: Set<'T> }

    interface Generic.IEnumerable<'T> with
        member this.GetEnumerator() : Generic.IEnumerator<'T> = (this.set :> seq<'T>).GetEnumerator()

    interface IEnumerable with
        member this.GetEnumerator() : IEnumerator =
            (this.set :> IEnumerable).GetEnumerator()

    member inline this.Any: bool = not this.set.IsEmpty
    member inline this.Contains(value: 'T) : bool = this.set.Contains(value)

    static member inline singleton(value: 'T) : StdSet<'T> = { set = Set.singleton value }

    static member inline partition (p: 'T -> bool) (s: StdSet<'T>) : (StdSet<'T> * StdSet<'T>) =
        let (first, second) = s.set |> Set.partition p
        ({ set = first }, { set = second })

    static member inline ofSeq(values: 'T seq) : StdSet<'T> =
        let set = Set.ofSeq values
        { set = set }

    static member inline count(s: StdSet<'T>) : int = s.set.Count

    static member inline difference (s: StdSet<'T>) (t: StdSet<'T>) : StdSet<'T> = { set = Set.difference s.set t.set }

    static member inline intersect (s: StdSet<'T>) (t: StdSet<'T>) : StdSet<'T> =
        // While waiting for .NET 11 (https://github.com/dotnet/fsharp/issues/19139)
        let set =
            if s.set.Count < t.set.Count then
                Set.intersect s.set t.set
            else
                Set.intersect t.set s.set

        { set = set }

    static member overlap (s: StdSet<'T>) (t: StdSet<'T>) : int =
        if s.set.Count < t.set.Count then
            s.set |> Seq.filter t.set.Contains |> Seq.length
        else
            t.set |> Seq.filter s.set.Contains |> Seq.length

    static member inline is_disjoint (s: StdSet<'T>) (t: StdSet<'T>) : bool =
        if s.set.Count < t.set.Count then
            s.set |> Set.forall (t.set.Contains >> not)
        else
            t.set |> Set.forall (s.set.Contains >> not)

    static member inline new_mutable(capacity: int) = { set = Set.empty }

    static member inline insert_mutably(s: byref<StdSet<'T>>, v: 'T) : Unit =
        Debug.Assert(s.set.Contains v |> not)
        s.set <- s.set.Add(v)

    static member inline remove_mutably(s: byref<StdSet<'T>>, v: 'T) : Unit =
        Debug.Assert(s.set.Contains v)
        s.set <- s.set.Remove(v)
