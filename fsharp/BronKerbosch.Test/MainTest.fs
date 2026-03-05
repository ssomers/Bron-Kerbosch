module BronKerbosch.Test.Main

open NUnit.Framework
open BronKerbosch

let Bk (adjacencies: int list array, expected_cliques: int array list) : Unit =
    let adjacencies =
        adjacencies
        |> Array.map (fun neighbours -> neighbours |> Set.ofList |> Set.map vertex)

    let expected_cliques =
        expected_cliques |> List.map (fun vertices -> vertices |> Array.map vertex)

    let graph = UndirectedGraph.ofAdjacencies adjacencies

    Portfolio.all_algos
    |> List.iter (fun algo ->
        let mutable cliques = List.empty

        algo.exec graph (fun clique -> cliques <- clique :: cliques)
        let obtained_cliques = cliques |> Cliques.sort
        Assert.That(obtained_cliques, Is.EqualTo<Clique list>(expected_cliques)))

[<Test>]
let TestOrder0 () = Bk([||], [])

[<Test>]
let TestOrder1 () = Bk([| [] |], [])

[<Test>]
let TestOrder2_Isolated () = Bk([| []; [] |], [])

[<Test>]
let TestOrder2_Connected () = Bk([| [ 1 ]; [ 0 ] |], [ [| 0; 1 |] ])

[<Test>]
let TestOrder3_Size1_Left () =
    Bk([| [ 1 ]; [ 0 ]; [] |], [ [| 0; 1 |] ])

[<Test>]
let TestOrder3_Size1_Long () =
    Bk([| [ 2 ]; []; [ 0 ] |], [ [| 0; 2 |] ])

[<Test>]
let TestOrder3_Size1_Right () =
    Bk([| []; [ 2 ]; [ 1 ] |], [ [| 1; 2 |] ])

[<Test>]
let TestOrder3_Size2 () =
    Bk([| [ 1 ]; [ 0; 2 ]; [ 1 ] |], [ [| 0; 1 |]; [| 1; 2 |] ])

[<Test>]
let TestOrder3_Size3 () =
    Bk([| [ 1; 2 ]; [ 0; 2 ]; [ 0; 1 ] |], [ [| 0; 1; 2 |] ])

[<Test>]
let TestOrder4_Size2 () =
    Bk([| [ 1 ]; [ 0 ]; [ 3 ]; [ 2 ] |], [ [| 0; 1 |]; [| 2; 3 |] ])

[<Test>]
let TestOrder4_Size3_Bus () =
    Bk([| [ 1 ]; [ 0; 2 ]; [ 1; 3 ]; [ 2 ] |], [ [| 0; 1 |]; [| 1; 2 |]; [| 2; 3 |] ])

[<Test>]
let TestOrder4_Size3_Star () =
    Bk([| [ 1; 2; 3 ]; [ 0 ]; [ 0 ]; [ 0 ] |], [ [| 0; 1 |]; [| 0; 2 |]; [| 0; 3 |] ])

[<Test>]
let TestOrder4_Size4_p () =
    Bk([| [ 1 ]; [ 0; 2; 3 ]; [ 1; 3 ]; [ 1; 2 ] |], [ [| 0; 1 |]; [| 1; 2; 3 |] ])

[<Test>]
let TestOrder4_Size4_Square () =
    Bk([| [ 1; 3 ]; [ 0; 2 ]; [ 1; 3 ]; [ 0; 2 ] |], [ [| 0; 1 |]; [| 0; 3 |]; [| 1; 2 |]; [| 2; 3 |] ])

[<Test>]
let TestOrder4_Size5 () =
    Bk([| [ 1; 2; 3 ]; [ 0; 2 ]; [ 0; 1; 3 ]; [ 0; 2 ] |], [ [| 0; 1; 2 |]; [| 0; 2; 3 |] ])

[<Test>]
let TestOrder4_Size6 () =
    Bk([| [ 1; 2; 3 ]; [ 0; 2; 3 ]; [ 0; 1; 3 ]; [ 0; 1; 2 ] |], [ [| 0; 1; 2; 3 |] ])

[<Test>]
let TestOrder4_Size6_Penultimate () =
    Bk(
        [| [ 1; 2; 3; 4 ]; [ 0; 2; 3; 4 ]; [ 0; 1; 3; 4 ]; [ 0; 1; 2 ]; [ 0; 1; 2 ] |],
        [ [| 0; 1; 2; 3 |]; [| 0; 1; 2; 4 |] ]
    )

[<Test>]
let TestSample () =
    Bk(
        [| []
           [ 2; 3; 4 ]
           [ 1; 3; 4; 5 ]
           [ 1; 2; 4; 5 ]
           [ 1; 2; 3 ]
           [ 2; 3; 6; 7 ]
           [ 5; 7 ]
           [ 5; 6 ] |],

        [ [| 1; 2; 3; 4 |]; [| 2; 3; 5 |]; [| 5; 6; 7 |] ]
    )

[<Test>]
let TestBigger () =
    Bk(
        [| [ 1; 2; 3; 4; 6; 7 ]
           [ 0; 3; 6; 7; 8; 9 ]
           [ 0; 3; 5; 7; 8; 9 ]
           [ 0; 1; 2; 4; 9 ]
           [ 0; 3; 6; 7; 9 ]
           [ 2; 6 ]
           [ 0; 1; 4; 5; 9 ]
           [ 0; 1; 2; 4; 9 ]
           [ 1; 2 ]
           [ 1; 2; 3; 4; 6; 7 ] |],

        [ [| 0; 1; 3 |]
          [| 0; 1; 6 |]
          [| 0; 1; 7 |]
          [| 0; 2; 3 |]
          [| 0; 2; 7 |]
          [| 0; 3; 4 |]
          [| 0; 4; 6 |]
          [| 0; 4; 7 |]
          [| 1; 3; 9 |]
          [| 1; 6; 9 |]
          [| 1; 7; 9 |]
          [| 1; 8 |]
          [| 2; 3; 9 |]
          [| 2; 5 |]
          [| 2; 7; 9 |]
          [| 2; 8 |]
          [| 3; 4; 9 |]
          [| 4; 6; 9 |]
          [| 4; 7; 9 |]
          [| 5; 6 |] ]
    )
