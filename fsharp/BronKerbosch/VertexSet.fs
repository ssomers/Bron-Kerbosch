namespace BronKerbosch

#if NDEBUG

type VertexSet = Set<Vertex>

module VertexSet =
    let empty: VertexSet = Set.empty
    let inline singleton (v: Vertex) : VertexSet = Set.singleton v
    let inline ofSeq (vertices: Vertex seq) : VertexSet = Set.ofSeq vertices

    let inline count (s: VertexSet) : int = s.Count

    let inline difference (a: VertexSet) (b: VertexSet) : VertexSet = Set.difference a b

    let inline intersect (a: VertexSet) (b: VertexSet) : VertexSet =
        // While waiting for .NET 11 (https://github.com/dotnet/fsharp/issues/19139)
        if a.Count < b.Count then
            Set.intersect a b
        else
            Set.intersect b a

    let overlap (a: VertexSet) (b: VertexSet) : int =
        if a.Count < b.Count then
            a |> Seq.filter b.Contains |> Seq.length
        else
            b |> Seq.filter a.Contains |> Seq.length

    let inline is_disjoint (a: VertexSet) (b: VertexSet) : bool =
        if a.Count < b.Count then
            a |> Set.forall (b.Contains >> not)
        else
            b |> Set.forall (a.Contains >> not)

    let inline pop_arbitrary (s: VertexSet) : Vertex option * VertexSet =
        match Seq.tryHead s with
        | None -> (None, s)
        | Some v -> (Some v, s.Remove v)

#else

type VertexSet = SizedSet<Vertex>

module VertexSet =
    let empty: VertexSet = { set = Set.empty; size = 0 }

    let inline singleton (v: Vertex) : VertexSet = { set = Set.singleton v; size = 1 }

    let inline ofSeq (vertices: Vertex seq) : VertexSet =
        let set = vertices |> Set.ofSeq
        { set = set; size = set.Count }

    let inline count (s: VertexSet) : int = s.size

    let difference (s: VertexSet) (t: VertexSet) : VertexSet =
        let set = Set.difference s.set t.set
        { set = set; size = set.Count }

    let intersect (s: VertexSet) (t: VertexSet) : VertexSet =
        // While waiting for .NET 11 (https://github.com/dotnet/fsharp/issues/19139)
        let set =
            if s.size < t.size then
                Set.intersect s.set t.set
            else
                Set.intersect t.set s.set

        { set = set; size = set.Count }

    let overlap (s: VertexSet) (t: VertexSet) : int =
        if s.size < t.size then
            s.set |> Seq.filter t.set.Contains |> Seq.length
        else
            t.set |> Seq.filter s.set.Contains |> Seq.length

    let is_disjoint (s: VertexSet) (t: VertexSet) : bool =
        if s.size < t.size then
            s.set |> Set.forall (t.set.Contains >> not)
        else
            t.set |> Set.forall (s.set.Contains >> not)

    let pop_arbitrary (s: VertexSet) : Vertex option * VertexSet =
        match Seq.tryHead s.set with
        | None -> (None, s)
        | Some v ->
            (Some v,
             { set = s.set.Remove v
               size = s.size - 1 })

#endif
