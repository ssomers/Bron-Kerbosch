#if DEBUG
using System.Collections.Generic;
using System.Diagnostics;
#else
using System;
#endif

namespace BronKerbosch
{
    // Tracks the coming and going of elements in debug builds only,
    // geared towards FortifiedCounter<T>.
    public sealed class DebugOnlyTracker<T>
    {
#if DEBUG
        private readonly HashSet<T> itsElements = [];

        public bool Contains(T element) => itsElements.Contains(element);

        // Returns the number of elements added and not yet removed.
        public int Add(T element)
        {
            bool added = itsElements.Add(element);
            Debug.Assert(added);
            return itsElements.Count;
        }

        // Returns the number of elements added and not yet removed.
        public int Remove(T element)
        {
            bool removed = itsElements.Remove(element);
            Debug.Assert(removed);
            return itsElements.Count;
        }
#else
        public bool Contains(T _) => throw new NotImplementedException("Debug build only, please");
        public int Add(T _) => throw new NotImplementedException("Debug build only, please");
        public int Remove(T _) => throw new NotImplementedException("Debug build only, please");
#endif
    }
}
