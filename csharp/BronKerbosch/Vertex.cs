using System;
#pragma warning disable CA1036 // should define operator(s) '<, <=, >, >=' since it implements IComparable

namespace BronKerbosch
{
    using Index = Int32;

    public readonly record struct Vertex : IComparable<Vertex>
    {
        private Index index { init; get; }

        public static Vertex Nth(Index i) => new() { index = i };
        public int CompareTo(Vertex other) => index.CompareTo(other.index);
        public Index Index() => index;
    }
}
