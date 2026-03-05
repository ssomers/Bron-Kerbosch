module BronKerboschStudy.Test.SampleStatistics

open NUnit.Framework
open BronKerboschStudy


[<Test>]
let Stats_1 () =
    let s: SampleStatistics<int> = SampleStatistics.New(-1)
    Assert.That(s.Mean().Equals(-1))
    Assert.That(System.Double.IsNaN(s.Variance()))
    Assert.That(System.Double.IsNaN(s.Deviation()))


[<Test>]
let Stats_2 () =
    let s: SampleStatistics<int> = SampleStatistics.New(-1).Add(+1)
    Assert.That(s.Mean().Equals(0))
    Assert.That(s.Variance(), Is.EqualTo(2.0))
    Assert.That(s.Deviation(), Is.EqualTo(sqrt 2.0))


[<Test>]
let Stats_3 () =
    let s: SampleStatistics<int> = (SampleStatistics.New 89).Add(90).Add(91)
    Assert.That(s.Mean(), Is.EqualTo(90))
    Assert.That(s.Variance(), Is.EqualTo(1))
    Assert.That(s.Deviation(), Is.EqualTo(1))


[<Test>]
let Stats_9 () =
    let s: SampleStatistics<int> =
        (SampleStatistics.New(2), [ 4; 4; 4; 5; 5; 5; 7; 9 ])
        ||> List.fold (fun stats sample -> stats.Add sample)

    Assert.That(s.Mean(), Is.EqualTo(5))
    Assert.That(s.Variance(), Is.EqualTo(4))
    Assert.That(s.Deviation(), Is.EqualTo(2))
