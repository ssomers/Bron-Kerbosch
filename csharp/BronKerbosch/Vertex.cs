using System;
#pragma warning disable CA1036 // should define operator(s) '<, <=, >, >=' since it implements IComparable

namespace BronKerbosch
{
    using Index = Int32;

    public readonly record struct Vertex : IComparable<Vertex>
    {
        private Index Idx { init; get; }

        public static Vertex Nth(Index idx) => new() { Idx = idx };
        public int CompareTo(Vertex other) => Idx.CompareTo(other.Idx);
        public Index Index() => Idx;
    }
}
