namespace BronKerbosch

[<StructuredFormatDisplay("v{index}")>]
type public Vertex = { index: int }

module VertexSet =
    let overlaps (a: Set<Vertex>, b: Set<Vertex>) : bool = not (Set.isEmpty (Set.intersect a b))

    let pop_arbitrary (s: Set<Vertex>) : Vertex * Set<Vertex> =
        let popped = s.MaximumElement
        (popped, s.Remove(popped))
