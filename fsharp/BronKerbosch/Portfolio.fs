namespace BronKerbosch

exception MalformedCliques of string

module Portfolio =
    let sort_cliques (cliques: List<Vertex array>) : List<Vertex array> =
        let compare_cliques (clique1: Vertex array) (clique2: Vertex array) : int =
            let common_indices = (min clique1.Length clique2.Length) - 1

            try
                seq { 0 .. common_indices - 1 }
                |> Seq.map (fun i -> clique1[i].index - clique2[i].index)
                |> Seq.where (fun diff -> diff <> 0)
                |> Seq.head
            with :? System.ArgumentException ->
                raise (MalformedCliques("overlapping or equal cliques $clique1 <> $clique2"))

        cliques
        |> List.map (fun clique -> clique |> Array.sort)
        |> List.sortWith compare_cliques

(*
    public static class Portfolio
    {
        public static readonly string[] FuncNames =
        [
            "Ver1½",
            "Ver2-GP", "Ver2½-GP", "Ver2½-GPX",
            "Ver3½-GP", "Ver3½-GPX",
            "Ver3½=GPc"
        ];

        public static void Explore<TVertexSet, TVertexSetMgr>(int funcIndex, UndirectedGraph<TVertexSet, TVertexSetMgr> graph, ICliqueConsumer consumer)
            where TVertexSet : ISet<Vertex>
            where TVertexSetMgr : IVertexSetMgr<TVertexSet>
        {
            switch (funcIndex)
            {
                case 0: BronKerbosch1<TVertexSet, TVertexSetMgr>.Explore(graph, consumer); break;
                case 1: BronKerbosch2aGP<TVertexSet, TVertexSetMgr>.Explore(graph, consumer); break;
                case 2: BronKerbosch2bGP<TVertexSet, TVertexSetMgr>.Explore(graph, consumer); break;
                case 3: BronKerbosch2bGPX<TVertexSet, TVertexSetMgr>.Explore(graph, consumer); break;
                case 4: BronKerbosch3GP<TVertexSet, TVertexSetMgr>.Explore(graph, consumer); break;
                case 5: BronKerbosch3GPX<TVertexSet, TVertexSetMgr>.Explore(graph, consumer); break;
                case 6: BronKerbosch3MT<TVertexSet, TVertexSetMgr>.Explore(graph, consumer); break;
                default: throw new ArgumentException("unknown func_index");
            }
        }

        public static void AssertSameCliques(List<ImmutableArray<Vertex>> lhs, List<ImmutableArray<Vertex>> rhs)
        {
            if (lhs.Count != rhs.Count)
            {
                throw new ArgumentException($"{lhs.Count} cliques <> {rhs.Count} cliques");
            }
            for (var i = 0; i < lhs.Count; ++i)
            {
                if (lhs[i].Length != rhs[i].Length)
                {
                    throw new ArgumentException($"clique #{i + 1}: length {lhs[i].Length} <> length {rhs[i].Length}");
                }
                for (var j = 0; j < lhs[i].Length; ++j)
                {
                    if (lhs[i][j] != rhs[i][j])
                    {
                        throw new ArgumentException($"clique #{i + 1}, vertex #{j + 1}: {lhs[i][j]} <> {rhs[i][j]}");
                    }
                }
            }
        }

        private static int Comparer(ImmutableArray<Vertex> lhs, ImmutableArray<Vertex> rhs)
        {
            if (Equals(lhs, rhs))
            {
                // Seriously, Sort sometimes compares an element with itself
                return 0;
            }

            for (var i = 0; i < lhs.Length && i < rhs.Length; ++i)
            {
                var d = lhs[i].CompareTo(rhs[i]);
                if (d != 0)
                {
                    return d;
                }
            }
            throw new ArgumentException(
                $"got overlapping or equal cliques (length {lhs.Length} <> length {rhs.Length})");
        }
    }
}
*)
