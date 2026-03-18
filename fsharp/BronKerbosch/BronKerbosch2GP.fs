// Bron-Kerbosch algorithm with pivot of highest degree (IK_GP)
module BronKerbosch2GP

open BronKerbosch

let public explore (graph: UndirectedGraph) (consumer: CliqueConsumer) : Unit =
    PivotBased.explore MaxDegreeLocal graph consumer

let algorithm: Algorithm = { name = "Ver2½-GP"; exec = explore }
