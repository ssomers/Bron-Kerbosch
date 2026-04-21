namespace BronKerbosch

open System.Diagnostics

type public CliqueConsumer =
    { MinSize: int
      Receiver: Clique -> Unit }

    member inline this.accept(clique: Clique) : Unit =
        Debug.Assert(clique.Length >= this.MinSize)
        this.Receiver clique
