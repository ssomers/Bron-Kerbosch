package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.*;

import java.io.IOException;
import java.io.Writer;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.Optional;
import java.util.Random;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

class Main {
    final static String[] FUNC_NAMES = {
            "Ver1+",
            "Ver2+",
            "Ver2+G",
            "Ver2+GP",
            "Ver2+GPX",
    };
    final static BronKerboschAlgorithm[] FUNCS = {
            new BronKerbosch1(),
            new BronKerbosch2(),
            new BronKerbosch2_g(),
            new BronKerbosch2_gp(),
            new BronKerbosch2_gpx(),
    };

    public static List<List<Integer>> OrderCliques(List<List<Integer>> cliques) {
        assert cliques.stream().allMatch((List<Integer> clique) -> clique.size() > 1);
        return cliques.stream()
                .map((List<Integer> clique) -> clique.stream()
                        .sorted()
                        .collect(Collectors.toList()))
                .sorted((List<Integer> clique1, List<Integer> clique2) ->
                        IntStream.range(0, Math.min(clique1.size(), clique2.size()))
                                .map((int i) -> clique1.get(i) - clique2.get(i))
                                .filter((int diff) -> diff != 0)
                                .findFirst()
                                .orElseThrow(() -> new IllegalArgumentException(String.format(
                                        "got overlapping or equal cliques %s <> %s", clique1, clique2
                                ))))
                .collect(Collectors.toList());
    }

    private static SampleStatistics[] bron_kerbosch_timed(UndirectedGraph graph,
                                                          int samples,
                                                          int[] func_indices) {
        Optional<List<List<Integer>>> firstOrdered = Optional.empty();
        SampleStatistics[] times = new SampleStatistics[FUNCS.length];
        IntStream.range(0, FUNCS.length).forEach(i -> times[i] = new SampleStatistics());
        times[0].mean();

        for (int sample = 1; sample <= samples; ++sample) {
            for (int func_index : func_indices) {
                var reporter = new SimpleReporter();
                var start = System.currentTimeMillis();
                FUNCS[func_index].explore(graph, reporter);
                var elapsed = System.currentTimeMillis() - start;
                times[func_index].put(elapsed);

                if (samples > 1 && sample <= 2) {
                    var ordered = OrderCliques(reporter.cliques);
                    if (firstOrdered.isEmpty()) {
                        firstOrdered = Optional.of(ordered);
                    } else if (!firstOrdered.get().equals(ordered)) {
                        throw new RuntimeException("Inconsistent results");
                    }
                }
            }
        }
        return times;
    }

    private static void bk(String order_str,
                           int order,
                           int[] sizes,
                           int samples,
                           int[] func_indices) {
        var name = "bron_kerbosch_java_order_" + order_str;
        var path = Paths.get("..").resolve(name + ".csv");
        try (Writer fo = Files.newBufferedWriter(path, StandardCharsets.US_ASCII)) {
            fo.write("Size");
            for (var func_index : func_indices) {
                String fn = FUNC_NAMES[func_index];
                fo.write(String.format(",%s min,%s mean,%s max", fn, fn, fn));
            }
            fo.write('\n');

            for (var size : sizes) {
                var start = System.currentTimeMillis();
                var rng = new Random(19680516L);
                var graph = new RandomGraphGenerator(rng).new_undirected(order, size);
                var elapsed = System.currentTimeMillis() - start;
                System.out.printf("order %7s size %7d creation: %5.2f\n", order_str, size, elapsed / 1e3);
                var times = bron_kerbosch_timed(graph, samples, func_indices);

                fo.write(String.format("%d", size));
                for (var func_index : func_indices) {
                    String func_name = FUNC_NAMES[func_index];
                    double max = times[func_index].max() / 1e3;
                    double min = times[func_index].min() / 1e3;
                    double mean = times[func_index].mean() / 1e3;
                    double dev = times[func_index].deviation() / 1e3;
                    fo.write(String.format(",%f,%f,%f", min, mean, max));
                    System.out.printf("order %7s size %7d %8s: %5.2f ±%5.2f\n",
                            order_str, size, func_name, mean, dev);
                }
                fo.write('\n');
            }
        } catch (IOException x) {
            System.err.format("IOException: %s%n", x);
        }
    }

    public static void main(String[] args) throws InterruptedException {
        assert false : "Omit -ea for meaningful measurements";

        int[] all_func_indices = IntStream.range(0, FUNCS.length).toArray();
        int[] most_func_indices = IntStream.range(1, FUNCS.length).toArray();
        int[] sizes_warm = {2000};
        int[] sizes_100 = IntStream.rangeClosed(2_000, 3_000).filter(i -> i % 50 == 0).toArray();
        int[] sizes_10k = IntStream.rangeClosed(100_000, 800_000).filter(i -> i % 100_000 == 0).toArray();
        int[] sizes_1M = IntStream.rangeClosed(200_000, 3_000_000).filter(i -> i < 1_000_000 ? i % 200_000 == 0 : i % 1_000_000 == 0).toArray();
        bk("warm-up", 100, sizes_warm, 3, all_func_indices);
        Thread.sleep(3210); // give IntelliJ launcher some time to cool down
        bk("100", 100, sizes_100, 5, all_func_indices);
        bk("10k", 10_000, sizes_10k, 3, all_func_indices);
        bk("1M", 1_000_000, sizes_1M, 3, most_func_indices);
    }
}
