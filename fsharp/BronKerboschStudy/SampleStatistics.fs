namespace BronKerboschStudy

type SampleStatisticsSamples< ^T when ^T: unmanaged and ^T: comparison> =
    { Count: int
      Max: ^T
      Min: ^T }

    static member AddTo(v: 'T, previous: SampleStatisticsSamples<'T> option) : SampleStatisticsSamples<'T> =
        match previous with
        | None -> { Count = 1; Max = v; Min = v }
        | Some(samples) ->
            { Count = samples.Count + 1
              Max = if v > samples.Max then v else samples.Max
              Min = if v < samples.Min then v else samples.Min }

type public SampleStatistics< ^T
    when ^T: unmanaged
    and ^T: comparison
    and ^T: (static member op_Explicit: ^T -> float)
    and ^T: (static member (-): ^T * ^T -> ^T)> =

    { Samples: SampleStatisticsSamples< ^T > option
      Sum: float
      SumOfSquares: float }

    member inline this.Mean() : float =
        match this.Samples with
        | None -> System.Double.NaN
        | Some(samples) ->
            System.Double.Clamp(this.Sum / float samples.Count, min = float samples.Min, max = float samples.Max)

    member inline this.Variance() : float =
        match this.Samples with
        | None -> System.Double.NaN
        | Some(samples) when samples.Count < 2 -> System.Double.NaN
        | Some(samples) ->
            System.Double.Max(0, this.SumOfSquares - this.Sum ** 2 / float samples.Count)
            / float (samples.Count - 1)

    member inline this.Deviation() : float =
        match this.Samples with
        | None -> System.Double.NaN
        | Some(samples) -> System.Double.Min(System.Double.Sqrt(this.Variance()), float (samples.Max - samples.Min))

    member inline this.Add(v: 'T) : SampleStatistics<'T> =
        { Samples = Some(SampleStatisticsSamples.AddTo(v, this.Samples))
          Sum = this.Sum + float v
          SumOfSquares = this.SumOfSquares + float v ** 2 }

module SampleStatistics =
    let inline Empty< ^T
        when ^T: unmanaged
        and ^T: comparison
        and ^T: (static member op_Explicit: ^T -> float)
        and ^T: (static member (-): ^T * ^T -> ^T)>
        ()
        : SampleStatistics< ^T > =
        { Samples = None
          Sum = 0
          SumOfSquares = 0 }
