package be.steinsomers.bron_kerbosch.study;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

final class SampleStatisticsTest {
    @Test
    void zero() {
        var s = new SampleStatistics();
        assertTrue(Double.isNaN(s.mean()));
        assertTrue(Double.isNaN(s.variance()));
        assertTrue(Double.isNaN(s.deviation()));
    }

    @Test
    void one() {
        var s = new SampleStatistics();
        s.put(-1);
        assertEquals(s.mean(), -1.0);
        assertTrue(Double.isNaN(s.variance()));
        assertTrue(Double.isNaN(s.deviation()));
    }

    @Test
    void two() {
        var s = new SampleStatistics();
        s.put(-1);
        s.put(1);
        assertEquals(s.mean(), 0.0);
        assertEquals(s.variance(), 2.0);
        assertEquals(s.deviation(), Math.sqrt(2.0));
    }

    @Test
    void three() {
        var s = new SampleStatistics();
        s.put(89);
        s.put(90);
        s.put(91);
        assertEquals(s.mean(), 90.0);
        assertEquals(s.variance(), 1.0);
        assertEquals(s.deviation(), 1.0);
    }

    @Test
    void nine() {
        var s = new SampleStatistics();
        s.put(2);
        s.put(4);
        s.put(4);
        s.put(4);
        s.put(5);
        s.put(5);
        s.put(5);
        s.put(7);
        s.put(9);
        assertEquals(s.mean(), 5.0);
        assertEquals(s.variance(), 4.0);
        assertEquals(s.deviation(), 2.0);
    }
}
