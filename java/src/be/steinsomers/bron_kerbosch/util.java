package be.steinsomers.bron_kerbosch;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public final class util {
    public static <T> ArrayList<T> Append(List<T> head, T tail) {
        var result = new ArrayList(head.size() + 1);
        result.addAll(head);
        result.add(tail);
        return result;
    }

    public static <T> T PopArbitrary(Collection<? extends T> c) {
        Iterator<? extends T> it = c.iterator();
        T arbitrary = it.next();
        it.remove();
        return arbitrary;
    }

    public static Set<Integer> Intersect(Set<Integer> vset1, Set<Integer> vset2) {
        if (vset1.size() <= vset2.size())
            return vset1.stream().filter(vset2::contains).collect(Collectors.toSet());
        else
            return vset2.stream().filter(vset1::contains).collect(Collectors.toSet());
    }

    public static long intersection_size(Set<Integer> vset1, Set<Integer> vset2) {
        if (vset1.size() <= vset2.size())
            return vset1.stream().filter(vset2::contains).count();
        else
            return vset2.stream().filter(vset1::contains).count();
    }

    public static boolean AreDisjoint(Set<Integer> vset1, Set<Integer> vset2) {
        if (vset1.size() <= vset2.size())
            return vset1.stream().filter(vset2::contains).findFirst().isEmpty();
        else
            return vset2.stream().filter(vset1::contains).findFirst().isEmpty();
    }

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
}
