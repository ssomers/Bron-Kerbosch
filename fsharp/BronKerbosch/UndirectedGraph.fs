namespace BronKerbosch

open System.Diagnostics

module Adjacencies =
    let adjacency_vertices (adjacencies: VertexSet array) : (Vertex * VertexSet) seq =
        adjacencies |> Seq.indexed |> Seq.map Verticise.first

    let public areSymmetrical (adjacencies: VertexSet array) : bool =
        let isSymmetric ((v, neighbours): Vertex * VertexSet) : bool =
            neighbours |> Seq.forall adjacencies[v.index].Contains

        adjacency_vertices adjacencies |> Seq.forall isSymmetric

    let public areLoopFree (adjacencies: VertexSet array) : bool =
        let isLoopFree ((v, neighbours): Vertex * VertexSet) : bool = not (neighbours.Contains(v))

        adjacency_vertices adjacencies |> Seq.forall isLoopFree

type public UndirectedGraph =
    { Adjacencies: VertexSet array
      Size: int
      MaxDegree: int }

    member this.Order = this.Adjacencies.Length
    member inline this.neighbours(v: Vertex) : VertexSet = this.Adjacencies[v.index]
    member inline this.hasNeighbours(v: Vertex) : bool = not this.Adjacencies[v.index].IsEmpty

    member inline this.degree(v: Vertex) : int =
        VertexSet.count this.Adjacencies[v.index]

    member inline this.Vertices() : Vertex seq =
        seq { 0 .. this.Adjacencies.Length - 1 } |> Seq.map Verticise.it

    member inline this.ConnectedVertices() : Vertex seq =
        this.Vertices() |> Seq.filter this.hasNeighbours

    member inline this.MaxDegreeVertices() : Vertex seq =
        this.Vertices() |> Seq.filter (fun v -> this.degree (v) = this.MaxDegree)

    static member inline ofAdjacencies(adjacencies: VertexSet array) : UndirectedGraph =
        Debug.Assert(Adjacencies.areSymmetrical (adjacencies))
        Debug.Assert(Adjacencies.areLoopFree (adjacencies))

        let total_degree = adjacencies |> Array.sumBy VertexSet.count
        Debug.Assert(total_degree % 2 = 0)

        let max_degree =
            match adjacencies with
            | [||] -> 0
            | some -> some |> Array.maxBy VertexSet.count |> VertexSet.count

        { Adjacencies = adjacencies
          Size = total_degree / 2
          MaxDegree = max_degree }
