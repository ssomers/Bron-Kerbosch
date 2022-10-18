using System;

namespace BronKerbosch
{
    using Index = Int32;

    public readonly struct Vertex : IComparable<Vertex>, IEquatable<Vertex>
    {
        public readonly Index index;
        private Vertex(Index i) => index = i;
        public static Vertex Nth(Index i) => new(i);
        public bool Equals(Vertex other) => index == other.index;
        public override bool Equals(object? obj) => obj is Vertex other && Equals(other);
        public override int GetHashCode() => index.GetHashCode();
        public int CompareTo(Vertex other) => index.CompareTo(other.index);
        public static bool operator <(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) < 0;
        public static bool operator <=(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) <= 0;
        public static bool operator >(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) > 0;
        public static bool operator >=(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) >= 0;
        public static bool operator ==(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) == 0;
        public static bool operator !=(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) != 0;
    }
}
