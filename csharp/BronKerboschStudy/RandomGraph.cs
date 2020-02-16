using BronKerbosch;
using System;
using System.IO;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace BronKerboschStudy
{
    public sealed class RandomUndirectedGraph
    {
        public static int ParsePositiveInt(string orderstr)
        {
            int order;
            if (orderstr.EndsWith("M"))
                order = Int32.Parse(orderstr.Substring(0, orderstr.Length - 1)) * 1_000_000;
            else if (orderstr.EndsWith("k"))
                order = Int32.Parse(orderstr.Substring(0, orderstr.Length - 1)) * 1_000;
            else
                order = Int32.Parse(orderstr);
            return order;
        }

        public static UndirectedGraph Read(string orderstr, int size)
        {
            int order = ParsePositiveInt(orderstr);
            long fully_meshed_size = (long)order * (order - 1) / 2;
            if (size > fully_meshed_size)
                throw new ArgumentException($"{order} nodes accommodate at most {fully_meshed_size} edges", "size");

            var path = @"..\random_edges_order_" + orderstr + ".txt";
            List<HashSet<Vertex>> adjacencies = Enumerable.Range(0, order).Select(_ => new HashSet<Vertex>()).ToList();
            var file = new StreamReader(path);
            int linenum = 0;
            string line;
            while (linenum < size && (line = file.ReadLine()) != null)
            {
                ++linenum;
                var fields = line.Split(' ');
                int v, w;
                if (!int.TryParse(fields[0], out v) ||
                    !int.TryParse(fields[1], out w))
                {
                    throw new ArgumentException($"File {path} line {linenum} contains bogus text {line}");
                }
                var added1 = adjacencies[v].Add(w);
                var added2 = adjacencies[w].Add(v);
                Debug.Assert(added1);
                Debug.Assert(added2);
            }
            file.Close();
            if (linenum < size) {
                throw new ArgumentException($"Exhausted generated list of {linenum} edges in {path}");
            }
            var g = new UndirectedGraph(adjacencies);
            Debug.Assert(g.Order == order);
            Debug.Assert(g.Size == size);
            return g;
        }
    }
}
