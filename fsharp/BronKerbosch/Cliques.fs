module BronKerbosch.Cliques

open System.Diagnostics

exception MalformedCliques of string

let sort (cliques: Clique list) : Clique list =
    let compare_cliques (clique1: Clique) (clique2: Clique) : int =
        let common_length = min clique1.Length clique2.Length
        Debug.Assert(common_length > 0)

        match
            seq { 0 .. common_length - 1 }
            |> Seq.map (fun i -> clique1[i].index - clique2[i].index)
            |> Seq.where (fun diff -> diff <> 0)
            |> Seq.tryHead
        with
        | Some diff -> diff
        | None when clique1.Length = clique2.Length ->
            // Smells like a duplicate clique, but F# uses structural equality,
            // comparing elements to themselves.
            0
        | None -> raise (MalformedCliques(sprintf "overlapping cliques %A <> %A" clique1 clique2))

    let result =
        cliques
        |> List.map (fun clique -> clique |> Array.sort)
        |> List.sortWith compare_cliques

    // Because of structural equality, need to check for duplicates again.
    match
        result
        |> List.pairwise
        |> List.choose (fun (clique1, clique2) -> if clique1 = clique2 then Some clique1 else None)
    with
    | first :: rest -> raise (MalformedCliques(sprintf "%A and %d more clique(s) are duplicate" first rest.Length))
    | [] -> ()

    result
