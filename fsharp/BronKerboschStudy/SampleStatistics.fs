namespace BronKerboschStudy

type public SampleStatistics<'T
    when 'T: unmanaged
    and 'T: comparison
    and 'T: (static member op_Explicit: 'T -> float)
    and 'T: (static member (-): 'T * 'T -> 'T)> =

    { Count: int
      Max: 'T
      Min: 'T
      Sum: float
      SumOfSquares: float }

    member inline this.Mean() : float =
        let mean = this.Sum / float this.Count
        System.Double.Clamp(mean, min = float this.Min, max = float this.Max)

    member inline this.Variance() : float =
        if this.Count < 2 then
            nan
        else
            let variance =
                (this.SumOfSquares - this.Sum ** 2 / float this.Count) / float (this.Count - 1)

            max variance 0

    member inline this.Deviation() : float =
        let deviation = sqrt (this.Variance())
        min deviation (float (this.Max - this.Min))

    member inline this.Add(v: 'T) : SampleStatistics<'T> =
        { Count = this.Count + 1
          Max = if v > this.Max then v else this.Max
          Min = if v < this.Min then v else this.Min
          Sum = this.Sum + float v
          SumOfSquares = this.SumOfSquares + float v ** 2 }

    static member inline New(v: 'T) : SampleStatistics<'T> =
        { Count = 1
          Max = v
          Min = v
          Sum = float v
          SumOfSquares = float v ** 2 }

    static member inline NewOrAdd(v: 'T, previous: SampleStatistics<'T> option) : SampleStatistics<'T> =
        match previous with
        | None -> SampleStatistics.New(v)
        | Some samples -> samples.Add(v)
