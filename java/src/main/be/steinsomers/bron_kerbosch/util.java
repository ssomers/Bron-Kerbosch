package be.steinsomers.bron_kerbosch;

import java.util.Arrays;
import java.util.Collection;
import java.util.Iterator;
import java.util.Set;
import java.util.stream.Stream;

@SuppressWarnings("TypeMayBeWeakened")
public enum util {
    ;

    public static int[] Append(final int[] head, final int tail) {
        final var result = Arrays.copyOf(head, head.length + 1);
        result[head.length] = tail;
        return result;
    }

    public static <T> T PopArbitrary(final Collection<? extends T> c) {
        final Iterator<? extends T> it = c.iterator();
        final T arbitrary = it.next();
        it.remove();
        return arbitrary;
    }

    public static <T> Stream<T> Difference(final Set<T> set1, final Set<T> set2) {
        return set1.stream().filter(v -> !set2.contains(v));
    }

    public static <T> Stream<T> Intersect(final Set<T> set1, final Set<T> set2) {
        if (set1.size() <= set2.size())
            return set1.stream().filter(set2::contains);
        else
            return set2.stream().filter(set1::contains);
    }

    public static <T> boolean AreDisjoint(final Set<T> set1, final Set<T> set2) {
        if (set1.size() <= set2.size())
            return set1.stream().noneMatch(set2::contains);
        else
            return set2.stream().noneMatch(set1::contains);
    }
}
