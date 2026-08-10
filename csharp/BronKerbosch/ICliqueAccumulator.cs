using System.Collections.Immutable;

namespace BronKerbosch
{
    public interface ICliqueAccumulator<TAccumulator>
        where TAccumulator : ICliqueAccumulator<TAccumulator>
    {
        void Add(ImmutableArray<Vertex> clique);
        void Absorb(TAccumulator spawned);
    }
}
