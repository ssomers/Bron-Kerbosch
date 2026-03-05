namespace BronKerboschStudy

open BronKerbosch
open BronKerboschStudy
open System
open System.Diagnostics
open System.IO

exception InvalidResult of string

module BronKerboschStudy =
    let ic = Globalization.CultureInfo.InvariantCulture

    type TimedAlgorithm =
        { algo: Algorithm
          mutable seconds: SampleStatistics<float> option }

        static member inline is_for (algo: Algorithm) (ta: TimedAlgorithm) = ta.algo = algo

    type Run =
        | WarmUp
        | Genuine


    let BronKerboschTimed
        (run: Run, orderstr: string, size: int, timed_algos: TimedAlgorithm array, timing_samples: int)
        =
        //let warning_interval = 3000
        let sw = Stopwatch.StartNew()
        let graph = RandomUndirectedGraph.Read(orderstr, size)
        sw.Stop()

        if run = Genuine then
            printfn
                $"random graph of order {orderstr}, {size} edges, \
                {graph.clique_count} cliques: \
                (generating took {sw.ElapsedMilliseconds}ms)"

        let mutable baseline: Clique list option = None

        seq { 0..timing_samples }
        |> Seq.iter (fun sample ->
            timed_algos
            |> Seq.iter (fun timed_algo ->
                if sample = 0 then
                    (*
                    let mutable int warnings = 0;
                    let ticker = new Timer((_) =>
                    {
                        warnings += 1;
                        int secs = warnings * warning_interval / 1000;
                        Console.WriteLine($"  {secs} seconds in, {Portfolio.FuncNames[funcIndex]} is still busy collecting");
                    }, null, warning_interval, warning_interval);
                    *)
                    let mutable cliques = List.empty
                    timed_algo.algo.exec graph.graph (fun clique -> cliques <- clique :: cliques)
                    (*
                    ticker.Dispose()
                    *)
                    if cliques.Length <> graph.clique_count then
                        raise (InvalidResult($"Expected {graph.clique_count} cliques, got {cliques.Length}"))

                    let current_result = cliques |> Cliques.sort

                    match baseline with
                    | None -> baseline <- Some current_result
                    | Some first_result when current_result = first_result -> ()
                    | _ -> raise (InvalidResult("Got unexpected cliques"))
                else
                    let mutable cliques = 0
                    sw.Restart()
                    timed_algo.algo.exec graph.graph (fun clique -> cliques <- 1 + cliques)
                    sw.Stop()
                    let secs = float sw.ElapsedMilliseconds / 1e3

                    if cliques <> baseline.Value.Length then
                        failwith "unstable results"

                    timed_algo.seconds <- Some(SampleStatistics.NewOrAdd(secs, timed_algo.seconds))))

        if run = Genuine then
            timed_algos
            |> Seq.iter (fun timed_algo ->
                let name = timed_algo.algo.name

                match timed_algo.seconds with
                | Some s ->
                    let mean = s.Mean()
                    let reldev = s.Deviation() / mean
                    printfn $"  {name, -10} {mean, 6:N3}s ± {reldev:P0}"
                | None -> failwith $"{name} executed but no time recorded")

    let Bk (run: Run, orderstr: string, sizes: int seq, includedVers: int -> Algorithm list, timed_samples: int) =
        let tmpfname = "tmp.csv"
        let fso = FileStreamOptions()
        fso.Mode <- FileMode.Create
        fso.Access <- FileAccess.Write
        let fo = new StreamWriter(tmpfname, Text.UTF8Encoding(false), fso)

        fo.WriteLine(
            "Size"
            :: (Portfolio.all_algos
                |> List.map (fun ver -> $"{ver.name} min,{ver.name} mean,{ver.name} max"))
            |> String.concat ","
        )

        sizes
        |> Seq.iter (fun size ->
            let timed_algos: TimedAlgorithm array =
                includedVers size
                |> Seq.map (fun ver -> { algo = ver; seconds = None })
                |> Array.ofSeq

            BronKerboschTimed(run, orderstr, size, timed_algos, timed_samples)

            fo.WriteLine(
                String.Format(ic, "{0}", size)
                :: (Portfolio.all_algos
                    |> List.map (fun ver ->
                        match timed_algos |> Array.tryFind (TimedAlgorithm.is_for ver) with
                        | None -> ",,"
                        | Some { seconds = Some s } -> String.Format(ic, "{0},{1},{2}", s.Min, s.Mean(), s.Max)
                        | Some { seconds = None; algo = ver } -> failwith $"{ver.name} ran but left no measurement"))
                |> String.concat ","
            ))

        fo.Close()

        let suffix =
            match run with
            | WarmUp -> "warmup"
            | Genuine -> orderstr

        let path = $"..\\bron_kerbosch_fsharp_order_{suffix}.csv"

        if File.Exists(path) then
            File.Delete(path)

        File.Move(tmpfname, path)
