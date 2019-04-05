// Bron-Kerbosch algorithm with pivot picked arbitrarily

using BronKerbosch;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

public class Util
{
    public static bool AreDisjoint(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        if (lhs.Count > rhs.Count)
            return AreDisjoint(rhs, lhs);

        return !rhs.Overlaps(lhs);
    }

    public static HashSet<Vertex> Intersect(ISet<Vertex> lhs, ISet<Vertex> rhs)
    {
        if (lhs.Count > rhs.Count)
            return Intersect(rhs, lhs);

        var result = new HashSet<Vertex>(lhs);
        result.IntersectWith(rhs);
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