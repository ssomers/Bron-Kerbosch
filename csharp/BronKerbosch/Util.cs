using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

public class Util
{
    public static bool AreDisjoint(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        if (lhs.Count > rhs.Count)
            return !lhs.Overlaps(rhs);
        else
            return !rhs.Overlaps(lhs);
    }

    public static HashSet<Vertex> Difference(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        var result = new HashSet<Vertex>();
        foreach (Vertex v in lhs)
        {
            if (!rhs.Contains(v))
                result.Add(v);
        }
        return result;
        // much slower: lhs.Except(rhs).ToHashSet();
    }

    public static int IntersectCount(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        if (lhs.Count > rhs.Count)
            return IntersectCount(rhs, lhs);

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

    public static HashSet<Vertex> Intersect(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        if (lhs.Count > rhs.Count)
            return Intersect(rhs, lhs);

        var result = new HashSet<Vertex>();
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