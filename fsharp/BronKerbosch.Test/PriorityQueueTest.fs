module BronKerbosch.Test.PriorityQueue

open NUnit.Framework
open BronKerbosch

[<Test>]
let empty () =
    let mutable q = PriorityQueue.init 0
    Assert.That(PriorityQueue.Pop &q, Is.Null)

[<Test>]
let one () =
    let mutable q = PriorityQueue.init 1
    PriorityQueue.Put(&q, 1, true)
    Assert.That(PriorityQueue.Pop &q, Is.EqualTo(Some true))
    Assert.That(PriorityQueue.Pop &q, Is.Null)

[<Test>]
let two_down () =
    let mutable q = PriorityQueue.init 2
    PriorityQueue.Put(&q, 2, 22)
    PriorityQueue.Put(&q, 1, 11)
    Assert.That(PriorityQueue.Pop &q, Is.EqualTo(Some 11))
    Assert.That(PriorityQueue.Pop &q, Is.EqualTo(Some 22))
    Assert.That(PriorityQueue.Pop &q, Is.Null)

[<Test>]
let two_up () =
    let mutable q = PriorityQueue.init 2
    PriorityQueue.Put(&q, 1, 22)
    PriorityQueue.Put(&q, 2, 11)
    Assert.That(PriorityQueue.Pop &q, Is.EqualTo(Some 22))
    Assert.That(PriorityQueue.Pop &q, Is.EqualTo(Some 11))
    Assert.That(PriorityQueue.Pop &q, Is.Null)
