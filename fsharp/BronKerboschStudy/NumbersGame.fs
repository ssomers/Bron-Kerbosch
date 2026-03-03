namespace BronKerboschStudy

module NumbersGame =
    let ParseInt (numstr: string) : int =
        let suffix_length, factor =
            match numstr with
            | s when s.EndsWith('k') -> 1, 1_000
            | s when s.EndsWith('M') -> 1, 1_000_000
            | _ -> 0, 1

        int (numstr[0 .. numstr.Length - 1 - suffix_length]) * factor
