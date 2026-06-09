module BronKerboschStudy.Test.NumbersGameTest

open NUnit.Framework
open BronKerboschStudy

[<Test>]
let ParsePositiveInt () =
    Assert.That(NumbersGame.ParseInt("0"), Is.EqualTo 0)
    Assert.That(NumbersGame.ParseInt("123"), Is.EqualTo 123)
    Assert.That(NumbersGame.ParseInt("1k"), Is.EqualTo 1_000)
    Assert.That(NumbersGame.ParseInt("1M"), Is.EqualTo 1_000_000)
    Assert.That(NumbersGame.ParseInt("42M"), Is.EqualTo 42_000_000)

[<Test>]
let ParseNegativeInt () =
    Assert.That(NumbersGame.ParseInt("-1"), Is.EqualTo -1)
    Assert.That(NumbersGame.ParseInt("-1M"), Is.EqualTo -1_000_000)

[<Test>]
let ParseEmpty () =
    Assert.That(System.Action(fun _ -> NumbersGame.ParseInt("") |> ignore), Throws.Exception)

[<Test>]
let ParseUnknownSuffix () =
    Assert.That(System.Action(fun _ -> NumbersGame.ParseInt("1K") |> ignore), Throws.Exception)

[<Test>]
let ParseNonInt () =
    Assert.That(System.Action(fun _ -> NumbersGame.ParseInt("1.1") |> ignore), Throws.Exception)
