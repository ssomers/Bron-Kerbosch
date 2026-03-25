namespace BronKerboschStudy

open BronKerbosch
open BronKerboschStudy
open System
open System.Diagnostics
open System.IO
open System.Threading

module BronKerboschStudy =
    let forCSV = Globalization.CultureInfo.InvariantCulture
    let MIN_CLIQUE_SIZE = 3

    type TimedAlgorithm =
        { algo: Algorithm
          mutable seconds: SampleStatistics<float> option }

        static member inline is_for (algo: Algorithm) (ta: TimedAlgorithm) = ta.algo = algo

    type Run =
        | WarmUp
        | Genuine
        | OneOff


    let BronKerboschTimed
        (run: Run, orderstr: string, size: int, timing_samples: int, algos: Algorithm list)
        : TimedAlgorithm array =
        let warning_interval = 3000

        let sw = Stopwatch.StartNew()
        let graph = RandomUndirectedGraph.Read(orderstr, size, MIN_CLIQUE_SIZE)
        sw.Stop()

        match run with
        | WarmUp -> ()
        | Genuine
        | OneOff ->
            printfn
                $"random graph of order {orderstr}, {size} edges, \
                {graph.clique_count} cliques: \
                (generating took {sw.ElapsedMilliseconds}ms)"

        let timed_algos: TimedAlgorithm array =
            algos |> List.map (fun algo -> { algo = algo; seconds = None }) |> Array.ofList

        let mutable baseline: Clique list option = None

        for sample = 0 to timing_samples do
            for timed_algo in timed_algos do
                if sample = 0 then
                    let mutable cliques = List.empty

                    let consumer =
                        { MinSize = MIN_CLIQUE_SIZE
                          Receiver = fun clique -> cliques <- clique :: cliques }

                    let cts = new CancellationTokenSource()

                    let ticker =
                        async {
                            for warnings in 1..99 do
                                do! Async.Sleep warning_interval
                                let secs = warnings * warning_interval / 1000
                                printfn $"  {secs} seconds in, {timed_algo.algo.name} is still busy collecting"
                        }

                    Async.StartImmediate(ticker, cts.Token)
                    timed_algo.algo.exec graph.graph consumer
                    cts.Cancel()

                    if cliques.Length <> graph.clique_count then
                        failwith $"Expected {graph.clique_count} cliques, got {cliques.Length}"

                    let current_result = cliques |> Cliques.sort

                    match baseline with
                    | None -> baseline <- Some current_result
                    | Some first_result when current_result = first_result -> ()
                    | _ -> failwith "Got unexpected cliques"
                else
                    let mutable cliques = 0

                    let consumer =
                        { MinSize = MIN_CLIQUE_SIZE
                          Receiver = fun clique -> cliques <- 1 + cliques }

                    sw.Restart()
                    timed_algo.algo.exec graph.graph consumer
                    sw.Stop()
                    let secs = float sw.ElapsedMilliseconds / 1e3

                    if cliques <> baseline.Value.Length then
                        failwith "unstable results"

                    timed_algo.seconds <- Some(SampleStatistics.NewOrAdd(secs, timed_algo.seconds))

        match run with
        | WarmUp -> ()
        | Genuine
        | OneOff ->
            for timed_algo in timed_algos do
                printf $"  {timed_algo.algo.name, -10} "

                match timed_algo.seconds with
                | Some s ->
                    let mean = s.Mean()
                    let reldev = s.Deviation() / mean
                    printfn $"{mean, 6:N3}s ± {reldev:P0}"
                | None -> printfn $"DNF"

        timed_algos

    let Bk (run: Run, orderstr: string, sizes: int seq, includedAlgos: int -> Algorithm list, timed_samples: int) =
        let tmpfname = "tmp.csv"
        let fso = FileStreamOptions()
        fso.Mode <- FileMode.Create
        fso.Access <- FileAccess.Write
        let fo = new StreamWriter(tmpfname, Text.UTF8Encoding(false), fso)

        fo.WriteLine(
            "Size"
            :: (Portfolio.all_algos
                |> List.map (fun algo -> algo.name)
                |> List.map (fun name -> $"{name} min,{name} mean,{name} max"))
            |> String.concat ","
        )

        for size in sizes do
            let algos = includedAlgos size
            let timed_algos = BronKerboschTimed(run, orderstr, size, timed_samples, algos)

            fo.WriteLine(
                String.Format(forCSV, "{0}", size)
                :: (Portfolio.all_algos
                    |> List.map (fun algo ->
                        match timed_algos |> Array.tryFind (TimedAlgorithm.is_for algo) with
                        | None -> ",,"
                        | Some { seconds = Some s } -> String.Format(forCSV, "{0},{1},{2}", s.Min, s.Mean(), s.Max)
                        | Some { seconds = None } -> failwith $"{algo.name} requested but not measured"))

                |> String.concat ","
            )

        fo.Close()

        match
            (match run with
             | WarmUp -> Some("warmup")
             | Genuine -> Some(orderstr)
             | OneOff -> None)
        with
        | Some suffix ->
            let path = $"..\\bron_kerbosch_fsharp_order_{suffix}.csv"

            if File.Exists(path) then
                File.Delete(path)

            File.Move(tmpfname, path)
        | None -> ()
