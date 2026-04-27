package be.steinsomers.bron_kerbosch;

import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.Map;
import java.util.stream.IntStream;

public final class Portfolio {
    public static final Map<BronKerboschAlgorithm, String> ALGOS = Map.of(
            new BronKerbosch1(), "Ver1½",
            new BronKerbosch2(), "Ver2½",
            new BronKerbosch2_gp(), "Ver2½-GP",
            new BronKerbosch2_gpx(), "Ver2½-GPX",
            new BronKerbosch3(), "Ver3½",
            new BronKerbosch3_gp(), "Ver3½-GP",
            new BronKerbosch3_gpx(), "Ver3½-GPX",
            new BronKerbosch3_MT(), "Ver3½=GPc",
            new BronKerbosch3_ST(), "Ver3½=GPs"
    );

    public static List<List<Integer>> OrderCliques(final Collection<int[]> cliques) {
        assert cliques.stream().allMatch(clique -> clique.length > 1);
        return cliques.stream()
                .map(clique -> Arrays.stream(clique).sorted().boxed().toList())
                .sorted((clique1, clique2) ->
                        IntStream.range(0, Math.min(clique1.size(), clique2.size()))
                                .map((int i) -> clique1.get(i) - clique2.get(i))
                                .filter((int diff) -> diff != 0)
                                .findFirst()
                                .orElseThrow(() -> new IllegalArgumentException(
                                        "got overlapping or equal cliques %s <> %s".formatted(clique1, clique2)
                                )))
                .toList();
    }
}
