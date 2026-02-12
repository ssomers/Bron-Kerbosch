using System.Collections.Immutable;

namespace BronKerbosch
{
    public interface ICliqueConsumer
    {
        void Accept(ImmutableArray<Vertex> clique);
    }
}
