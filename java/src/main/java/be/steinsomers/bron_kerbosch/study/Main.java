package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.*;

import java.io.IOException;
import java.io.Writer;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Collection;
import java.util.List;
import java.util.Optional;
import java.util.Random;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

final class Main {
    static final String[] FUNC_NAMES = {
            "Ver1+",
            "Ver2+",
            "Ver2+G",
            "Ver2+GP",
            "Ver2+GPX",
            "Ver3+",
            "Ver3+GP",
            "Ver3+GPX",
            "Ver3+MT",
    };
    static final BronKerboschAlgorithm[] FUNCS = {
            new BronKerbosch1(),
            new BronKerbosch2(),
            new BronKerbosch2_g(),
            new BronKerbosch2_gp(),
            new BronKerbosch2_gpx(),
            new BronKerbosch3(),
            new BronKerbosch3_gp(),
            new BronKerbosch3_gpx(),
            new BronKerbosch3_MT(),
    };

    static List<List<Integer>> OrderCliques(Collection<? extends Collection<Integer>> cliques) {
        assert cliques.stream().allMatch(clique -> clique.size() > 1);
        return cliques.stream()
                .map(clique -> clique.stream()
                        .sorted()
                        .collect(Collectors.toList()))
                .sorted((clique1, clique2) ->
                        IntStream.range(0, Math.min(clique1.size(), clique2.size()))
                                .map((int i) -> clique1.get(i) - clique2.get(i))
                                .filter((int diff) -> diff != 0)
                                .findFirst()
                                .orElseThrow(() -> new IllegalArgumentException(String.format(
                                        "got overlapping or equal cliques %s <> %s",
                                        clique1, clique2))))
                .collect(Collectors.toList());
    }

    private static SampleStatistics[] bron_kerbosch_timed(UndirectedGraph graph,
                                                          int samples, int[] funcIndices)
            throws InterruptedException {
        Optional<List<List<Integer>>> firstOrdered = Optional.empty();
        SampleStatistics[] times = new SampleStatistics[FUNCS.length];
        IntStream.range(0, FUNCS.length).forEach(i -> times[i] = new SampleStatistics());
        for (int sample = 1; sample <= samples; ++sample) {
            for (int funcIndex : funcIndices) {
                var reporter = new SimpleReporter();
                var start = System.currentTimeMillis();
                FUNCS[funcIndex].explore(graph, reporter);
                var elapsed = System.currentTimeMillis() - start;
                times[funcIndex].put(elapsed);

                if (samples > 1 && sample <= 2) {
                    var ordered = OrderCliques(reporter.cliques);
                    if (firstOrdered.isEmpty()) {
                        firstOrdered = Optional.of(ordered);
                    } else {
                        if (!firstOrdered.get().equals(ordered)) {
                            throw new AssertionError("Inconsistent results");
                        }
                    }
                }
            }
        }
        return times;
    }

    private static void bk(String orderStr,
                           int order,
                           int[] sizes,
                           int samples,
                           int[] funcIndices) {
        var name = "bron_kerbosch_java_order_" + orderStr;
        var path = Paths.get("..").resolve(name + ".csv");
        try (Writer fo = Files.newBufferedWriter(path, StandardCharsets.US_ASCII)) {
            fo.write("Size");
            for (var funcIndex : funcIndices) {
                String fn = FUNC_NAMES[funcIndex];
                fo.write(String.format(",%s min,%s mean,%s max", fn, fn, fn));
            }
            fo.write(System.lineSeparator());

            for (var size : sizes) {
                var start = System.currentTimeMillis();
                var rng = new Random(19680516L);
                var graph = new RandomGraphGenerator(rng).newUndirected(order, size);
                var elapsed = System.currentTimeMillis() - start;
                System.out.printf("%7s nodes, %7d edges, creation: %5.2f%n",
                        orderStr, size, elapsed / 1e3);
                var times = bron_kerbosch_timed(graph, samples, funcIndices);

                fo.write(String.format("%d", size));
                for (var funcIndex : funcIndices) {
                    String funcName = FUNC_NAMES[funcIndex];
                    double max = times[funcIndex].max() / 1e3;
                    double min = times[funcIndex].min() / 1e3;
                    double mean = times[funcIndex].mean() / 1e3;
                    double dev = times[funcIndex].deviation() / 1e3;
                    fo.write(String.format(",%f,%f,%f", min, mean, max));
                    System.out.printf("%7s nodes, %7d edges, %8s: %5.2f ±%5.2f%n",
                            orderStr, size, funcName, mean, dev);
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
        int[] sizesWarm = {2000};
        int[] sizes100 = IntStream.iterate(2_000, s -> s <= 3_000, s -> s + 50).toArray();
        int[] sizes10K = IntStream.iterate(100_000, s -> s <= 800_000, s -> s + 100_000).toArray();
        int[] sizes1M = IntStream.iterate(200_000, s -> s <= 5_000_000,
                s -> s + (s < 2_000_000 ? 200_000 : 1_000_000)).toArray();
        bk("warm-up", 100, sizesWarm, 3, allFuncIndices);
        Thread.sleep(3210); // give IntelliJ launcher some time to cool down
        bk("100", 100, sizes100, 5, allFuncIndices);
        bk("10k", 10_000, sizes10K, 3, mostFuncIndices);
        bk("1M", 1_000_000, sizes1M, 3, mostFuncIndices);
        /*
        int[] sizesT = {500_000};
        int[] funcIndices = {8};
        bk("tt", 1_000_000, sizesT, 3, funcIndices);
        */
    }
}