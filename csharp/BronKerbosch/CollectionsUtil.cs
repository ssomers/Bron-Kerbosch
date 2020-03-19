using System;
using System.Collections.Generic;
using System.Collections.Immutable;
using System.Diagnostics;
using Vertex = System.UInt32;

public static class CollectionsUtil
{
    public static ImmutableArray<Vertex> Append(ImmutableArray<Vertex> head, Vertex tail)
    {
        var builder = ImmutableArray.CreateBuilder<Vertex>(head.Length + 1);
        builder.AddRange(head);
        builder.Add(tail);
        return builder.MoveToImmutable();
    }

    public static bool AreDisjoint(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        if (lhs is null || rhs is null)
            return true;
        else if (lhs.Count > rhs.Count)
            return !lhs.Overlaps(rhs);
        else
            return !rhs.Overlaps(lhs);
    }

    public static HashSet<Vertex> Difference(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        var result = new HashSet<Vertex>(capacity: lhs.Count);
        foreach (Vertex v in lhs)
        {
            if (!rhs.Contains(v))
                result.Add(v);
        }
        return result;
        // much slower: lhs.Except(rhs).ToHashSet();
    }

    public static int IntersectionSize(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        if (lhs.Count > rhs.Count)
            return IntersectionSize(rhs, lhs);

        var result = 0;
        foreach (Vertex v in lhs)
        {
            if (rhs.Contains(v))
                result += 1;
        }
        return result;
        // wee bit slower: return lhs.Where(v => rhs.Contains(v)).Count();
        // much slower: return rhs.Intersect(lhs).Count();
        // even slower: return lhs.Intersect(rhs).Count();
    }

    public static HashSet<Vertex> Intersection(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        if (lhs.Count > rhs.Count)
            return Intersection(rhs, lhs);

        var result = new HashSet<Vertex>(capacity: Math.Min(lhs.Count, rhs.Count));
        foreach (Vertex v in lhs)
        {
            if (rhs.Contains(v))
                result.Add(v);
        }
        return result;
        // much slower: return rhs.Intersect(lhs).ToHashSet();
        // even slower: return lhs.Intersect(rhs).ToHashSet();
    }

    public static Vertex GetArbitrary(ISet<Vertex> candidates)
    {
        var en = candidates.GetEnumerator();
        var ok = en.MoveNext();
        Debug.Assert(ok);
        return en.Current;
    }

    public static Vertex PopArbitrary(ISet<Vertex> candidates)
    {
        var en = candidates.GetEnumerator();
        var ok = en.MoveNext();
        Debug.Assert(ok);
        candidates.Remove(en.Current);
        return en.Current;
    }
}