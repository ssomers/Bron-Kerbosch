package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.BronKerbosch1;
import be.steinsomers.bron_kerbosch.BronKerbosch2;
import be.steinsomers.bron_kerbosch.BronKerbosch2_gp;
import be.steinsomers.bron_kerbosch.BronKerbosch2_gpx;
import be.steinsomers.bron_kerbosch.BronKerbosch3;
import be.steinsomers.bron_kerbosch.BronKerbosch3_MT;
import be.steinsomers.bron_kerbosch.BronKerbosch3_ST;
import be.steinsomers.bron_kerbosch.BronKerbosch3_gp;
import be.steinsomers.bron_kerbosch.BronKerbosch3_gpx;
import be.steinsomers.bron_kerbosch.BronKerboschAlgorithm;

import java.io.IOException;
import java.io.Writer;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

final class Main {
    static final String[] FUNC_NAMES = {
            "Ver1½",
            "Ver2½",
            "Ver2½-GP",
            "Ver2½-GPX",
            "Ver3½",
            "Ver3½-GP",
            "Ver3½-GPX",
            "Ver3½=GPc",
            "Ver3½=GPs",
    };
    static final BronKerboschAlgorithm[] FUNCS = {
            new BronKerbosch1(),
            new BronKerbosch2(),
            new BronKerbosch2_gp(),
            new BronKerbosch2_gpx(),
            new BronKerbosch3(),
            new BronKerbosch3_gp(),
            new BronKerbosch3_gpx(),
            new BronKerbosch3_MT(),
            new BronKerbosch3_ST(),
    };

    static List<List<Integer>> OrderCliques(Collection<int[]> cliques) {
        assert cliques.stream().allMatch(clique -> clique.length > 1);
        return cliques.stream()
                .map(clique -> Arrays.stream(clique)
                        .sorted()
                        .boxed()
                        .collect(Collectors.toUnmodifiableList()))
                .sorted((clique1, clique2) ->
                        IntStream.range(0, Math.min(clique1.size(), clique2.size()))
                                .map((int i) -> clique1.get(i) - clique2.get(i))
                                .filter((int diff) -> diff != 0)
                                .findFirst()
                                .orElseThrow(() -> new IllegalArgumentException(String.format(
                                        "got overlapping or equal cliques %s <> %s",
                                        clique1, clique2))))
                .collect(Collectors.toUnmodifiableList());
    }

    private static SampleStatistics[] bron_kerbosch_timed(RandomGraph graph,
                                                          int samples, int[] funcIndices)
            throws InterruptedException {
        Optional<List<List<Integer>>> firstOrdered = Optional.empty();
        var times = new SampleStatistics[FUNCS.length];
        IntStream.range(0, FUNCS.length).forEach(i -> times[i] = new SampleStatistics());
        for (int sample = samples == 1 ? 1 : 0; sample <= samples; ++sample) {
            for (int funcIndex : funcIndices) {
                if (sample == 0) {
                    var cliques = FUNCS[funcIndex].explore(graph)
                            .collect(Collectors.toUnmodifiableList());
                    var ordered = OrderCliques(cliques);
                    if (firstOrdered.isEmpty()) {
                        if (cliques.size() != graph.cliqueCount) {
                            throw new AssertionError("Inconsistent results");
                        }
                        firstOrdered = Optional.of(ordered);
                    } else {
                        if (!firstOrdered.get().equals(ordered)) {
                            throw new AssertionError("Inconsistent results");
                        }
                    }
                } else {
                    var start = System.nanoTime();
                    var cliqueCount = FUNCS[funcIndex].explore(graph).count();
                    var elapsed = System.nanoTime() - start;
                    if (cliqueCount != graph.cliqueCount) {
                        throw new AssertionError("Inconsistent results");
                    }
                    times[funcIndex].put(elapsed);
                }
            }
        }
        return times;
    }

    private static void bk(boolean genuine,
                           String orderStr,
                           int order,
                           int[] sizes,
                           int samples,
                           int[] funcIndices) {
        var name = "bron_kerbosch_java_order_" + (genuine ? orderStr : "warmup");
        var path = Paths.get("..", name + ".csv");
        try (Writer fo = Files.newBufferedWriter(path, StandardCharsets.UTF_8)) {
            fo.write("Size");
            for (var funcIndex : funcIndices) {
                var fn = FUNC_NAMES[funcIndex];
                fo.write(String.format(",%s min,%s mean,%s max", fn, fn, fn));
            }
            fo.write(System.lineSeparator());

            for (var size : sizes) {
                var start = System.nanoTime();
                var graph = RandomGraph.readUndirected(orderStr, order, size);
                var elapsed = System.nanoTime() - start;
                System.out.printf("%4s nodes, %7d edges, creation: %6.3f%n",
                        orderStr, size, elapsed / 1e9);
                var times = bron_kerbosch_timed(graph, samples, funcIndices);

                fo.write(String.format("%d", size));
                for (var funcIndex : funcIndices) {
                    var funcName = FUNC_NAMES[funcIndex];
                    double max = times[funcIndex].max() / 1e9;
                    double min = times[funcIndex].min() / 1e9;
                    double mean = times[funcIndex].mean() / 1e9;
                    double dev = times[funcIndex].deviation() / 1e9;
                    fo.write(String.format(",%f,%f,%f", min, mean, max));
                    System.out.printf("%4s nodes, %7d edges, %8s: %6.3f ± %.0f%%%n",
                            orderStr, size, funcName, mean, 100 * dev / mean);
                }
                fo.write(System.lineSeparator());
            }
        } catch (InterruptedException x) {
            System.err.format("InterruptedException: %s%n", x);
        } catch (IOException x) {
            System.err.format("IOException: %s%n", x);
        }
    }

    public static void main(String[] args) throws InterruptedException {
        assert false : "Omit -ea for meaningful measurements";

        int[] allFuncIndices = IntStream.range(0, FUNCS.length).toArray();
        int[] mostFuncIndices = IntStream.range(1, FUNCS.length).toArray();
        int[] sizes100 = IntStream.iterate(2_000, s -> s <= 3_000, s -> s + 50).toArray();
        int[] sizes10K = IntStream.iterate(1_000, s -> s <= 200_000,
                s -> s + (s < 10_000 ? 1_000 : s < 100_000 ? 10_000 : 25_000)).toArray();
        int[] sizes1M = IntStream.iterate(50_000, s -> s <= 5_000_000,
                s -> s + (s < 250_000 ? 50_000 : s < 2_000_000 ? 250_000 : 1_000_000)).toArray();

        bk(false, "100", 100, new int[]{2000}, 3, allFuncIndices); // warm up
        Thread.sleep(3210); // give IntelliJ launcher some time to cool down
        bk(true, "100", 100, sizes100, 5, allFuncIndices);
        bk(true, "10k", 10_000, sizes10K, 3, mostFuncIndices);
        bk(true, "1M", 1_000_000, sizes1M, 3, new int[]{2, 5, 7, 8});
        /*
        int[] sizesT = {500_000};
        int[] funcIndices = {8};
        bk("tt", 1_000_000, sizesT, 3, funcIndices);
        */
    }
}
