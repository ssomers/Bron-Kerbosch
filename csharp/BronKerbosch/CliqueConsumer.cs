using System.Collections.Immutable;
using System.Diagnostics;

namespace BronKerbosch
{
    public sealed class CliqueConsumer<TAccumulator>(int minCliqueSize, TAccumulator Accu)
        where TAccumulator : ICliqueAccumulator<TAccumulator>
    {
        public bool IsAcceptedSize(int size) => size >= minCliqueSize;

        public void Accept(ImmutableArray<Vertex> clique)
        {
            Debug.Assert(IsAcceptedSize(clique.Length));
            Accu.Add(clique);
        }
    }
}
