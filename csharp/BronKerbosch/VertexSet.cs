using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    public interface IVertexSetMgr<TSet>
    {
        static abstract string Name();
        static abstract TSet Empty();
        static abstract TSet EmptyWithCapacity(int capacity);
        static abstract TSet From(IEnumerable<Vertex> vertices);
        static abstract bool Add(TSet s, Vertex v);
        static abstract bool Remove(TSet s, Vertex v);
        static abstract Vertex PopArbitrary(TSet s);
        static abstract TSet Difference(TSet s1, TSet s2);
        static abstract TSet Intersection(TSet s1, TSet s2);
        static abstract int IntersectionSize(TSet s1, TSet s2);
        static abstract bool Overlaps(TSet s1, TSet s2);
    }

    public record struct HashSetMgr : IVertexSetMgr<HashSet<Vertex>>
    {
        public static string Name() => "HashSet";
        public static HashSet<Vertex> Empty() => new();
        public static HashSet<Vertex> EmptyWithCapacity(int capacity) => new(capacity: capacity);
        public static HashSet<Vertex> From(IEnumerable<Vertex> vertices) => new(vertices);
        public static bool Add(HashSet<Vertex> s, Vertex v) => s.Add(v);
        public static bool Remove(HashSet<Vertex> s, Vertex v) => s.Remove(v);
        public static Vertex PopArbitrary(HashSet<Vertex> s)
        {
            using var en = s.GetEnumerator();
            var ok = en.MoveNext();
            Debug.Assert(ok);
            ok = s.Remove(en.Current);
            Debug.Assert(ok);
            return en.Current;
        }
        public static HashSet<Vertex> Difference(HashSet<Vertex> s1, HashSet<Vertex> s2)
        {
            var result = new HashSet<Vertex>(capacity: s1.Count);
            foreach (var v in s1)
            {
                if (!s2.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
            // much slower: s1.Except(s2).ToHashSet();
        }
        public static HashSet<Vertex> Intersection(HashSet<Vertex> s1, HashSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
            {
                return Intersection(s2, s1);
            }

            var result = new HashSet<Vertex>(capacity: Math.Min(s1.Count, s2.Count));
            foreach (var v in s1)
            {
                if (s2.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
            // much slower: return s2.Intersect(s1).ToHashSet();
            // even slower: return s1.Intersect(s2).ToHashSet();
        }
        public static int IntersectionSize(HashSet<Vertex> s1, HashSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
            {
                return IntersectionSize(s2, s1);
            }

            return s1.Count(s2.Contains);
            // wee bit slower: return s1.Where(v => s2.Contains(v)).Cardinality();
            // much slower: return s2.Intersect(s1).Cardinality();
            // even slower: return s1.Intersect(s2).Cardinality();
        }
        public static bool Overlaps(HashSet<Vertex> s1, HashSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
                return s1.Overlaps(s2);
            else
                return s2.Overlaps(s1);
        }
    }

    public record struct SortedSetMgr : IVertexSetMgr<SortedSet<Vertex>>
    {
        public static string Name() => "SortedSet";
        public static SortedSet<Vertex> Empty() => new();
        public static SortedSet<Vertex> EmptyWithCapacity(int capacity) => new();
        public static SortedSet<Vertex> From(IEnumerable<Vertex> vertices) => new(vertices);
        public static bool Add(SortedSet<Vertex> s, Vertex v) => s.Add(v);
        public static bool Remove(SortedSet<Vertex> s, Vertex v) => s.Remove(v);
        public static Vertex PopArbitrary(SortedSet<Vertex> s)
        {
            using var en = s.GetEnumerator();
            var ok = en.MoveNext();
            Debug.Assert(ok);
            ok = s.Remove(en.Current);
            Debug.Assert(ok);
            return en.Current;
        }
        public static SortedSet<Vertex> Difference(SortedSet<Vertex> s1, SortedSet<Vertex> s2)
        {
            var result = new SortedSet<Vertex>();
            foreach (var v in s1)
            {
                if (!s2.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
            // much slower: s1.Except(s2).ToHashSet();
        }
        public static SortedSet<Vertex> Intersection(SortedSet<Vertex> s1, SortedSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
            {
                return Intersection(s2, s1);
            }

            var result = new SortedSet<Vertex>();
            foreach (var v in s1)
            {
                if (s2.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
            // much slower: return s2.Intersect(s1).ToHashSet();
            // even slower: return s1.Intersect(s2).ToHashSet();
        }
        public static int IntersectionSize(SortedSet<Vertex> s1, SortedSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
            {
                return IntersectionSize(s2, s1);
            }

            return s1.Count(s2.Contains);
            // wee bit slower: return s1.Where(v => s2.Contains(v)).Cardinality();
            // much slower: return s2.Intersect(s1).Cardinality();
            // even slower: return s1.Intersect(s2).Cardinality();
        }
        public static bool Overlaps(SortedSet<Vertex> s1, SortedSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
                return s1.Overlaps(s2);
            else
                return s2.Overlaps(s1);
        }
    }
}
