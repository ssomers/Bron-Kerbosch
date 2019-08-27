package be.steinsomers.bron_kerbosch;

import java.util.*;
import java.util.stream.Collectors;

public final class util {
    public static <T> ArrayList<T> Append(List<T> head, T tail) {
        ArrayList<T> result = new ArrayList<>(head.size() + 1);
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

    public static HashSet<Integer> Intersect(HashSet<Integer> set1, Set<Integer> set2) {
        if (set1.size() <= set2.size())
            return set1.stream().filter(set2::contains).collect(Collectors.toCollection(HashSet::new));
        else
            return set2.stream().filter(set1::contains).collect(Collectors.toCollection(HashSet::new));
    }

    public static long intersection_size(Set<Integer> set1, Set<Integer> set2) {
        if (set1.size() <= set2.size())
            return set1.stream().filter(set2::contains).count();
        else
            return set2.stream().filter(set1::contains).count();
    }

    public static boolean AreDisjoint(Set<Integer> set1, Set<Integer> set2) {
        if (set1.size() <= set2.size())
            return set1.stream().filter(set2::contains).findFirst().isEmpty();
        else
            return set2.stream().filter(set1::contains).findFirst().isEmpty();
    }

    public static int random_choice(Random rng, ArrayList<Integer> list) {
        var i = rng.nextInt(list.size());
        return list.get(i);
    }

    public static int random_sample(Random rng, HashSet<Integer> set) {
        var i = rng.nextInt(set.size());
        return set.stream().skip(i).findFirst().orElseThrow();
    }

    public static void remove_from(ArrayList<Integer> list, int v) {
        var i = list.indexOf(v);
        var last = list.size() - 1;
        list.set(i, list.get(last));
        list.remove(last);
    }
}
