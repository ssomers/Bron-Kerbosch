module BronKerbosch.VertexSet

let inline difference (a: Set<Vertex>) (b: Set<Vertex>) : Set<Vertex> = Set.difference a b

let inline intersect (a: Set<Vertex>) (b: Set<Vertex>) : Set<Vertex> =
    // https://github.com/dotnet/fsharp/issues/19139
    if a.Count < b.Count then
        Set.intersect a b
    else
        Set.intersect b a

let intersection_size (a: Set<Vertex>) (b: Set<Vertex>) : int = intersect a b |> Set.count // TODO probably optimize

let inline is_disjoint (a: Set<Vertex>) (b: Set<Vertex>) : bool =
    if a.Count < b.Count then
        a |> Set.forall (b.Contains >> not)
    else
        b |> Set.forall (a.Contains >> not)

let inline pop_arbitrary (s: Set<Vertex>) : Vertex option * Set<Vertex> =
    match Seq.tryHead s with
    | None -> (None, s)
    | Some v -> (Some v, s.Remove v)
