module BronKerbosch.Test.UndirectedGraph

open NUnit.Framework
open BronKerbosch

let graph (adjacencies: int list array) : UndirectedGraph =
    let adjacencies =
        adjacencies
        |> Array.map (fun neighbours -> neighbours |> Seq.map Verticise.it |> VertexSet.ofSeq)

    UndirectedGraph.ofAdjacencies adjacencies

let graph_order0 = graph [||]

[<Test>]
let order0 () =
    let g = graph_order0
    Assert.That(g.Order, Is.EqualTo 0)
    Assert.That(g.Size, Is.EqualTo 0)
    Assert.That(g.MaxDegree, Is.EqualTo 0)
    Assert.That(g.Vertices(), Is.Empty)
    Assert.That(g.ConnectedVertices(), Is.Empty)
    Assert.That(g.MaxDegreeVertices(), Is.Empty)


let graph_order1 = graph [| [] |]

[<Test>]
let order1 () =
    let g = graph_order1
    Assert.That(g.Order, Is.EqualTo 1)
    Assert.That(g.Size, Is.EqualTo 0)
    Assert.That(g.MaxDegree, Is.EqualTo 0)
    Assert.That(g.Vertices(), Is.EqualTo<Vertex seq>(seq { Vertex 0 }))
    Assert.That(g.ConnectedVertices(), Is.Empty)
    Assert.That(g.MaxDegreeVertices(), Is.EqualTo<Vertex seq>(seq { Vertex 0 }))


let graph_order2_isolated = graph [| []; [] |]

[<Test>]
let order2_isolated () =
    let g = graph_order2_isolated
    Assert.That(g.Order, Is.EqualTo 2)
    Assert.That(g.Size, Is.EqualTo 0)
    Assert.That(g.MaxDegree, Is.EqualTo 0)

    Assert.That(g.Vertices(), Is.EqualTo<Vertex seq>([ 0; 1 ] |> Seq.map Verticise.it))

    Assert.That(g.ConnectedVertices(), Is.Empty)
    Assert.That(g.MaxDegreeVertices(), Is.EqualTo<Vertex seq>(g.Vertices()))


let graph_order2_connected = graph [| [ 1 ]; [ 0 ] |]

[<Test>]
let order2_connected () =
    let g = graph_order2_connected
    Assert.That(g.Order, Is.EqualTo 2)
    Assert.That(g.Size, Is.EqualTo 1)
    Assert.That(g.MaxDegree, Is.EqualTo 1)

    Assert.That(g.Vertices(), Is.EqualTo<Vertex seq>([ 0; 1 ] |> Seq.map Verticise.it))

    Assert.That(g.ConnectedVertices(), Is.EqualTo<Vertex seq>(g.Vertices()))
    Assert.That(g.MaxDegreeVertices(), Is.EqualTo<Vertex seq>(g.Vertices()))


let graph_order3_size1_left = graph [| [ 1 ]; [ 0 ]; [] |]

[<Test>]
let order3_size1_left () =
    let g = graph_order3_size1_left
    Assert.That(g.Order, Is.EqualTo 3)
    Assert.That(g.Size, Is.EqualTo 1)
    Assert.That(g.MaxDegree, Is.EqualTo 1)

    Assert.That(g.Vertices(), Is.EqualTo<Vertex seq>([ 0; 1; 2 ] |> Seq.map Verticise.it))

    Assert.That(g.ConnectedVertices(), Is.EqualTo<Vertex seq>([ 0; 1 ] |> Seq.map Verticise.it))

    Assert.That(g.MaxDegreeVertices(), Is.EqualTo<Vertex seq>(g.ConnectedVertices()))


let graph_order3_size1_long = graph [| [ 2 ]; []; [ 0 ] |]

[<Test>]
let order3_size1_long () =
    let g = graph_order3_size1_long
    Assert.That(g.Order, Is.EqualTo 3)
    Assert.That(g.Size, Is.EqualTo 1)
    Assert.That(g.MaxDegree, Is.EqualTo 1)

    Assert.That(g.Vertices(), Is.EqualTo<Vertex seq>([ 0; 1; 2 ] |> Seq.map Verticise.it))

    Assert.That(g.ConnectedVertices(), Is.EqualTo<Vertex seq>([ 0; 2 ] |> Seq.map Verticise.it))

    Assert.That(g.MaxDegreeVertices(), Is.EqualTo<Vertex seq>(g.ConnectedVertices()))


let graph_order3_size1_right = graph [| []; [ 2 ]; [ 1 ] |]

[<Test>]
let order3_size1_right () =
    let g = graph_order3_size1_right
    Assert.That(g.Order, Is.EqualTo 3)
    Assert.That(g.Size, Is.EqualTo 1)
    Assert.That(g.MaxDegree, Is.EqualTo 1)

    Assert.That(g.Vertices(), Is.EqualTo<Vertex seq>([ 0; 1; 2 ] |> Seq.map Verticise.it))

    Assert.That(g.ConnectedVertices(), Is.EqualTo<Vertex seq>([ 1; 2 ] |> Seq.map Verticise.it))

    Assert.That(g.MaxDegreeVertices(), Is.EqualTo<Vertex seq>(g.ConnectedVertices()))


let graph_order3_size2 = graph [| [ 1 ]; [ 0; 2 ]; [ 1 ] |]

[<Test>]
let order3_size2 () =
    let g = graph_order3_size2
    Assert.That(g.Order, Is.EqualTo 3)
    Assert.That(g.Size, Is.EqualTo 2)
    Assert.That(g.MaxDegree, Is.EqualTo 2)

    Assert.That(g.Vertices(), Is.EqualTo<Vertex seq>([ 0; 1; 2 ] |> Seq.map Verticise.it))

    Assert.That(g.ConnectedVertices(), Is.EqualTo<Vertex seq>(g.Vertices()))
    Assert.That(g.MaxDegreeVertices(), Is.EqualTo<Vertex seq>(seq { Vertex 1 }))


let graph_order3_size3 = graph [| [ 1; 2 ]; [ 0; 2 ]; [ 0; 1 ] |]

[<Test>]
let order3_size3 () =
    let g = graph_order3_size3
    Assert.That(g.Order, Is.EqualTo 3)
    Assert.That(g.Size, Is.EqualTo 3)
    Assert.That(g.MaxDegree, Is.EqualTo 2)

    Assert.That(g.Vertices(), Is.EqualTo<Vertex seq>([ 0; 1; 2 ] |> Seq.map Verticise.it))

    Assert.That(g.ConnectedVertices(), Is.EqualTo<Vertex seq>(g.Vertices()))
    Assert.That(g.MaxDegreeVertices(), Is.EqualTo<Vertex seq>(g.Vertices()))


let graph_order4_size2 = graph [| [ 1 ]; [ 0 ]; [ 3 ]; [ 2 ] |]

let graph_order4_size3_bus = graph [| [ 1 ]; [ 0; 2 ]; [ 1; 3 ]; [ 2 ] |]

// 0 - 1   2   3
// |\_____/    |
// |___________|
let graph_order4_size3_star = graph [| [ 1; 2; 3 ]; [ 0 ]; [ 0 ]; [ 0 ] |]

// 0 - 1 - 2
//      \ /
//       3
let graph_order4_size4_p = graph [| [ 1 ]; [ 0; 2; 3 ]; [ 1; 3 ]; [ 1; 2 ] |]

// 0 - 1
// |   |
// 3 - 2
let graph_order4_size4_square = graph [| [ 1; 3 ]; [ 0; 2 ]; [ 1; 3 ]; [ 0; 2 ] |]

// 0 - 1 - 2 - 3
// |\_____/    |
// |___________|
let graph_order4_size5 = graph [| [ 1; 2; 3 ]; [ 0; 2 ]; [ 0; 1; 3 ]; [ 0; 2 ] |]

//      _______
//     |       |
// 0 - 1 - 2 - 3
// |\_____/    |
// |___________|
let graph_order4_size6 =
    graph [| [ 1; 2; 3 ]; [ 0; 2; 3 ]; [ 0; 1; 3 ]; [ 0; 1; 2 ] |]

//  _______________
// |    ________   |
// |   | _____   \ |
// |   |/     \   \|
// 0 - 1 - 2 - 3   4
// |\_____/ \__|__/
// |___________|
let graph_order5_size6_penultimate =
    graph [| [ 1; 2; 3; 4 ]; [ 0; 2; 3; 4 ]; [ 0; 1; 3; 4 ]; [ 0; 1; 2 ]; [ 0; 1; 2 ] |]

let graph_sample =
    graph
        [| []
           [ 2; 3; 4 ]
           [ 1; 3; 4; 5 ]
           [ 1; 2; 4; 5 ]
           [ 1; 2; 3 ]
           [ 2; 3; 6; 7 ]
           [ 5; 7 ]
           [ 5; 6 ] |]

let graph_bigger =
    graph
        [| [ 1; 2; 3; 4; 6; 7 ]
           [ 0; 3; 6; 7; 8; 9 ]
           [ 0; 3; 5; 7; 8; 9 ]
           [ 0; 1; 2; 4; 9 ]
           [ 0; 3; 6; 7; 9 ]
           [ 2; 6 ]
           [ 0; 1; 4; 5; 9 ]
           [ 0; 1; 2; 4; 9 ]
           [ 1; 2 ]
           [ 1; 2; 3; 4; 6; 7 ] |]
