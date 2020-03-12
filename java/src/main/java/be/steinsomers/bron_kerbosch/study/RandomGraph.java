package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.UndirectedGraph;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

final class RandomGraph extends UndirectedGraph {
    public final int cliqueCount;

    private RandomGraph(List<Set<Integer>> adjacencies, int cliqueCount) {
        super(adjacencies);
        this.cliqueCount = cliqueCount;
    }

    private static List<Set<Integer>> new_sets(int n) {
        return Stream
                .generate(() -> new HashSet<Integer>())
                .limit(n)
                .collect(Collectors.toUnmodifiableList());
    }

    static RandomGraph readUndirected(String orderStr, int order, int size) throws IOException {
        assert order > 2;
        assert size >= 0;
        var fullyMeshedSize = ((long) order) * (order - 1) / 2;
        if (size > fullyMeshedSize) {
            throw new IllegalArgumentException(String.format(
                    "%d nodes accommodate at most %d edges", order, fullyMeshedSize));
        }

        var edgesPath = Paths.get("..").resolve("random_edges_order_" + orderStr + ".txt");
        var statsPath = Paths.get("..").resolve("random_stats.txt");
        var adjacencies = readEdges(edgesPath, order, size);
        var cliqueCount = readStats(statsPath, orderStr, size);

        var g = new RandomGraph(adjacencies, cliqueCount);
        if (g.order() != order) throw new AssertionError("order mishap");
        if (g.size() != size) throw new AssertionError("size mishap");
        return g;
    }

    private static List<Set<Integer>> readEdges(Path path, int order, int size) throws IOException {
        var adjacencies = new_sets(order);
        try (var br = Files.newBufferedReader(path)) {
            String line;
            int lineNum = 0;
            while (lineNum < size && (line = br.readLine()) != null) {
                ++lineNum;
                var fields = line.split(" ", 2);
                int v;
                int w;
                //noinspection ProhibitedExceptionCaught
                try {
                    v = Integer.parseInt(fields[0]);
                    w = Integer.parseInt(fields[1]);
                } catch (NumberFormatException | ArrayIndexOutOfBoundsException err) {
                    //noinspection ThrowInsideCatchBlockWhichIgnoresCaughtException
                    throw new IOException("File " + path + " contains bogus text " + line);
                }
                adjacencies.get(v).add(w);
                adjacencies.get(w).add(v);
            }
            if (lineNum < size) {
                throw new IOException("Exhausted list of " + lineNum + " edges in " + path);
            }
        }
        return adjacencies;
    }

    private static int readStats(Path path, String orderStr, int size) throws IOException {
        var prefix = String.format("%s\t%d\t", orderStr, size);
        try (var br = Files.newBufferedReader(path)) {
            String line;
            while ((line = br.readLine()) != null) {
                if (line.startsWith(prefix)) {
                    int c;
                    try {
                        c = Integer.parseInt(line.substring(prefix.length()));
                    } catch (NumberFormatException err) {
                        //noinspection ThrowInsideCatchBlockWhichIgnoresCaughtException
                        throw new IOException("File " + path + " has bogus line “" + line + "”");
                    }
                    return c;
                }
            }
            throw new IOException("File " + path + " lacks order " + orderStr + " size " + size);
        }
    }
}
