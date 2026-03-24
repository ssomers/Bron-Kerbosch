open BronKerbosch
open BronKerboschStudy
open System.Diagnostics

let sizes100 = seq { 2_000..50..3_000 } // max 4_950

let sizes10k =
    [ seq { 10_000..10_000..90_000 }; seq { 100_000..25_000..200_000 } ]
    |> Seq.concat

let sizes1M =
    [ seq { 2_000..2_000..18_000 }
      seq { 20_000..10_000..40_000 }
      seq { 50_000..50_000..200_000 }
      seq { 250_000..250_000..1_750_000 }
      seq { 2_000_000..1_000_000..3_000_000 } ]
    |> Seq.concat

let algos100 = fun _ -> Portfolio.all_algos

let algos10k = fun _ -> Portfolio.all_algos |> List.skip 1

let algos1M = fun _ -> [ BronKerbosch2GP.algorithm; BronKerbosch3GP.algorithm ]

BronKerboschStudy.Bk(BronKerboschStudy.WarmUp, "100", [ 2000 ], algos100, 3) // warm up
System.Threading.Thread.Sleep(321)

Debug.Fail("Run Release build for meaningful measurements")
BronKerboschStudy.Bk(BronKerboschStudy.Genuine, "100", sizes100, algos100, 5)
BronKerboschStudy.Bk(BronKerboschStudy.Genuine, "10k", sizes10k, algos10k, 3)
BronKerboschStudy.Bk(BronKerboschStudy.Genuine, "1M", sizes1M, algos1M, 3)
