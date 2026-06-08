module BronKerbosch.Test.PriorityQueue

open BronKerbosch
open Expecto

let tests =
    testList
        "PriorityQueue"
        [ test "empty" {
              let mutable q = PriorityQueue.init 0
              Expect.isNone (PriorityQueue.Pop &q) "empty"
          }

          test "one" {
              let mutable q = PriorityQueue.init 1
              PriorityQueue.Put(&q, 1, true)
              Expect.isSome (PriorityQueue.Pop &q) ""
              Expect.isNone (PriorityQueue.Pop &q) "empty"
          }

          test "two_down" {
              let mutable q = PriorityQueue.init 2
              PriorityQueue.Put(&q, 2, 22)
              PriorityQueue.Put(&q, 1, 11)
              Expect.equal (PriorityQueue.Pop &q) (Some 11) ""
              Expect.equal (PriorityQueue.Pop &q) (Some 22) ""
              Expect.isNone (PriorityQueue.Pop &q) "empty"
          }

          test "two_up" {
              let mutable q = PriorityQueue.init 2
              PriorityQueue.Put(&q, 1, 22)
              PriorityQueue.Put(&q, 2, 11)
              Expect.equal (PriorityQueue.Pop &q) (Some 22) ""
              Expect.equal (PriorityQueue.Pop &q) (Some 11) ""
              Expect.isNone (PriorityQueue.Pop &q) "empty"
          } ]
