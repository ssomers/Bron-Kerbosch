module BronKerbosch.Test.VertexSet

open NUnit.Framework
open BronKerbosch

let empty: Set<Vertex> = Set.empty
let one: Set<Vertex> = Set.singleton ({ index = 1 })
let two: Set<Vertex> = [ { index = 1 }; { index = 2 } ] |> Set.ofList

let six: Set<Vertex> =
    [ { index = 0 }
      { index = 1 }
      { index = 2 }
      { index = 3 }
      { index = 4 }
      { index = 5 } ]
    |> Set.ofList

[<Test>]
let PopArbitrary1 () =
    let x, rest = VertexSet.pop_arbitrary one
    Assert.That(x.Value.index, Is.EqualTo(1))
    Assert.That(rest, Is.Empty)

[<Test>]
let PopArbitrary2 () =
    let x, rest = VertexSet.pop_arbitrary two
    let y, rest = VertexSet.pop_arbitrary rest
    Assert.That(min x.Value.index y.Value.index, Is.EqualTo(1))
    Assert.That(max x.Value.index y.Value.index, Is.EqualTo(2))
    Assert.That(rest, Is.Empty)

[<Test>]
let Overlaps () =
    Assert.That(VertexSet.is_disjoint empty one)
    Assert.That(VertexSet.is_disjoint one empty)
    Assert.That(VertexSet.is_disjoint empty two)
    Assert.That(VertexSet.is_disjoint two empty)
    Assert.That(VertexSet.is_disjoint empty six)
    Assert.That(VertexSet.is_disjoint six empty)
    Assert.That(not (VertexSet.is_disjoint one two))
    Assert.That(not (VertexSet.is_disjoint two one))
    Assert.That(not (VertexSet.is_disjoint one six))
    Assert.That(not (VertexSet.is_disjoint six one))
    Assert.That(not (VertexSet.is_disjoint two six))
    Assert.That(not (VertexSet.is_disjoint six two))
    Assert.That(not (VertexSet.is_disjoint one one))
    Assert.That(not (VertexSet.is_disjoint two two))
    Assert.That(not (VertexSet.is_disjoint six six))

[<Test>]
let intersection () =
    Assert.That(VertexSet.intersect empty one, Is.Empty)
    Assert.That(VertexSet.intersect one empty, Is.Empty)
    Assert.That(VertexSet.intersect empty two, Is.Empty)
    Assert.That(VertexSet.intersect two empty, Is.Empty)
    Assert.That(VertexSet.intersect empty six, Is.Empty)
    Assert.That(VertexSet.intersect six empty, Is.Empty)
    Assert.That(VertexSet.intersect one two, Is.EqualTo<Set<Vertex>>(one))
    Assert.That(VertexSet.intersect two one, Is.EqualTo<Set<Vertex>>(one))
    Assert.That(VertexSet.intersect one six, Is.EqualTo<Set<Vertex>>(one))
    Assert.That(VertexSet.intersect six one, Is.EqualTo<Set<Vertex>>(one))
    Assert.That(VertexSet.intersect two six, Is.EqualTo<Set<Vertex>>(two))
    Assert.That(VertexSet.intersect six two, Is.EqualTo<Set<Vertex>>(two))
    Assert.That(VertexSet.intersect one one, Is.EqualTo<Set<Vertex>>(one))
    Assert.That(VertexSet.intersect two two, Is.EqualTo<Set<Vertex>>(two))
    Assert.That(VertexSet.intersect six six, Is.EqualTo<Set<Vertex>>(six))

[<Test>]
let IntersectCount () =
    Assert.That(VertexSet.intersection_size empty one, Is.EqualTo(0))
    Assert.That(VertexSet.intersection_size one empty, Is.EqualTo(0))
    Assert.That(VertexSet.intersection_size empty two, Is.EqualTo(0))
    Assert.That(VertexSet.intersection_size two empty, Is.EqualTo(0))
    Assert.That(VertexSet.intersection_size empty six, Is.EqualTo(0))
    Assert.That(VertexSet.intersection_size six empty, Is.EqualTo(0))
    Assert.That(VertexSet.intersection_size one two, Is.EqualTo(1))
    Assert.That(VertexSet.intersection_size two one, Is.EqualTo(1))
    Assert.That(VertexSet.intersection_size one six, Is.EqualTo(1))
    Assert.That(VertexSet.intersection_size six one, Is.EqualTo(1))
    Assert.That(VertexSet.intersection_size two six, Is.EqualTo(2))
    Assert.That(VertexSet.intersection_size six two, Is.EqualTo(2))
    Assert.That(VertexSet.intersection_size one one, Is.EqualTo(1))
    Assert.That(VertexSet.intersection_size two two, Is.EqualTo(2))
    Assert.That(VertexSet.intersection_size six six, Is.EqualTo(6))

[<Test>]
let Difference () =
    Assert.That(VertexSet.difference empty one, Is.EqualTo<Set<Vertex>>(empty))
    Assert.That(VertexSet.difference empty two, Is.EqualTo<Set<Vertex>>(empty))
    Assert.That(VertexSet.difference empty six, Is.EqualTo<Set<Vertex>>(empty))
    Assert.That(VertexSet.difference one one, Is.EqualTo<Set<Vertex>>(empty))
    Assert.That(VertexSet.difference one two, Is.EqualTo<Set<Vertex>>(empty))
    Assert.That(VertexSet.difference one six, Is.EqualTo<Set<Vertex>>(empty))
    Assert.That(VertexSet.difference two two, Is.EqualTo<Set<Vertex>>(empty))
    Assert.That(VertexSet.difference two six, Is.EqualTo<Set<Vertex>>(empty))
    Assert.That(VertexSet.difference six six, Is.EqualTo<Set<Vertex>>(empty))
    Assert.That(VertexSet.difference one empty, Is.EqualTo<Set<Vertex>>(one))
    Assert.That(VertexSet.difference two empty, Is.EqualTo<Set<Vertex>>(two))
    Assert.That(VertexSet.difference six empty, Is.EqualTo<Set<Vertex>>(six))
    Assert.That(VertexSet.difference two one, Is.EqualTo<Set<Vertex>>(Set.singleton { index = 2 }))

    Assert.That(
        VertexSet.difference six one,
        Is.EqualTo<Set<Vertex>>(
            [ { index = 0 }; { index = 2 }; { index = 3 }; { index = 4 }; { index = 5 } ]
            |> Set.ofList
        )
    )

    Assert.That(
        VertexSet.difference six two,
        Is.EqualTo<Set<Vertex>>([ { index = 0 }; { index = 3 }; { index = 4 }; { index = 5 } ] |> Set.ofList)
    )
