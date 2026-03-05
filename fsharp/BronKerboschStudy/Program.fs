open BronKerbosch
open BronKerboschStudy
open System.Diagnostics

//let sizes100 = seq { 2_000..50..3_001 } // max 4_950
let sizes100 = seq { 2_000..50..2_501 }

let sizes10k =
    [ seq { 10_000..10_000..100_000 }; seq { 100_000..25_000..200_001 } ]
    |> Seq.concat

let sizes1M =
    [ seq { 500_000..250_000..2_000_000 }; seq { 2_000_000..1_000_000..5_000_001 } ]
    |> Seq.concat

let algos100 = fun _ -> Portfolio.all_algos
let algos10k = fun _ -> Portfolio.all_algos |> List.skip 1

let algos1M =
    fun size -> Portfolio.all_algos |> List.skip (if size > 2_000_000 then 2 else 1)

BronKerboschStudy.Bk(BronKerboschStudy.WarmUp, "100", [ 2000 ], algos100, 3) // warm up
System.Threading.Thread.Sleep(321)

Debug.Fail("Run Release build for meaningful measurements")

BronKerboschStudy.Bk(BronKerboschStudy.Genuine, "100", sizes100, algos100, 5)
(*
BronKerboschStudy.Bk(BronKerboschStudy.Genuine, "10k", sizes10k, algos10k, 3)
BronKerboschStudy.Bk(BronKerboschStudy.Genuine, "1M", sizes1M, algos1M, 3)
*)
