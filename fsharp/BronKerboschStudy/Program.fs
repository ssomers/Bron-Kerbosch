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
      seq { 2_000_000..1_000_000..4_000_000 } ]
    |> Seq.concat

let algos100 = fun _ -> Portfolio.all_algos

let algos10k = fun _ -> Portfolio.all_algos

let algos1M = fun _ -> [ BronKerbosch2GP.algorithm; BronKerbosch3GP.algorithm ]

//Benchmark.Do(Run.OneOff, "1M", [ 500_000 ], (fun _ -> []), 0)

Benchmark.Do(Run.WarmUp, "100", [ 2000 ], algos100, 3) // warm up
System.Threading.Thread.Sleep(321)

Debug.Fail("Run Release build for meaningful measurements")
Benchmark.Do(Run.Genuine, "100", sizes100, algos100, 5)
Benchmark.Do(Run.Genuine, "10k", sizes10k, algos10k, 3)
Benchmark.Do(Run.Genuine, "1M", sizes1M, algos1M, 3)
