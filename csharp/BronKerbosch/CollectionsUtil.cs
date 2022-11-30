using System.Collections.Immutable;

namespace BronKerbosch
{
    public static class CollectionsUtil
    {
        public static ImmutableArray<T> Append<T>(ImmutableArray<T> head, T tail)
        {
            var builder = ImmutableArray.CreateBuilder<T>(head.Length + 1);
            builder.AddRange(head);
            builder.Add(tail);
            return builder.MoveToImmutable();
        }
    }
}
