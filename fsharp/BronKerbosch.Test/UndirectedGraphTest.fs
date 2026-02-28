module BronKerbosch.Test.UndirectedGraph

open NUnit.Framework
open BronKerbosch

[<Test>]
let EmptyUndirectedGraph () =
    let g = UndirectedGraph.From([||])
    Assert.That(g.Order().Equals(0))
    Assert.That(g.Size.Equals(0))
    Assert.That(g.MaxDegree.Equals(0))
    // g.Vertices() |> Seq.iter (fun v -> printfn "vertex %d" v.index)
    Assert.That(g.Vertices() |> Seq.isEmpty)
    Assert.That(g.ConnectedVertices() |> Seq.isEmpty)
    Assert.That(g.MaxDegreeVertices() |> Seq.isEmpty)
