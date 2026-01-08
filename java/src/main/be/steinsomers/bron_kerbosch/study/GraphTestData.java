package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.UndirectedGraph;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.List;
import java.util.Set;
import java.util.stream.Stream;

record GraphTestData(UndirectedGraph graph, int cliqueCount) {
    private static List<Set<Integer>> new_sets(final int n) {
        return Stream
                .generate(() -> (Set<Integer>) new HashSet<Integer>(16))
                .limit(n)
                .toList();
    }

    static GraphTestData readUndirected(final String orderStr, final int order, final int size) throws IOException {
        assert order > 2;
        assert size >= 0;
        final var fullyMeshedSize = ((long) order) * (order - 1) / 2;
        if (size > fullyMeshedSize) {
            throw new IllegalArgumentException(
                    "%d nodes accommodate at most %d edges".formatted(order, fullyMeshedSize));
        }

        final var edgesPath = Paths.get("..", "data", "random_edges_order_" + orderStr + ".txt");
        final var statsPath = Paths.get("..", "data", "random_stats.txt");
        final var adjacencies = readEdges(edgesPath, order, size);
        final var cliqueCount = readStats(statsPath, orderStr, size);

        final var g = new UndirectedGraph(adjacencies);
        if (g.order() != order) throw new AssertionError("order mishap");
        if (g.size() != size) throw new AssertionError("size mishap");
        return new GraphTestData(g, cliqueCount);
    }

    private static List<Set<Integer>> readEdges(final Path path, final int order, final int size) throws IOException {
        final var adjacencies = new_sets(order);
        try (final var br = Files.newBufferedReader(path)) {
            for (int lineNum = 0; lineNum < size; ++lineNum) {
                final var line = br.readLine();
                if (line == null) {
                    throw new IOException("File %s has only %d of the requested %d lines"
                            .formatted(path, lineNum, size));
                }
                final var fields = line.split(" ", 2);
                final int v;
                final int w;
                //noinspection ProhibitedExceptionCaught
                try {
                    v = Integer.parseInt(fields[0]);
                    w = Integer.parseInt(fields[1]);
                } catch (final NumberFormatException | ArrayIndexOutOfBoundsException err) {
                    //noinspection ThrowInsideCatchBlockWhichIgnoresCaughtException
                    throw new IOException("File %s contains bogus text %s".formatted(path, line));
                }
                adjacencies.get(v).add(w);
                adjacencies.get(w).add(v);
            }
        }
        return adjacencies;
    }

    private static int readStats(final Path path, final String orderStr, final int size) throws IOException {
        final var prefix = String.format("%s\t%d\t", orderStr, size);
        try (final var br = Files.newBufferedReader(path)) {
            String line;
            while ((line = br.readLine()) != null) {
                if (line.startsWith(prefix)) {
                    try {
                        return Integer.parseInt(line.substring(prefix.length()));
                    } catch (final NumberFormatException err) {
                        //noinspection ThrowInsideCatchBlockWhichIgnoresCaughtException
                        throw new IOException("File %s contains bogus line %s".formatted(path, line));
                    }
                }
            }
            throw new IOException("File %s lacks order %s size %d".formatted(path, orderStr, size));
        }
    }
}
