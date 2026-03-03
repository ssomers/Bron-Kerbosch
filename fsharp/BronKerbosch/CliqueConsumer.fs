namespace BronKerbosch

type CliqueConsumer = Vertex array -> Unit

exception MalformedCliques of string

module CliqueJanitor =
    let sort_cliques (cliques: List<Vertex array>) : List<Vertex array> =
        let compare_cliques (clique1: Vertex array) (clique2: Vertex array) : int =
            let common_length = min clique1.Length clique2.Length
            assert (common_length > 0)

            try
                seq { 0 .. common_length - 1 }
                |> Seq.map (fun i -> clique1[i].index - clique2[i].index)
                |> Seq.where (fun diff -> diff <> 0)
                |> Seq.head
            with :? System.ArgumentException ->
                raise (MalformedCliques(sprintf "overlapping or equal cliques %A <> %A" clique1 clique2))

        cliques
        |> List.map (fun clique -> clique |> Array.sort)
        |> List.sortWith compare_cliques


    let equal_cliques (expected: Vertex array seq) (obtained: Vertex array list) : bool =
        expected
        |> Seq.indexed
        |> Seq.forall (fun (index: int, clique: Vertex array) -> clique = obtained[index])
