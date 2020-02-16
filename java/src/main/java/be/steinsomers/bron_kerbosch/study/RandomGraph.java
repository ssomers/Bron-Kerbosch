package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.UndirectedGraph;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

final class RandomGraph {
    private static List<Set<Integer>> new_sets(int n) {
        return Stream.generate(() -> new HashSet<Integer>()).limit(n).collect(Collectors.toList());
    }

    UndirectedGraph readUndirected(String orderstr, int order, int size) throws IOException {
        assert order > 2;
        assert size >= 0;
        var fullyMeshedSize = ((long) order) * (order - 1) / 2;
        if (size > fullyMeshedSize) {
            throw new IllegalArgumentException(String.format(
                    "%d nodes accommodate at most %d edges", order, fullyMeshedSize));
        }

        var adjacencies = new_sets(order);
        var path = Paths.get("..").resolve("random_edges_order_" + orderstr + ".txt");
        try (var br = Files.newBufferedReader(path)) {
            String line;
            int linenum = 0;
            while (linenum < size && (line = br.readLine()) != null) {
                ++linenum;
                var fields = line.split(" ", 2);
                int v;
                int w;
                try {
                    v = Integer.parseInt(fields[0]);
                    w = Integer.parseInt(fields[1]);
                } catch (NumberFormatException err) {
                    throw new IOException("Garbage at line " + linenum + " in file " + path);
                }
                adjacencies.get(v).add(w);
                adjacencies.get(w).add(v);
            }
            if (linenum < size) {
                throw new IOException("Exhausted generated list of " + linenum + " edges in " + path);
            }
        }

        var g = new UndirectedGraph(adjacencies);
        if (g.order() != order) throw new AssertionError("order mishap");
        if (g.size() != size) throw new AssertionError("size mishap");
        return g;
    }
}
