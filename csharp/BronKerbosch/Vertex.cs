using System;
using Index = System.Int32;

#pragma warning disable CA1036 // should define operator(s) '<, <=, >, >=' since it implements IComparable

namespace BronKerbosch
{
    public readonly record struct Vertex : IComparable<Vertex>
    {
        private Index Idx { init; get; }

        public static Vertex Nth(Index idx) => new() { Idx = idx };
        public int CompareTo(Vertex other) => Idx.CompareTo(other.Idx);
        public Index Index() => Idx;
        public override string ToString() => $"vertex {Idx}";
    }
}
