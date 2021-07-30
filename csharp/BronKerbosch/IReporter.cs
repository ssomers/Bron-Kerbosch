using System.Collections.Immutable;

namespace BronKerbosch
{
    public interface IReporter
    {
        void Record(ImmutableArray<Vertex> clique);
    }
}
