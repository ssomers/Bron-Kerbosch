package be.steinsomers.bron_kerbosch

import java.util.function.Predicate

object Util {
    @OptIn(ExperimentalStdlibApi::class)
    fun append(head: IntArray, tail: Vertex): IntArray {
        return head.copyOf(newSize = head.size + 1) { tail.index }
    }

    fun <T> popArbitrary(c: MutableCollection<out T>): T {
        val it: MutableIterator<T> = c.iterator()
        val arbitrary = it.next()
        it.remove()
        return arbitrary
    }

    fun <T> intersect(set1: Set<T>, set2: Set<T>): MutableSet<T> {
        return (if (set1.size <= set2.size) set1 intersect set2 else set2 intersect set1).toMutableSet()
    }

    fun <T> overlap(set1: Set<T>, set2: Set<T>): Int {
        return if (set1.size <= set2.size) set1.count(set2::contains) else set2.count(set1::contains)
    }

    fun <T> areDisjoint(set1: Set<T>, set2: Set<T>): Boolean {
        return if (set1.size <= set2.size) set1.none(set2::contains) else set2.none(set1::contains)
    }

    fun <T> partition(set: Set<T>, isFirst: Predicate<T>): Pair<MutableSet<T>, MutableSet<T>> {
        val first: MutableSet<T> = HashSet(set.size)
        val second: MutableSet<T> = HashSet(set.size)
        set.forEach { v -> (if (isFirst.test(v)) first else second).add(v) }
        return Pair(first, second)
    }
}
