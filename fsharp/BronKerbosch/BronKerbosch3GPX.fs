// Bron-Kerbosch algorithm with degeneracy ordering,
// choosing a pivot from both candidates and excluded vertices (IK_GPX).
module BronKerbosch3GPX

open BronKerbosch

let public explore (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
    DegeneracyBased.explore MaxDegreeLocalX graph consumer

let algorithm: Algorithm = { name = "Ver3½-GPX"; exec = explore }
