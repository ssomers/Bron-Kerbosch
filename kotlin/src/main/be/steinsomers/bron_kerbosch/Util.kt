package be.steinsomers.bron_kerbosch

object Util {
    @OptIn(ExperimentalStdlibApi::class)
    fun append(head: IntArray, tail: Int): IntArray {
        return head.copyOf(newSize = head.size + 1) { tail }
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

    fun intersect(set1: Set<Int>, set2: BooleanArray): MutableSet<Int> {
        return (set1.filter { v: Int -> set2[v] }).toMutableSet()
    }

    fun <T> intersectionSize(set1: Set<T>, set2: Set<T>): Int {
        return if (set1.size <= set2.size) set1.count(set2::contains) else set2.count(set1::contains)
    }

    fun <T> areDisjoint(set1: Set<T>, set2: Set<T>): Boolean {
        return if (set1.size <= set2.size) set1.none(set2::contains) else set2.none(set1::contains)
    }
}
