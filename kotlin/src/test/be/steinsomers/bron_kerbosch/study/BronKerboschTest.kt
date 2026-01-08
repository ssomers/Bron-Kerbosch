package be.steinsomers.bron_kerbosch.study

import be.steinsomers.bron_kerbosch.UndirectedGraph
import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test
import java.util.Collections
import kotlin.collections.ArrayDeque

internal class BronKerboschTest {
    @Test
    fun order_0() {
        bk(listOf(), listOf())
    }

    @Test
    fun order_1() {
        bk(listOf(listOf()), listOf())
    }

    @Test
    fun order_2_isolated() {
        bk(
            listOf(listOf(), listOf()),
            listOf()
        )
    }

    @Test
    fun order_2_connected() {
        bk(
            listOf(listOf(1), listOf(0)),
            listOf(listOf(0, 1))
        )
    }

    @Test
    fun order_3_size_1_left() {
        bk(
            listOf(listOf(1), listOf(0), listOf()),
            listOf(listOf(0, 1))
        )
    }

    @Test
    fun order_3_size_1_middle() {
        bk(
            listOf(listOf(2), listOf(), listOf(0)),
            listOf(listOf(0, 2))
        )
    }

    @Test
    fun order_3_size_1_right() {
        bk(
            listOf(listOf(), listOf(2), listOf(1)),
            listOf(listOf(1, 2))
        )
    }

    @Test
    fun order_3_size_2() {
        bk(
            listOf(listOf(1), listOf(0, 2), listOf(1)),
            listOf(listOf(0, 1), listOf(1, 2))
        )
    }

    @Test
    fun order_3_size_3() {
        bk(
            listOf(listOf(1, 2), listOf(0, 2), listOf(0, 1)),
            listOf(listOf(0, 1, 2))
        )
    }

    @Test
    fun order_4_size_2() {
        bk(
            listOf(listOf(1), listOf(0), listOf(3), listOf(2)),
            listOf(listOf(0, 1), listOf(2, 3))
        )
    }

    @Test
    fun order_4_size_3_bus() {
        bk(
            listOf(listOf(1), listOf(0, 2), listOf(1, 3), listOf(2)),
            listOf(listOf(0, 1), listOf(1, 2), listOf(2, 3))
        )
    }

    @Test
    fun order_4_size_3_star() {
        bk(
            listOf(listOf(1, 2, 3), listOf(0), listOf(0), listOf(0)),
            listOf(listOf(0, 1), listOf(0, 2), listOf(0, 3))
        )
    }

    @Test
    fun order_4_size_4_p() {
        bk(
            listOf(listOf(1), listOf(0, 2, 3), listOf(1, 3), listOf(1, 2)),
            listOf(listOf(0, 1), listOf(1, 2, 3))
        )
    }

    @Test
    fun order_4_size_4_square() {
        bk(
            listOf(listOf(1, 3), listOf(0, 2), listOf(1, 3), listOf(0, 2)),
            listOf(listOf(0, 1), listOf(0, 3), listOf(1, 2), listOf(2, 3))
        )
    }

    @Test
    fun order_4_size_5() {
        bk(
            listOf(listOf(1, 2, 3), listOf(0, 2), listOf(0, 1, 3), listOf(0, 2)),
            listOf(listOf(0, 1, 2), listOf(0, 2, 3))
        )
    }

    @Test
    fun order_4_size_6() {
        bk(
            listOf(listOf(1, 2, 3), listOf(0, 2, 3), listOf(0, 1, 3), listOf(0, 1, 2)),
            listOf(listOf(0, 1, 2, 3))
        )
    }

    @Test
    fun order_5_penultimate() {
        bk(
            listOf(
                listOf(1, 2, 3, 4),
                listOf(0, 2, 3, 4),
                listOf(0, 1, 3, 4),
                listOf(0, 1, 2),
                listOf(0, 1, 2)
            ),
            listOf(listOf(0, 1, 2, 3), listOf(0, 1, 2, 4))
        )
    }

    @Test
    fun sample() {
        bk(
            listOf(
                listOf(),
                listOf(2, 3, 4),
                listOf(1, 3, 4, 5),
                listOf(1, 2, 4, 5),
                listOf(1, 2, 3),
                listOf(2, 3, 6, 7),
                listOf(5, 7),
                listOf(5, 6)
            ),
            listOf(
                listOf(1, 2, 3, 4),
                listOf(2, 3, 5),
                listOf(5, 6, 7)
            )
        )
    }

    @Test
    fun bigger() {
        bk(
            listOf(
                listOf(1, 2, 3, 4, 6, 7),
                listOf(0, 3, 6, 7, 8, 9),
                listOf(0, 3, 5, 7, 8, 9),
                listOf(0, 1, 2, 4, 9),
                listOf(0, 3, 6, 7, 9),
                listOf(2, 6),
                listOf(0, 1, 4, 5, 9),
                listOf(0, 1, 2, 4, 9),
                listOf(1, 2),
                listOf(1, 2, 3, 4, 6, 7)
            ),
            listOf(
                listOf(0, 1, 3),
                listOf(0, 1, 6),
                listOf(0, 1, 7),
                listOf(0, 2, 3),
                listOf(0, 2, 7),
                listOf(0, 3, 4),
                listOf(0, 4, 6),
                listOf(0, 4, 7),
                listOf(1, 3, 9),
                listOf(1, 6, 9),
                listOf(1, 7, 9),
                listOf(1, 8),
                listOf(2, 3, 9),
                listOf(2, 5),
                listOf(2, 7, 9),
                listOf(2, 8),
                listOf(3, 4, 9),
                listOf(4, 6, 9),
                listOf(4, 7, 9),
                listOf(5, 6)
            )
        )
    }

    companion object {
        private fun bk(
            adjacenciesList: Collection<List<Int>>,
            expectedCliques: List<List<Int>>
        ) {
            val adjacencies = adjacenciesList.map { coll -> coll.toSet() }.toList()
            val graph = UndirectedGraph(adjacencies)
            for (funcIndex in Main.FUNCS.indices) {
                val funcName = Main.FUNC_NAMES[funcIndex]
                val rawCliques = Collections.synchronizedCollection(ArrayDeque<IntArray>())
                Main.FUNCS[funcIndex].explore(graph, rawCliques::add)
                val cliques = Main.orderCliques(rawCliques)
                Assertions.assertEquals(expectedCliques, cliques, "Unexpected result for $funcName")
            }
        }
    }
}
