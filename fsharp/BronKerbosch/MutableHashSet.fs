namespace BronKerbosch

open System.Collections
open System.Diagnostics

type MutableHashSet<'T> =
    { set: Generic.HashSet<'T> }

    interface Generic.IEnumerable<'T> with
        member this.GetEnumerator() : Generic.IEnumerator<'T> = (this.set :> seq<'T>).GetEnumerator()

    interface IEnumerable with
        member this.GetEnumerator() : IEnumerator =
            (this.set :> IEnumerable).GetEnumerator()

    member inline this.Any: bool = this.set.Count <> 0
    member inline this.Contains(value: 'T) : bool = this.set.Contains(value)

    static member empty: MutableHashSet<'T> = { set = new Generic.HashSet<'T>() }

    static member inline singleton(value: 'T) : MutableHashSet<'T> =
        { set = new Generic.HashSet<'T>([ value ]) }

    static member inline ofSeq(values: 'T seq) : MutableHashSet<'T> =
        { set = new Generic.HashSet<'T>(values) }

    static member inline count(s: MutableHashSet<'T>) : int = s.set.Count

    static member inline difference (s: MutableHashSet<'T>) (t: MutableHashSet<'T>) : MutableHashSet<'T> =
        { set = new Generic.HashSet<'T>(s.set |> Seq.filter (t.set.Contains >> not)) }

    static member inline intersect (s: MutableHashSet<'T>) (t: MutableHashSet<'T>) : MutableHashSet<'T> =
        { set =
            if s.set.Count < t.set.Count then
                new Generic.HashSet<'T>(s.set |> Seq.filter (t.set.Contains))
            else
                new Generic.HashSet<'T>(t.set |> Seq.filter (s.set.Contains)) }

    static member inline partition (p: 'T -> bool) (s: MutableHashSet<'T>) : (MutableHashSet<'T> * MutableHashSet<'T>) =
        let store
            ((first, second): MutableHashSet<'T> * MutableHashSet<'T>)
            (value: 'T)
            : MutableHashSet<'T> * MutableHashSet<'T> =
            let destination = if p value then first else second
            let added = destination.set.Add(value)
            Debug.Assert(added)
            (first, second)

        s.set |> Seq.fold store (MutableHashSet.empty, MutableHashSet.empty)

    static member inline overlap (s: MutableHashSet<'T>) (t: MutableHashSet<'T>) : int =
        if s.set.Count < t.set.Count then
            s.set |> Seq.filter (t.set.Contains) |> Seq.length
        else
            t.set |> Seq.filter (s.set.Contains) |> Seq.length

    static member inline is_disjoint (s: MutableHashSet<'T>) (t: MutableHashSet<'T>) : bool =
        if s.set.Count < t.set.Count then
            s.set |> Seq.forall (t.set.Contains >> not)
        else
            t.set |> Seq.forall (s.set.Contains >> not)

    static member inline new_mutable(capacity: int) =
        { set = new Generic.HashSet<'T>(capacity) }

    static member inline insert_mutably(s: byref<MutableHashSet<'T>>, v: 'T) : Unit =
        let added = s.set.Add(v)
        Debug.Assert added

    static member inline remove_mutably(s: byref<MutableHashSet<'T>>, v: 'T) : Unit =
        let removed = s.set.Remove(v)
        Debug.Assert removed
