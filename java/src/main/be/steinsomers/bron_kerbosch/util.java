package be.steinsomers.bron_kerbosch;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Iterator;
import java.util.List;
import java.util.Random;
import java.util.Set;
import java.util.stream.Stream;

public final class util {
    public static <T> List<T> Append(List<? extends T> head, T tail) {
        List<T> result = new ArrayList<>(head.size() + 1);
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

    public static Stream<Integer> Difference(Set<Integer> set1, Set<Integer> set2) {
        return set1.stream().filter(v -> !set2.contains(v));
    }

    public static Stream<Integer> Intersect(Set<Integer> set1, Set<Integer> set2) {
        if (set1.size() <= set2.size())
            return set1.stream().filter(set2::contains);
        else
            return set2.stream().filter(set1::contains);
    }

    public static boolean AreDisjoint(Set<Integer> set1, Set<Integer> set2) {
        return Intersect(set1, set2).findFirst().isEmpty();
    }

    public static int RandomChoice(Random rng, List<Integer> list) {
        var i = rng.nextInt(list.size());
        return list.get(i);
    }

    public static int RandomSample(Random rng, Set<Integer> set) {
        var i = rng.nextInt(set.size());
        return set.stream().skip(i).findFirst().orElseThrow();
    }

    public static void RemoveFrom(ArrayList<Integer> list, int value) {
        var index = list.indexOf(value);
        var last = list.size() - 1;
        list.set(index, list.get(last));
        list.remove(last);
    }
}
