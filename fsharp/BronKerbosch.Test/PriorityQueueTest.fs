module BronKerbosch.Test.PriorityQueue

open NUnit.Framework
open BronKerbosch

[<Test>]
let empty () =
    let q = PriorityQueue.empty 0
    Assert.That(q.Pop(), Is.Null)

[<Test>]
let one () =
    let q = PriorityQueue.empty 1
    q.Put(1, true)
    Assert.That(q.Pop(), Is.EqualTo(Some true))
    Assert.That(q.Pop(), Is.Null)

[<Test>]
let two_down () =
    let q = PriorityQueue.empty 2
    q.Put(2, 22)
    q.Put(1, 11)
    Assert.That(q.Pop(), Is.EqualTo(Some 11))
    Assert.That(q.Pop(), Is.EqualTo(Some 22))
    Assert.That(q.Pop(), Is.Null)

[<Test>]
let two_up () =
    let q = PriorityQueue.empty 2
    q.Put(1, 22)
    q.Put(2, 11)
    Assert.That(q.Pop(), Is.EqualTo(Some 22))
    Assert.That(q.Pop(), Is.EqualTo(Some 11))
    Assert.That(q.Pop(), Is.Null)
