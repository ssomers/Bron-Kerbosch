namespace BronKerbosch

open System.Collections
open System.Diagnostics

type SizedSet<'T when 'T: comparison> =
    { mutable set: Set<'T>
      mutable size: int }

    interface Generic.IEnumerable<'T> with
        member this.GetEnumerator() : Generic.IEnumerator<'T> = (this.set :> seq<'T>).GetEnumerator()

    interface IEnumerable with
        member this.GetEnumerator() : IEnumerator =
            (this.set :> IEnumerable).GetEnumerator()

    member inline this.Any: bool = not this.set.IsEmpty
    member inline this.Contains(value: 'T) : bool = this.set.Contains(value)

    static member inline singleton(value: 'T) : SizedSet<'T> = { set = Set.singleton value; size = 1 }

    static member inline partition (p: 'T -> bool) (s: SizedSet<'T>) : (SizedSet<'T> * SizedSet<'T>) =
        let (first, second) = s.set |> Set.partition p
        ({ set = first; size = first.Count }, { set = second; size = second.Count })

    static member inline ofSeq(values: 'T seq) : SizedSet<'T> =
        let set = Set.ofSeq values
        { set = set; size = set.Count }

    static member inline count(s: SizedSet<'T>) : int = s.size

    static member inline difference (s: SizedSet<'T>) (t: SizedSet<'T>) : SizedSet<'T> =
        let set = Set.difference s.set t.set
        { set = set; size = set.Count }

    static member inline intersect (s: SizedSet<'T>) (t: SizedSet<'T>) : SizedSet<'T> =
        // While waiting for .NET 11 (https://github.com/dotnet/fsharp/issues/19139)
        let set =
            if s.size < t.size then
                Set.intersect s.set t.set
            else
                Set.intersect t.set s.set

        { set = set; size = set.Count }

    static member inline overlap (s: SizedSet<'T>) (t: SizedSet<'T>) : int =
        if s.size < t.size then
            s.set |> Seq.filter t.set.Contains |> Seq.length
        else
            t.set |> Seq.filter s.set.Contains |> Seq.length

    static member inline is_disjoint (s: SizedSet<'T>) (t: SizedSet<'T>) : bool =
        if s.size < t.size then
            s.set |> Set.forall (t.set.Contains >> not)
        else
            t.set |> Set.forall (s.set.Contains >> not)

    static member inline new_mutable(capacity: int) = { set = Set.empty; size = 0 }

    static member inline insert_mutably(s: byref<SizedSet<'T>>, v: 'T) : Unit =
        Debug.Assert(s.set.Contains v |> not)
        s.set <- s.set.Add(v)
        s.size <- s.size + 1

    static member inline remove_mutably(s: byref<SizedSet<'T>>, v: 'T) : Unit =
        Debug.Assert(s.set.Contains v)
        s.set <- s.set.Remove(v)
        s.size <- s.size - 1
