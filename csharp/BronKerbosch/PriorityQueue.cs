using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

using Priority = int;

namespace BronKerbosch
{
    internal sealed class PriorityQueue<T>(int maxPriority)
    {
        private readonly List<T>[] itsQueuePerPriority = [.. Enumerable.Repeat(true, maxPriority).Select(_ => new List<T>())];

        // Putting the same element again does not replace the previous entry.
        public void Put(Priority priority, T element)
        {
            Debug.Assert(priority > 0);
            itsQueuePerPriority[priority - 1].Add(element);
        }

        // May pop an element already popped earlier, in case it was put multiple times.
        public T Pop()
        {
            foreach (List<T> stack in itsQueuePerPriority)
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
