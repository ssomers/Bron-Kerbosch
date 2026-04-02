namespace BronKerbosch

type public CliqueConsumer = 
    { MinSize: int
      Receiver: Clique -> Unit
    }

    member inline this.accept(clique: Clique) : Unit =
        this.Receiver clique


