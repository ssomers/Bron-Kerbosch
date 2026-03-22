module BronKerbosch.Test.VertexSet

open NUnit.Framework
open BronKerbosch

let nix = VertexSet.empty
let one = VertexSet.singleton (Vertex 1)
let two: VertexSet = [ 1; 2 ] |> Seq.map Verticise.it |> VertexSet.ofSeq

let six: VertexSet = [ 0; 1; 2; 3; 4; 5 ] |> Seq.map Verticise.it |> VertexSet.ofSeq

[<Test>]
let is_disjoint () =
    Assert.That(VertexSet.is_disjoint nix one)
    Assert.That(VertexSet.is_disjoint one nix)
    Assert.That(VertexSet.is_disjoint nix two)
    Assert.That(VertexSet.is_disjoint two nix)
    Assert.That(VertexSet.is_disjoint nix six)
    Assert.That(VertexSet.is_disjoint six nix)
    Assert.That(not (VertexSet.is_disjoint one two))
    Assert.That(not (VertexSet.is_disjoint two one))
    Assert.That(not (VertexSet.is_disjoint one six))
    Assert.That(not (VertexSet.is_disjoint six one))
    Assert.That(not (VertexSet.is_disjoint six two))
    Assert.That(not (VertexSet.is_disjoint two six))
    Assert.That(not (VertexSet.is_disjoint one one))
    Assert.That(not (VertexSet.is_disjoint two two))
    Assert.That(not (VertexSet.is_disjoint six six))

[<Test>]
let intersection () =
    Assert.That(VertexSet.intersect nix one, Is.Empty)
    Assert.That(VertexSet.intersect one nix, Is.Empty)
    Assert.That(VertexSet.intersect nix two, Is.Empty)
    Assert.That(VertexSet.intersect two nix, Is.Empty)
    Assert.That(VertexSet.intersect nix six, Is.Empty)
    Assert.That(VertexSet.intersect six nix, Is.Empty)
    Assert.That(VertexSet.intersect one two, Is.EqualTo<VertexSet> one)
    Assert.That(VertexSet.intersect two one, Is.EqualTo<VertexSet> one)
    Assert.That(VertexSet.intersect one six, Is.EqualTo<VertexSet> one)
    Assert.That(VertexSet.intersect six one, Is.EqualTo<VertexSet> one)
    Assert.That(VertexSet.intersect two six, Is.EqualTo<VertexSet> two)
    Assert.That(VertexSet.intersect six two, Is.EqualTo<VertexSet> two)
    Assert.That(VertexSet.intersect one one, Is.EqualTo<VertexSet> one)
    Assert.That(VertexSet.intersect two two, Is.EqualTo<VertexSet> two)
    Assert.That(VertexSet.intersect six six, Is.EqualTo<VertexSet> six)

[<Test>]
let overlap () =
    Assert.That(VertexSet.overlap nix one, Is.EqualTo 0)
    Assert.That(VertexSet.overlap one nix, Is.EqualTo 0)
    Assert.That(VertexSet.overlap nix two, Is.EqualTo 0)
    Assert.That(VertexSet.overlap two nix, Is.EqualTo 0)
    Assert.That(VertexSet.overlap nix six, Is.EqualTo 0)
    Assert.That(VertexSet.overlap six nix, Is.EqualTo 0)
    Assert.That(VertexSet.overlap one two, Is.EqualTo 1)
    Assert.That(VertexSet.overlap two one, Is.EqualTo 1)
    Assert.That(VertexSet.overlap one six, Is.EqualTo 1)
    Assert.That(VertexSet.overlap six one, Is.EqualTo 1)
    Assert.That(VertexSet.overlap two six, Is.EqualTo 2)
    Assert.That(VertexSet.overlap six two, Is.EqualTo 2)
    Assert.That(VertexSet.overlap one one, Is.EqualTo 1)
    Assert.That(VertexSet.overlap two two, Is.EqualTo 2)
    Assert.That(VertexSet.overlap six six, Is.EqualTo 6)

[<Test>]
let difference () =
    Assert.That(VertexSet.difference nix one, Is.Empty)
    Assert.That(VertexSet.difference nix two, Is.Empty)
    Assert.That(VertexSet.difference nix six, Is.Empty)
    Assert.That(VertexSet.difference one one, Is.Empty)
    Assert.That(VertexSet.difference one two, Is.Empty)
    Assert.That(VertexSet.difference one six, Is.Empty)
    Assert.That(VertexSet.difference two two, Is.Empty)
    Assert.That(VertexSet.difference two six, Is.Empty)
    Assert.That(VertexSet.difference six six, Is.Empty)
    Assert.That(VertexSet.difference one nix, Is.EqualTo<VertexSet> one)
    Assert.That(VertexSet.difference two nix, Is.EqualTo<VertexSet> two)
    Assert.That(VertexSet.difference six nix, Is.EqualTo<VertexSet> six)
    Assert.That(VertexSet.difference two one, Is.EqualTo<VertexSet>(VertexSet.singleton (Vertex 2)))

    Assert.That(
        VertexSet.difference six one,
        Is.EqualTo<VertexSet>([ 0; 2; 3; 4; 5 ] |> Seq.map Verticise.it |> VertexSet.ofSeq)
    )

    Assert.That(
        VertexSet.difference six two,
        Is.EqualTo<VertexSet>([ 0; 3; 4; 5 ] |> Seq.map Verticise.it |> VertexSet.ofSeq)
    )

[<Test>]
let pop_arbitrary_1 () =
    let x, rest = VertexSet.pop_arbitrary one
    Assert.That(x.Value, Is.EqualTo(Vertex 1))
    Assert.That(rest, Is.Empty)
    let x, rest = VertexSet.pop_arbitrary rest
    Assert.That(x, Is.Null)
    Assert.That(rest, Is.Empty)

[<Test>]
let pop_arbitrary_2 () =
    let x, rest = VertexSet.pop_arbitrary two
    let y, rest = VertexSet.pop_arbitrary rest
    Assert.That(min x.Value y.Value, Is.EqualTo(Vertex 1))
    Assert.That(max x.Value y.Value, Is.EqualTo(Vertex 2))
    Assert.That(rest, Is.Empty)
    let x, rest = VertexSet.pop_arbitrary rest
    Assert.That(x, Is.Null)
    Assert.That(rest, Is.Empty)
