package be.steinsomers.bron_kerbosch.study

import org.junit.jupiter.api.Assertions
import org.junit.jupiter.api.Test
import java.lang.Double.isNaN
import kotlin.math.sqrt

internal class SampleStatisticsTest {
    @Test
    fun zero() {
        val s = SampleStatistics()
        Assertions.assertTrue(isNaN(s.mean()))
        Assertions.assertTrue(isNaN(s.variance()))
        Assertions.assertTrue(isNaN(s.deviation()))
    }

    @Test
    fun one() {
        val s = SampleStatistics()
        s.put(-1)
        Assertions.assertEquals(s.mean(), -1.0)
        Assertions.assertTrue(isNaN(s.variance()))
        Assertions.assertTrue(isNaN(s.deviation()))
    }

    @Test
    fun two() {
        val s = SampleStatistics()
        s.put(-1)
        s.put(1)
        Assertions.assertEquals(s.mean(), 0.0)
        Assertions.assertEquals(s.variance(), 2.0)
        Assertions.assertEquals(s.deviation(), sqrt(2.0))
    }

    @Test
    fun three() {
        val s = SampleStatistics()
        s.put(89)
        s.put(90)
        s.put(91)
        Assertions.assertEquals(s.mean(), 90.0)
        Assertions.assertEquals(s.variance(), 1.0)
        Assertions.assertEquals(s.deviation(), 1.0)
    }

    @Test
    fun nine() {
        val s = SampleStatistics()
        s.put(2)
        s.put(4)
        s.put(4)
        s.put(4)
        s.put(5)
        s.put(5)
        s.put(5)
        s.put(7)
        s.put(9)
        Assertions.assertEquals(s.mean(), 5.0)
        Assertions.assertEquals(s.variance(), 4.0)
        Assertions.assertEquals(s.deviation(), 2.0)
    }
}
