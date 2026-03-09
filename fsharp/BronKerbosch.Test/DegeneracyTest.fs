module BronKerbosch.Test.Degeneracy

open NUnit.Framework
open BronKerbosch
open BronKerbosch.Test.UndirectedGraph

[<Test>]
let order0 () =
    let g = graph_order0
    Assert.That(Degeneracy.iter g, Is.Empty)

[<Test>]
let order1 () =
    let g = graph_order1
    Assert.That(Degeneracy.iter g, Is.Empty)

[<Test>]
let order2_isolated () =
    let g = graph_order2_isolated
    Assert.That(Degeneracy.iter g, Is.Empty)

[<Test>]
let order2_connected () =
    let g = graph_order2_connected
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 1)

[<Test>]
let order3_size1_left () =
    let g = graph_order3_size1_left
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 1)

[<Test>]
let order3_size1_long () =
    let g = graph_order3_size1_long
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 1)

[<Test>]
let order3_size1_right () =
    let g = graph_order3_size1_right
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 1)

[<Test>]
let order3_size2 () =
    let g = graph_order3_size2
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 2)

[<Test>]
let order3_size3 () =
    let g = graph_order3_size3
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 2)

[<Test>]
let order4_size2 () =
    let g = graph_order4_size2
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 2)

[<Test>]
let order4_size3_bus () =
    let g = graph_order4_size3_bus
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 3)
    Assert.That(Degeneracy.iter g |> Seq.map (fun (v, _) -> v) |> Seq.head, Is.AnyOf [| { index = 0 }; { index = 3 } |])

[<Test>]
let order4_size3_star () =
    let g = graph_order4_size3_star
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 3)
    Assert.That(Degeneracy.iter g |> Seq.map (fun (v, _) -> v) |> Seq.head, Is.Not.EqualTo { index = 0 })

[<Test>]
let order4_size4_p () =
    let g = graph_order4_size4_p
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 3)
    Assert.That(Degeneracy.iter g |> Seq.map (fun (v, _) -> v) |> Seq.head, Is.EqualTo { index = 0 })

[<Test>]
let order4_size4_square () =
    let g = graph_order4_size4_square
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 3)

[<Test>]
let order4_size5 () =
    let g = graph_order4_size5
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 3)
    Assert.That(Degeneracy.iter g |> Seq.map (fun (v, _) -> v) |> Seq.head, Is.AnyOf [| { index = 1 }; { index = 3 } |])

[<Test>]
let order5_size6_penultimate () =
    let g = graph_order5_size6_penultimate
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 4)

[<Test>]
let sample () =
    let g = graph_sample
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 6)
    Assert.That(Degeneracy.iter g |> Seq.map (fun (v, _) -> v) |> Seq.head, Is.AnyOf [| { index = 6 }; { index = 7 } |])

[<Test>]
let bigger () =
    let g = graph_bigger
    Assert.That(Degeneracy.iter g |> Seq.length, Is.EqualTo 9)
    Assert.That(Degeneracy.iter g |> Seq.map (fun (v, _) -> v) |> Seq.head, Is.AnyOf [| { index = 5 }; { index = 8 } |])
