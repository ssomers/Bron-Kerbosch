namespace BronKerbosch

[<StructuredFormatDisplay("v{index}")>]
type public Vertex =
    | Vertex of int

    member this.index = let (Vertex v) = this in v


module Verticise =
    let it (index: int) : Vertex = Vertex.Vertex index
    let first<'T> ((index, arg2): int * 'T) : Vertex * 'T = Vertex.Vertex index, arg2
