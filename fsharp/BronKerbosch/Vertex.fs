namespace BronKerbosch

[<StructuredFormatDisplay("v{index}")>]
type public Vertex = { index: int }

module Verticise =
    let it (index: int) : Vertex = { index = index }
    let first<'T> ((index, second): int * 'T) : Vertex * 'T = it index, second
