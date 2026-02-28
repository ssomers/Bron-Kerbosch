namespace BronKerbosch

type public Vertex = { index: int }

module Adjacencies =
    let verticise (index: int) : Vertex = { index = index }

    let verticise2 ((index, neighbours): int * Set<Vertex>) : Vertex * Set<Vertex> = verticise (index), neighbours

    let public AreSymmetrical (adjacencies: Set<Vertex> array) : bool =
        let isSymmetric ((v, neighbours): Vertex * Set<Vertex>) : bool =
            neighbours |> Set.forall adjacencies[v.index].Contains

        adjacencies |> Array.indexed |> Array.map verticise2 |> Array.forall isSymmetric

    let public AreLoopFree (adjacencies: Set<Vertex> array) : bool =
        let isLoopFree ((v, neighbours): Vertex * Set<Vertex>) : bool = not (neighbours.Contains(v))

        adjacencies |> Array.indexed |> Array.map verticise2 |> Array.forall isLoopFree

type public UndirectedGraph =
    { Adjacencies: Set<Vertex> array
      Size: int
      MaxDegree: int }

    member inline this.Order() : int = this.Adjacencies.Length
    member inline this.Neighbours(v: Vertex) : Set<Vertex> = this.Adjacencies[v.index]
    member inline this.HasNeighbours(v: Vertex) : bool = not (this.Adjacencies[v.index].IsEmpty)
    member inline this.Degree(v: Vertex) : int = this.Adjacencies[v.index].Count

    member inline this.Vertices() : Vertex seq =
        seq { 0 .. this.Adjacencies.Length - 1 } |> Seq.map (fun i -> { index = i })

    member inline this.ConnectedVertices() : Vertex seq =
        this.Vertices() |> Seq.filter this.HasNeighbours

    member inline this.MaxDegreeVertices() : Vertex seq =
        this.Vertices() |> Seq.filter (fun v -> this.Degree(v) = this.MaxDegree)

module UndirectedGraph =
    let public From (adjacencies: Set<Vertex> array) : UndirectedGraph =
        assert (Adjacencies.AreSymmetrical(adjacencies))
        assert (Adjacencies.AreLoopFree(adjacencies))

        let total_degree = adjacencies |> Array.sumBy Set.count
        assert (total_degree % 2 = 0)

        let max_degree =
            match adjacencies with
            | [||] -> 0
            | some -> some |> Array.maxBy Set.count |> Set.count

        { Adjacencies = adjacencies
          Size = total_degree / 2
          MaxDegree = max_degree }
