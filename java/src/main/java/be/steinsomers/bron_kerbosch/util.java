package be.steinsomers.bron_kerbosch;

import java.util.Arrays;
import java.util.Collection;
import java.util.Iterator;
import java.util.Set;
import java.util.stream.Stream;

@SuppressWarnings("TypeMayBeWeakened")
public final class util {
    public static int[] Append(int[] head, int tail) {
        var result = Arrays.copyOf(head, head.length + 1);
        result[head.length] = tail;
        return result;
    }

    public static <T> T PopArbitrary(Collection<? extends T> c) {
        Iterator<? extends T> it = c.iterator();
        T arbitrary = it.next();
        it.remove();
        return arbitrary;
    }

    public static <T> Stream<T> Difference(Set<T> set1, Set<T> set2) {
        return set1.stream().filter(v -> !set2.contains(v));
    }

    public static <T> Stream<T> Intersect(Set<T> set1, Set<T> set2) {
        if (set1.size() <= set2.size())
            return set1.stream().filter(set2::contains);
        else
            return set2.stream().filter(set1::contains);
    }

    public static <T> boolean AreDisjoint(Set<T> set1, Set<T> set2) {
        return Intersect(set1, set2).findFirst().isEmpty();
    }
}
