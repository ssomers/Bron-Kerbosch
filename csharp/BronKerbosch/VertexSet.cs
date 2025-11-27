using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace BronKerbosch
{
    public static class VertexSetExtensions
    {
        public static Vertex PopArbitrary<TVertexSet>(this TVertexSet set) where TVertexSet : ISet<Vertex>
        {
            using var en = set.GetEnumerator();
            bool ok = en.MoveNext();
            if (!ok)
                throw new ArgumentException("Attempt to pop from empty set");
            Vertex result = en.Current;
            ok = set.Remove(result);
            if (!ok)
                throw new ArgumentException("Attempt to pop from invalid set");
            return result;
        }
    }

    public interface IVertexSetMgr<TVertexSet> where TVertexSet : ISet<Vertex>
    {
        static abstract string Name();
        static abstract TVertexSet Empty();
        static abstract TVertexSet EmptyWithCapacity(int capacity);
        static abstract TVertexSet From(IEnumerable<Vertex> vertices);
        static abstract bool Add(TVertexSet s, Vertex v);
        static abstract bool Remove(TVertexSet s, Vertex v);
        static abstract TVertexSet Difference(TVertexSet s1, TVertexSet s2);
        static abstract TVertexSet Intersection(TVertexSet s1, TVertexSet s2);
        static abstract int IntersectionSize(TVertexSet s1, TVertexSet s2);
        static abstract bool Overlaps(TVertexSet s1, TVertexSet s2);
    }

    public record struct HashSetMgr : IVertexSetMgr<HashSet<Vertex>>
    {
        public static string Name() => "HashSet";
        public static HashSet<Vertex> Empty() => [];
        public static HashSet<Vertex> EmptyWithCapacity(int capacity) => new(capacity: capacity);
        public static HashSet<Vertex> From(IEnumerable<Vertex> vertices) => [.. vertices];
        public static bool Add(HashSet<Vertex> s, Vertex v) => s.Add(v);
        public static bool Remove(HashSet<Vertex> s, Vertex v) => s.Remove(v);

        public static HashSet<Vertex> Difference(HashSet<Vertex> s1, HashSet<Vertex> s2)
        {
            HashSet<Vertex> result = new(capacity: s1.Count);
            foreach (Vertex v in s1)
            {
                if (!s2.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
            // much slower: [.. s1.Except(s2)]
        }

        public static HashSet<Vertex> Intersection(HashSet<Vertex> s1, HashSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
                (s1, s2) = (s2, s1);

            HashSet<Vertex> result = new(capacity: s1.Count);
            foreach (var v in s1)
            {
                if (s2.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
            // much slower: [.. s2.Intersect(s1)]
            // even slower: [.. s1.Intersect(s2)]
        }

        public static int IntersectionSize(HashSet<Vertex> s1, HashSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
                return s2.Count(s1.Contains);
            else
                return s1.Count(s2.Contains);
            // wee bit slower: s1.Where(v => s2.Contains(v)).Count()
            // much slower (Ver2-GP in particular): s2.Intersect(s1).Count()
            // even slower: s1.Intersect(s2).Count()
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
        public static SortedSet<Vertex> Empty() => [];
        public static SortedSet<Vertex> EmptyWithCapacity(int capacity) => [];
        public static SortedSet<Vertex> From(IEnumerable<Vertex> vertices) => [.. vertices];
        public static bool Add(SortedSet<Vertex> s, Vertex v) => s.Add(v);
        public static bool Remove(SortedSet<Vertex> s, Vertex v) => s.Remove(v);

        public static SortedSet<Vertex> Difference(SortedSet<Vertex> s1, SortedSet<Vertex> s2)
        {
            SortedSet<Vertex> result = [];
            foreach (Vertex v in s1)
            {
                if (!s2.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
        }

        public static SortedSet<Vertex> Intersection(SortedSet<Vertex> s1, SortedSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
                (s1, s2) = (s2, s1);

            SortedSet<Vertex> result = [];
            foreach (Vertex v in s1)
            {
                if (s2.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
            // much slower: [.. s2.Intersect(s1)]
            // even slower: [.. s1.Intersect(s2)]
        }

        public static int IntersectionSize(SortedSet<Vertex> s1, SortedSet<Vertex> s2)
        {
            if (s1.Count > s2.Count)
                return s2.Count(s1.Contains);
            else
                return s1.Count(s2.Contains);
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
