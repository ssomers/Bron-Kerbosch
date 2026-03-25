namespace BronKerboschStudy

open BronKerbosch
open System.Diagnostics
open System.IO

type KnownUndirectedGraph =
    { graph: UndirectedGraph
      clique_count: int }

module RandomUndirectedGraph =
    let ReadEdges (path: string, order: int, size: int) : VertexSet array =
        let adjacencies = Array.init order (fun _ -> VertexSet.new_mutable (0))
        let mutable last_seen_line = 0

        for line in File.ReadLines path |> Seq.take size do
            last_seen_line <- last_seen_line + 1
            let fields = line.Split ' '

            let v, w =
                try
                    int (fields[0]), int (fields[1])
                with :? System.ArgumentException ->
                    failwith $"File {path} line {last_seen_line} contains bogus text {line}"

            VertexSet.insert_mutably (&adjacencies[v], Vertex w)
            VertexSet.insert_mutably (&adjacencies[w], Vertex v)

        if last_seen_line <> size then
            failwith $"{size} edges requested but only {last_seen_line} are listed in {path}"

        adjacencies


    let ReadStats (path: string, orderstr: string, size: int, min_clique_size: int) : int =
        Trace.Assert(min_clique_size >= 2)
        let prefix = $"{orderstr}\t{size}\t"

        let line =
            try
                File.ReadLines path
                |> Seq.skip 1
                |> Seq.filter (fun line -> line.StartsWith(prefix))
                |> Seq.head
            with :? System.ArgumentException ->
                failwith $"File {path} lacks order {orderstr} size {size}"

        let value = line.Substring(prefix.Length).Split('\t')[min_clique_size - 2]

        try
            int (value)
        with :? System.ArgumentException ->
            failwith $"File {path} has bogus line “{line}”"

    let public Read (orderstr: string, size: int, min_clique_size: int) : KnownUndirectedGraph =
        let order = NumbersGame.ParseInt orderstr
        let fullyMeshedSize = int64 order * (int64 order - 1L) / 2L

        if int64 size > fullyMeshedSize then
            failwith $"{order} nodes accommodate at most {fullyMeshedSize} edges"

        let edgesPath = $"..\\data\\random_edges_order_{orderstr}.txt"
        let statsPath = "..\\data\\random_stats.txt"
        let adjacencies = ReadEdges(edgesPath, order, size)
        let clique_count = ReadStats(statsPath, orderstr, size, min_clique_size)
        let graph = UndirectedGraph.ofAdjacencies adjacencies
        Debug.Assert(graph.Order = order)
        Debug.Assert(graph.Size = size)

        { graph = graph
          clique_count = clique_count }
