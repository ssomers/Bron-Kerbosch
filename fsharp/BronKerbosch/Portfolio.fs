module BronKerbosch.Portfolio

let all_algos = [ BronKerbosch1.algorithm; BronKerbosch2GP.algorithm; BronKerbosch3GP.algorithm; BronKerbosch3GPX.algorithm ]
assert (all_algos |> List.distinctBy (fun algo -> algo.name) |> List.length = all_algos.Length)