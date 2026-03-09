namespace BronKerbosch

open System.Diagnostics

[<AutoOpen>]
module Adjacencies =
    let public areSymmetrical (adjacencies: Set<Vertex> array) : bool =
        let isSymmetric ((v, neighbours): Vertex * Set<Vertex>) : bool =
            neighbours |> Set.forall adjacencies[v.index].Contains

        adjacencies |> Array.indexed |> Array.map Verticise.first |> Array.forall isSymmetric

    let public areLoopFree (adjacencies: Set<Vertex> array) : bool =
        let isLoopFree ((v, neighbours): Vertex * Set<Vertex>) : bool = not (neighbours.Contains(v))

        adjacencies |> Array.indexed |> Array.map Verticise.first |> Array.forall isLoopFree

type public UndirectedGraph =
    { Adjacencies: Set<Vertex> array
      Size: int
      MaxDegree: int }

    member this.Order = this.Adjacencies.Length
    member inline this.neighbours(v: Vertex) : Set<Vertex> = this.Adjacencies[v.index]
    member inline this.hasNeighbours(v: Vertex) : bool = not this.Adjacencies[v.index].IsEmpty
    member inline this.degree(v: Vertex) : int = this.Adjacencies[v.index].Count

    member inline this.Vertices() : Vertex seq =
        seq { 0 .. this.Adjacencies.Length - 1 } |> Seq.map (fun i -> { index = i })

    member inline this.ConnectedVertices() : Vertex seq =
        this.Vertices() |> Seq.filter this.hasNeighbours

    member inline this.MaxDegreeVertices() : Vertex seq =
        this.Vertices() |> Seq.filter (fun v -> this.degree (v) = this.MaxDegree)

module UndirectedGraph =
    let public ofAdjacencies (adjacencies: Set<Vertex> array) : UndirectedGraph =
        Debug.Assert(Adjacencies.areSymmetrical (adjacencies))
        Debug.Assert(Adjacencies.areLoopFree (adjacencies))

        let total_degree = adjacencies |> Array.sumBy Set.count
        Debug.Assert(total_degree % 2 = 0)

        let max_degree =
            match adjacencies with
            | [||] -> 0
            | some -> some |> Array.maxBy Set.count |> Set.count

        { Adjacencies = adjacencies
          Size = total_degree / 2
          MaxDegree = max_degree }
