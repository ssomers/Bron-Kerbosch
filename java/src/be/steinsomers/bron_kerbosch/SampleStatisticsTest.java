package be.steinsomers.bron_kerbosch;

import org.junit.jupiter.api.Test;

class SampleStatisticsTest {
    @Test
    void zero() {
        var s = new SampleStatistics();
        assert Double.isNaN(s.mean());
        assert Double.isNaN(s.variance());
        assert Double.isNaN(s.deviation());
    }

    @Test
    void one() {
        var s = new SampleStatistics();
        s.put(-1);
        assert (s.mean() == -1.0);
        assert Double.isNaN(s.variance());
        assert Double.isNaN(s.deviation());
    }

    @Test
    void two() {
        var s = new SampleStatistics();
        s.put(-1);
        s.put(1);
        assert (s.mean() == 0.0);
        assert (s.variance() == 2.0);
        assert (s.deviation() == Math.sqrt(2.0));
    }

    @Test
    void three() {
        var s = new SampleStatistics();
        s.put(89);
        s.put(90);
        s.put(91);
        assert (s.mean() == 90.0);
        assert (s.variance() == 1.0);
        assert (s.deviation() == 1.0);
    }

    @Test
    void nigh() {
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
        assert (s.mean() == 5.0);
        assert (s.variance() == 4.0);
        assert (s.deviation() == 2.0);
    }
}
