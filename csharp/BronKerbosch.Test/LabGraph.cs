using System.Collections.Generic;
using System.Collections.Immutable;
using System.Linq;

namespace BronKerbosch.Test
{
    public class LabGraph<TVertexSet, TVertexSetMgr>
        where TVertexSet : ISet<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        public UndirectedGraph<TVertexSet, TVertexSetMgr> Graph { get; init; }

        internal LabGraph(int[][] adjacencies)
        {
            var adjacencies2 = adjacencies.Select(neighbours => TVertexSetMgr.From(neighbours.Select(i => Vertex.Nth(i))))
                                 .ToImmutableArray();
            Graph = new UndirectedGraph<TVertexSet, TVertexSetMgr>(adjacencies2);
        }

    }

    // Separate class because we want to avoid specifying the template arguments everywhere by inheriting,
    // but we don't want to inherit the LabGraph constructor.
    public class LabGraphs<TVertexSet, TVertexSetMgr>
        where TVertexSet : ISet<Vertex>
        where TVertexSetMgr : IVertexSetMgr<TVertexSet>
    {
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order0 = new([]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order1 = new([[]]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order2_isolated = new([[], []]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order2_connected = new([[1], [0]]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order3_size1_left = new([[1], [0], []]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order3_size1_long = new([[2], [], [0]]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order3_size1_right = new([[], [2], [1]]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order3_size2 = new([[1], [0, 2], [1]]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order3_size3 = new([[1, 2], [0, 2], [0, 1]]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order4_size2 = new([[1], [0], [3], [2]]);
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order4_size3_bus = new([[1], [0, 2], [1, 3], [2]]);

        // 0 - 1   2   3
        // |\_____/    |
        // |___________|
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order4_size3_star = new([[1, 2, 3], [0], [0], [0]]);

        // 0 - 1 - 2
        //      \ /
        //       3
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order4_size4_p = new([[1], [0, 2, 3], [1, 3], [1, 2]]);

        // 0 - 1
        // |   |
        // 3 - 2
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order4_size4_square = new([[1, 3], [0, 2], [1, 3], [0, 2]]);

        // 0 - 1 - 2 - 3
        // |\_____/    |
        // |___________|
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order4_size5 = new([[1, 2, 3], [0, 2], [0, 1, 3], [0, 2]]);

        //      _______
        //     |       |
        // 0 - 1 - 2 - 3
        // |\_____/    |
        // |___________|
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order4_size6 =
            new([[1, 2, 3], [0, 2, 3], [0, 1, 3], [0, 1, 2]]);

        //  _______________
        // |    ________   |
        // |   | _____   \ |
        // |   |/     \   \|
        // 0 - 1 - 2 - 3   4
        // |\_____/ \__|__/
        // |___________|
        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Order5_size6_penultimate =
            new([[1, 2, 3, 4], [0, 2, 3, 4], [0, 1, 3, 4], [0, 1, 2], [0, 1, 2]]);

        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Sample =
            new([[],
                 [ 2, 3, 4 ],
                 [1, 3, 4, 5 ],
                 [1, 2, 4, 5 ],
                 [1, 2, 3 ],
                 [2, 3, 6, 7 ],
                 [5, 7 ],
                 [5, 6 ] ]);

        public static readonly LabGraph<TVertexSet, TVertexSetMgr> Bigger =
            new([[1, 2, 3, 4, 6, 7 ],
                 [0, 3, 6, 7, 8, 9 ],
                 [0, 3, 5, 7, 8, 9 ],
                 [0, 1, 2, 4, 9 ],
                 [0, 3, 6, 7, 9 ],
                 [2, 6 ],
                 [0, 1, 4, 5, 9 ],
                 [0, 1, 2, 4, 9 ],
                 [1, 2 ],
                 [1, 2, 3, 4, 6, 7 ] ]);
    }
}
