package be.steinsomers.bron_kerbosch.study;

import be.steinsomers.bron_kerbosch.*;
import lombok.RequiredArgsConstructor;

import java.io.IOException;
import java.io.Writer;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.IntStream;

@RequiredArgsConstructor
final class TimedAlgo {
    final BronKerboschAlgorithm algo;
    final SampleStatistics time;

    String name() {
        return Portfolio.ALGOS.get(algo);
    }
}

@SuppressWarnings("UseOfConcreteClass")
enum Main {
    ;

    private static void bron_kerbosch_timed(final GraphTestData testData,
                                            final int timedSamples,
                                            final List<TimedAlgo> measurements)
            throws InterruptedException {
        Optional<List<List<Integer>>> firstOrdered = Optional.empty();
        final var graph = testData.graph();
        for (int sample = 0; sample <= timedSamples; ++sample) {
            for (final var measurement : measurements) {
                if (sample == 0) {
                    final var cliques = Collections.synchronizedCollection(new ArrayDeque<int[]>());
                    final var consumer = new CliqueConsumer(3, cliques::add);
                    measurement.algo.explore(graph, consumer);
                    final var ordered = Portfolio.OrderCliques(cliques);
                    if (firstOrdered.isEmpty()) {
                        if (cliques.size() != testData.cliqueCount()) {
                            throw new AssertionError("Inconsistent results");
                        }
                        firstOrdered = Optional.of(ordered);
                    } else {
                        if (!firstOrdered.get().equals(ordered)) {
                            throw new AssertionError("Inconsistent results");
                        }
                    }
                } else {
                    final var start = System.nanoTime();
                    final var cliqueCounter = new AtomicInteger();
                    final var consumer = new CliqueConsumer(3, (int[] _) -> cliqueCounter.incrementAndGet());
                    measurement.algo.explore(graph, consumer);
                    final var elapsed = System.nanoTime() - start;
                    final var cliqueCount = cliqueCounter.get();
                    if (cliqueCount != testData.cliqueCount()) {
                        throw new AssertionError("Inconsistent results");
                    }
                    measurement.time.put(elapsed);
                }
            }
        }
    }

    private static void bk(final boolean genuine,
                           final String orderStr,
                           final int order,
                           final int[] sizes,
                           final int timedSamples,
                           final List<BronKerboschAlgorithm> algos) {
        final var name = "random_time_java_order_" + (genuine ? orderStr : "warmup");
        final var path = Paths.get("..", "data", name + ".csv");
        try (final Writer fo = Files.newBufferedWriter(path, StandardCharsets.UTF_8)) {
            fo.write("Size");
            for (final var algo : algos) {
                final var fn = Portfolio.ALGOS.get(algo);
                fo.write(String.format(Locale.US, ",%s min,%s mean,%s max", fn, fn, fn));
            }
            fo.write(System.lineSeparator());

            for (final var size : sizes) {
                final var start = System.nanoTime();
                final var testData = GraphTestData.readUndirected(orderStr, order, size);
                final var elapsed = System.nanoTime() - start;
                if (genuine) {
                    System.out.printf("%4s nodes, %7d edges, creation: %6.3f%n",
                            orderStr, size, elapsed / 1e9);
                }
                var measurements = algos.stream().map(algo -> new TimedAlgo(algo, new SampleStatistics())).toList();
                bron_kerbosch_timed(testData, timedSamples, measurements);

                fo.write(String.format(Locale.US, "%d", size));
                for (final var mm : measurements) {
                    final double max = mm.time.max() / 1e9;
                    final double min = mm.time.min() / 1e9;
                    final double mean = mm.time.mean() / 1e9;
                    final double dev = mm.time.deviation() / 1e9;
                    fo.write(String.format(Locale.US, ",%f,%f,%f", min, mean, max));
                    if (genuine) {
                        System.out.printf("%4s nodes, %7d edges, %8s: %6.3f ± %.0f%%%n",
                                orderStr, size, mm.name(), mean, 100 * dev / mean);
                    }
                }
                fo.write(System.lineSeparator());
            }
        } catch (final InterruptedException x) {
            System.err.format("InterruptedException: %s%n", x);
        } catch (final IOException x) {
            System.err.format("IOException: %s%n", x);
        }
    }

    @SuppressWarnings("CommentedOutCode")
    public static void main(final String[] args) throws InterruptedException {
        assert false : "Omit -ea for meaningful measurements";

        final var allAlgos = new ArrayList<>(Portfolio.ALGOS.keySet());
        final var mostAlgos = allAlgos.stream().filter(algo -> !(algo instanceof BronKerbosch1)).toList();
        final var eliteAlgos = List.of(
                new BronKerbosch2_gp(),
                new BronKerbosch3_gp(),
                new BronKerbosch3_MT(),
                new BronKerbosch3_ST());
        final int[] sizes100 = IntStream.iterate(2_000, s -> s <= 3_000, s -> s + 50).toArray();
        final int[] sizes10K = IntStream.iterate(10_000, s -> s <= 200_000,
                s -> s + (s < 100_000 ? 10_000 : 25_000)).toArray();
        final int[] sizes1M = IntStream.iterate(500_000, s -> s <= 5_000_000,
                s -> s + (s < 2_000_000 ? 250_000 : 1_000_000)).toArray();

        bk(false, "100", 100, new int[]{2000}, 3, allAlgos); // warm up
        Thread.sleep(3210); // give IntelliJ launcher some time to cool down
        bk(true, "100", 100, sizes100, 5, allAlgos);
        bk(true, "10k", 10_000, sizes10K, 3, mostAlgos);
        bk(true, "1M", 1_000_000, sizes1M, 3, eliteAlgos);
        /*
        int[] sizes = {500_000};
        int[] funcIndices = {8};
        bk("tt", 1_000_000, sizes, 3, funcIndices);
        */
    }
}
