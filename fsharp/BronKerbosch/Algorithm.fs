namespace BronKerbosch

[<ReferenceEquality>]
type Algorithm =
    { name: string
      exec: UndirectedGraph -> CliqueConsumer -> Unit }
