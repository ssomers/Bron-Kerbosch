using System.Diagnostics;

namespace BronKerbosch
{
    // Counts the coming and going of elements and, in debug builds only, checks their identity.
    public sealed class FortifiedCounter<T>
    {
        public int Count { get; private set; }

        private readonly DebugOnlyTracker<T> Tracker = new();

        public bool Contains(T element) => Tracker.Contains(element);

        public void Add(T element)
        {
            Count += 1;
            Tracker.Add(element);
            Debug.Assert(Count == Tracker.Count);
        }

        public void Remove(T element)
        {
            Count -= 1;
            Trace.Assert(Count >= 0);
            Tracker.Remove(element);
            Debug.Assert(Count == Tracker.Count);
        }
    }
}
