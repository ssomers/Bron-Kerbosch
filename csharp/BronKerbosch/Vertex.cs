using System;
#pragma warning disable CA1036 // should define operator(s) '<, <=, >, >=' since it implements IComparable

namespace BronKerbosch
{
    using Index = Int32;

    public readonly record struct Vertex : IComparable<Vertex>
    {
        public readonly Index index;
        private Vertex(Index i) => index = i;
        public static Vertex Nth(Index i) => new(i);
        public int CompareTo(Vertex other) => index.CompareTo(other.index);
    }
}
