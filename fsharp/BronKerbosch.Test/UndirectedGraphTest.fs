module BronKerbosch.Test.UndirectedGraph

open NUnit.Framework
open BronKerbosch

[<Test>]
let EmptyUndirectedGraph () =
    let g = UndirectedGraph.ofAdjacencies [||]
    Assert.That(g.Order, Is.EqualTo(0))
    Assert.That(g.Size, Is.EqualTo(0))
    Assert.That(g.MaxDegree, Is.EqualTo(0))
    // g.Vertices() |> Seq.iter (fun v -> printfn "vertex %d" v.index)
    Assert.That(g.Vertices(), Is.Empty)
    Assert.That(g.ConnectedVertices(), Is.Empty)
    Assert.That(g.MaxDegreeVertices(), Is.Empty)
