namespace BronKerboschStudy

open BronKerbosch
open System.Diagnostics
open System.IO

exception RandomComplaint of string

type KnownUndirectedGraph =
    { graph: UndirectedGraph
      clique_count: int }

module RandomUndirectedGraph =
    let ReadEdges (path: string, orderstr: string, size: int) : Set<Vertex> array =
        let order = NumbersGame.ParseInt(orderstr)
        let adjacencies: Set<Vertex> array = Array.init order (fun _ -> Set.empty)
        let mut lines_read = 0

        let lines_read =
            File.ReadLines(path)
            |> Seq.indexed
            |> Seq.take size
            |> Seq.map (fun (lineindex, line) ->
                let fields = line.Split(' ')

                let v, w =
                    try
                        int (fields[0]), int (fields[1])
                    with :? System.ArgumentException ->
                        raise (RandomComplaint($"File {path} line {lineindex + 1} contains bogus text {line}"))

                adjacencies[v] <- adjacencies[v].Add({ index = w })
                adjacencies[w] <- adjacencies[w].Add({ index = v })
                lineindex + 1)
            |> Seq.last

        if lines_read = size then
            adjacencies
        else
            raise (RandomComplaint($"Exhausted generated list of {lines_read} edges in {path}"))


    let ReadStats (path: string, orderstr: string, size: int) : int =
        let prefix = $"{orderstr}\t{size}\t"

        let line =
            try
                File.ReadLines(path)
                |> Seq.skip 1
                |> Seq.filter (fun line -> line.StartsWith(prefix))
                |> Seq.head
            with :? System.ArgumentException ->
                raise (RandomComplaint($"File {path} lacks order {orderstr} size {size}"))

        let value = line.Substring(prefix.Length)

        try
            int (value)
        with :? System.ArgumentException ->
            raise (RandomComplaint($"File {path} has bogus line “{line}”"))

    let public Read (orderstr: string, size: int) : KnownUndirectedGraph =
        let order = NumbersGame.ParseInt(orderstr)
        let fullyMeshedSize = int64 order * (int64 order - 1L) / 2L

        if int64 size > fullyMeshedSize then
            raise (RandomComplaint($"{order} nodes accommodate at most {fullyMeshedSize} edges"))

        let edgesPath = $"..\\data\\random_edges_order_{orderstr}.txt"
        let statsPath = "..\\data\\random_stats.txt"
        let adjacencies = ReadEdges(edgesPath, orderstr, size)
        let clique_count = ReadStats(statsPath, orderstr, size)
        let graph = UndirectedGraph.ofAdjacencies adjacencies
        Debug.Assert(graph.Order = order)
        Debug.Assert(graph.Size = size)

        { graph = graph
          clique_count = clique_count }
