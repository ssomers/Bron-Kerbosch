// Bron-Kerbosch algorithm with degeneracy ordering,
// with nested searches choosing a pivot from candidates only (IK_GP).
module BronKerbosch3GP

open BronKerbosch

let public explore (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
    DegeneracyBased.explore MaxDegreeLocal graph consumer

let algorithm: Algorithm = { name = "Ver3½-GP"; exec = explore }
