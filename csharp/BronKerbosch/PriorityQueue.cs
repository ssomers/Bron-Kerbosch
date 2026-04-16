using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

using Priority = int;

namespace BronKerbosch
{
    internal sealed class PriorityQueue<T>(int maxPriority)
    {
        private readonly List<T>[] itsStackPerPriority = [.. Enumerable.Repeat(true, maxPriority).Select(_ => new List<T>())];

        public bool Contains(Priority priority, T element)
        {
            Debug.Assert(priority > 0);
#if DEBUG
            return itsStackPerPriority[priority - 1].Contains(element);
#else
            _ = itsStackPerPriority;
            throw new NotImplementedException("Debug build only, please");
#endif
        }

        // Putting the same element again does not replace the previous entry.
        public void Put(Priority priority, T element)
        {
            Debug.Assert(priority > 0);
            itsStackPerPriority[priority - 1].Add(element);
        }

        // May pop an element already popped earlier, in case it was put multiple times.
        public T Pop()
        {
            foreach (List<T> stack in itsStackPerPriority)
            {
                if (stack.Count > 0)
                {
                    var last = stack.Count - 1;
                    T element = stack[last];
                    stack.RemoveAt(last);
                    return element;
                }
            }
            throw new ArgumentException("Cannot pop more than has been put");
        }
    }
}
