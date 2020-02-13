from stats import SampleStatistics

from hypothesis import given
from hypothesis.strategies import floats, lists
from math import isnan, sqrt
import pytest


def test_stats_0_i32():
    s = SampleStatistics()
    assert isnan(s.mean())
    assert isnan(s.variance())
    assert isnan(s.deviation())


def test_stats_1_int():
    s = SampleStatistics()
    s.put(-1)
    assert s.mean() == -1.0
    assert isnan(s.variance())
    assert isnan(s.deviation())


def test_stats_2_int():
    s = SampleStatistics()
    s.put(-1)
    s.put(1)
    assert s.mean() == 0.0
    assert s.variance() == 2.0
    assert s.deviation() == sqrt(2.0)


def test_stats_3_int():
    s = SampleStatistics()
    s.put(89)
    s.put(90)
    s.put(91)
    assert s.mean() == 90.0
    assert s.variance() == 1.0
    assert s.deviation() == 1.0


def test_stats_9_int():
    s = SampleStatistics()
    s.put(2)
    s.put(4)
    s.put(4)
    s.put(4)
    s.put(5)
    s.put(5)
    s.put(5)
    s.put(7)
    s.put(9)
    assert s.mean() == 5.0
    assert s.variance() == 4.0
    assert s.deviation() == 2.0


def test_stats_2_float():
    s = SampleStatistics()
    s.put(1.0)
    s.put(2.0)
    assert s.mean() == 1.5
    assert s.variance() == 0.5
    assert s.deviation() == sqrt(0.5)


def test_stats_3_float_deviation_big():
    # found by hypothesis
    s = SampleStatistics()
    s.put(688338275.2675972)
    s.put(688338275.2675972)
    s.put(688338275.2675972)
    assert s.max == s.min
    assert (s.sum_of_squares - s.sum * s.sum / 3) / 2 > 0

def test_stats_3_float_deviation_small():
    # found by hypothesis
    s = SampleStatistics()
    s.put(1.5765166949677225e-06)
    s.put(1.5765166949677225e-06)
    s.put(1.5765166949677225e-06)
    assert s.max == s.min
    assert (s.sum_of_squares - s.sum * s.sum / 3) / 2 > 0

def test_stats_3_float_mean_small():
    # found by hypothesis
    s = SampleStatistics()
    s.put(-9.020465019382587e+92)
    s.put(-9.020465019382587e+92)
    s.put(-9.020465019382587e+92)
    assert s.sum / s.samples < s.min
    assert s.mean() == s.min

@given(lists(floats(min_value=-1e100, max_value=1e100), min_size=2))
def test_stats_floats(samples):
    s = SampleStatistics()
    for sample in samples:
        s.put(sample)
    assert s.mean() >= s.min
    assert s.mean() <= s.max
    assert s.variance() >= 0.
    if s.min < s.max:
        assert s.deviation() <= s.max - s.min
