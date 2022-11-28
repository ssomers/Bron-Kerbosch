using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using System.Linq;

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

        public static bool Overlaps<T>(ISet<T> lhs, ISet<T> rhs)
        {
            if (lhs is null || rhs is null)
                return false;
            else if (lhs.Count > rhs.Count)
                return lhs.Overlaps(rhs);
            else
                return rhs.Overlaps(lhs);
        }

        public static HashSet<T> Difference<T>(ISet<T> lhs, ISet<T> rhs)
        {
            var result = new HashSet<T>(capacity: lhs.Count);
            foreach (var v in lhs)
            {
                if (!rhs.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
            // much slower: lhs.Except(rhs).ToHashSet();
        }

        public static int IntersectionSize<T>(ISet<T> lhs, ISet<T> rhs)
        {
            if (lhs.Count > rhs.Count)
            {
                return IntersectionSize(rhs, lhs);
            }

            return lhs.Count(rhs.Contains);
            // wee bit slower: return lhs.Where(v => rhs.Contains(v)).Count();
            // much slower: return rhs.Intersect(lhs).Count();
            // even slower: return lhs.Intersect(rhs).Count();
        }

        public static HashSet<T> Intersection<T>(ISet<T> lhs, ISet<T> rhs)
        {
            if (lhs.Count > rhs.Count)
            {
                return Intersection(rhs, lhs);
            }

            var result = new HashSet<T>(capacity: Math.Min(lhs.Count, rhs.Count));
            foreach (var v in lhs)
            {
                if (rhs.Contains(v))
                {
                    var added = result.Add(v);
                    Debug.Assert(added);
                }
            }
            return result;
            // much slower: return rhs.Intersect(lhs).ToHashSet();
            // even slower: return lhs.Intersect(rhs).ToHashSet();
        }

        public static T GetArbitrary<T>(ISet<T> candidates)
        {
            using var en = candidates.GetEnumerator();
            var ok = en.MoveNext();
            Debug.Assert(ok);
            return en.Current;
        }

        public static T PopArbitrary<T>(ISet<T> candidates)
        {
            using var en = candidates.GetEnumerator();
            var ok = en.MoveNext();
            Debug.Assert(ok);
            ok = candidates.Remove(en.Current);
            Debug.Assert(ok);
            return en.Current;
        }
    }
}
