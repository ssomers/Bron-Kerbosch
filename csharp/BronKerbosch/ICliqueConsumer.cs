using System.Collections.Immutable;

namespace BronKerbosch
{
    public interface ICliqueConsumer
    {
        bool IsAcceptedSize(int size);
        void Accept(ImmutableArray<Vertex> clique);
    }
}
