module BronKerbosch.Portfolio

open System.Diagnostics

let all_algos =
    [ BronKerbosch2GP.algorithm
      BronKerbosch3GP.algorithm
      BronKerbosch3GPX.algorithm ]

Trace.Assert(all_algos |> List.distinctBy (fun algo -> algo.name) |> List.length = all_algos.Length)
