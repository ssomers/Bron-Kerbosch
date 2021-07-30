using System;

namespace BronKerbosch
{
    using Index = Int32;

    public struct Vertex : IComparable<Vertex>, IEquatable<Vertex>
    {
        public readonly Index index;
        private Vertex(Index i) => index = i;
        public static Vertex nth(Index i) => new Vertex(i);
        public bool Equals(Vertex that) => this.index == that.index;
        public override bool Equals(object obj) => obj is Vertex that && this.Equals(that);
        public override int GetHashCode() => index.GetHashCode();
        public int CompareTo(Vertex that) => this.index.CompareTo(that.index);
        public static bool operator <(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) < 0;
        public static bool operator <=(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) <= 0;
        public static bool operator >(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) > 0;
        public static bool operator >=(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) >= 0;
        public static bool operator ==(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) == 0;
        public static bool operator !=(Vertex lhs, Vertex rhs) => lhs.CompareTo(rhs) != 0;
    }
}
