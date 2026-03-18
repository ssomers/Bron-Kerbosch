using System.Collections.Generic;
using System.Diagnostics;

namespace BronKerbosch
{
    internal sealed class FortifiedCounter<T>
    {
        public int Count { get; private set; }

#if DEBUG
        private readonly HashSet<T> itsLeftToPick = [];
#endif

        public void Add(T element)
        {
            Count += 1;
#if DEBUG
            bool added = itsLeftToPick.Add(element);
            Debug.Assert(added);
            Debug.Assert(Count == itsLeftToPick.Count);
#endif
        }

        public void Remove(T element)
        {
            Count -= 1;
            Trace.Assert(Count >= 0);
#if DEBUG
            bool removed = itsLeftToPick.Remove(element);
            Debug.Assert(removed);
            Debug.Assert(Count == itsLeftToPick.Count);
#endif
        }
    }
}
