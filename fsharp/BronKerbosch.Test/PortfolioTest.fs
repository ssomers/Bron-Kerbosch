module BronKerbosch.Test.Main

open NUnit.Framework
open BronKerbosch
open BronKerbosch.Test.UndirectedGraph

let bk (graph: UndirectedGraph, expected_cliques: int list list) : Unit =
    let expected_cliques =
        expected_cliques |> List.map (fun vertices -> vertices |> List.map vertex)

    Portfolio.all_algos
    |> List.iter (fun algo ->
        let mutable cliques = List.empty

        algo.exec graph (fun clique -> cliques <- clique :: cliques)
        let obtained_cliques = cliques |> Cliques.sort
        Assert.That(obtained_cliques, Is.EqualTo<Clique list> expected_cliques))

[<Test>]
let order0 () = bk (graph_order0, [])

[<Test>]
let order1 () = bk (graph_order1, [])

[<Test>]
let order2_isolated () = bk (graph_order2_isolated, [])

[<Test>]
let order2_connected () =
    bk (graph_order2_connected, [ [ 0; 1 ] ])

[<Test>]
let order3_size1_left () =
    bk (graph_order3_size1_left, [ [ 0; 1 ] ])

[<Test>]
let order3_size1_long () =
    bk (graph_order3_size1_long, [ [ 0; 2 ] ])

[<Test>]
let order3_size1_right () =
    bk (graph_order3_size1_right, [ [ 1; 2 ] ])

[<Test>]
let order3_size2 () =
    bk (graph_order3_size2, [ [ 0; 1 ]; [ 1; 2 ] ])

[<Test>]
let order3_size3 () =
    bk (graph_order3_size3, [ [ 0; 1; 2 ] ])

[<Test>]
let order4_size2 () =
    bk (graph_order4_size2, [ [ 0; 1 ]; [ 2; 3 ] ])

[<Test>]
let order4_size3_bus () =
    bk (graph_order4_size3_bus, [ [ 0; 1 ]; [ 1; 2 ]; [ 2; 3 ] ])

[<Test>]
let order4_size3_star () =
    bk (graph_order4_size3_star, [ [ 0; 1 ]; [ 0; 2 ]; [ 0; 3 ] ])

[<Test>]
let order4_size4_p () =
    bk (graph_order4_size4_p, [ [ 0; 1 ]; [ 1; 2; 3 ] ])

[<Test>]
let order4_size4_square () =
    bk (graph_order4_size4_square, [ [ 0; 1 ]; [ 0; 3 ]; [ 1; 2 ]; [ 2; 3 ] ])

[<Test>]
let order4_size5 () =
    bk (graph_order4_size5, [ [ 0; 1; 2 ]; [ 0; 2; 3 ] ])

[<Test>]
let order4_size6 () =
    bk (graph_order4_size6, [ [ 0; 1; 2; 3 ] ])

[<Test>]
let order5_size6_penultimate () =
    bk (graph_order5_size6_penultimate, [ [ 0; 1; 2; 3 ]; [ 0; 1; 2; 4 ] ])

[<Test>]
let sample () =
    bk (graph_sample, [ [ 1; 2; 3; 4 ]; [ 2; 3; 5 ]; [ 5; 6; 7 ] ])

[<Test>]
let bigger () =
    bk (
        graph_bigger,
        [ [ 0; 1; 3 ]
          [ 0; 1; 6 ]
          [ 0; 1; 7 ]
          [ 0; 2; 3 ]
          [ 0; 2; 7 ]
          [ 0; 3; 4 ]
          [ 0; 4; 6 ]
          [ 0; 4; 7 ]
          [ 1; 3; 9 ]
          [ 1; 6; 9 ]
          [ 1; 7; 9 ]
          [ 1; 8 ]
          [ 2; 3; 9 ]
          [ 2; 5 ]
          [ 2; 7; 9 ]
          [ 2; 8 ]
          [ 3; 4; 9 ]
          [ 4; 6; 9 ]
          [ 4; 7; 9 ]
          [ 5; 6 ] ]
    )
