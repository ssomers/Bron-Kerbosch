module BronKerboschStudy.Test.SampleStatistics

open NUnit.Framework
open BronKerboschStudy


[<Test>]
let Stats_0 () =
    let s = SampleStatistics.Empty<int>();
    Assert.That(System.Double.IsNaN(s.Mean()));
    Assert.That(System.Double.IsNaN(s.Variance()));
    Assert.That(System.Double.IsNaN(s.Deviation()));

[<Test>]
let Stats_1() =
    let s = SampleStatistics.Empty<int>().Add(-1);
    Assert.That(s.Mean().Equals(-1));
    Assert.That(System.Double.IsNaN(s.Variance()));
    Assert.That(System.Double.IsNaN(s.Deviation()));


[<Test>]
let Stats_2() =
    let s = SampleStatistics.Empty<int>().Add(-1).Add(+1);
    Assert.That(s.Mean().Equals(0));
    Assert.That(s.Variance(), Is.EqualTo(2));
    Assert.That(s.Deviation(), Is.EqualTo(System.Double.Sqrt(2)));


[<Test>]
let Stats_3() =
    let s = SampleStatistics.Empty<int>().Add(89).Add(90).Add(91);
    Assert.That(s.Mean(), Is.EqualTo(90));
    Assert.That(s.Variance(), Is.EqualTo(1));
    Assert.That(s.Deviation(), Is.EqualTo(1));


[<Test>]
let Stats_9() =
    let mutable s = SampleStatistics.Empty<int>();
    s <- s.Add(2)
    s <- s.Add(4)
    s <- s.Add(4)
    s <- s.Add(4)
    s <- s.Add(5)
    s <- s.Add(5)
    s <- s.Add(5)
    s <- s.Add(7)
    s <- s.Add(9)
    Assert.That(s.Mean(), Is.EqualTo(5));
    Assert.That(s.Variance(), Is.EqualTo(4));
    Assert.That(s.Deviation(), Is.EqualTo(2));
