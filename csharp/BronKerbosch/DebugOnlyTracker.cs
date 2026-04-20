#if DEBUG
using System.Collections.Generic;
using System.Diagnostics;
#else
#   pragma warning disable CA1822 // Mark members as static
#endif

namespace BronKerbosch
{
    // Tracks the coming and going of elements in debug builds only.
    public sealed class DebugOnlyTracker<T>
    {
#if DEBUG
        private readonly HashSet<T> itsLeftToPick = [];
#endif

#if DEBUG
        public int Count => itsLeftToPick.Count;
#else
        public int Count => throw new System.NotImplementedException("Debug build only, please");
#endif


#if DEBUG
        public bool Contains(T element) => itsLeftToPick.Contains(element);
#else
        public bool Contains(T element) => throw new System.NotImplementedException("Debug build only, please");
#endif

        public void Add(T element)
        {
#if DEBUG
            bool added = itsLeftToPick.Add(element);
            Debug.Assert(added);
#endif
        }

        public void Remove(T element)
        {
#if DEBUG
            bool removed = itsLeftToPick.Remove(element);
            Debug.Assert(removed);
#endif
        }
    }
}
