from math import isfinite, isnan, nan, sqrt


class SampleStatistics(object):

    def __init__(self) -> None:
        self.max = nan
        self.min = nan
        self.samples: int = 0
        self.sum: float = 0
        self.sum_of_squares: float = 0

    def put(self, v: float) -> None:
        if self.samples == 0:
            self.min = v
            self.max = v
        elif self.min > v:
            self.min = v
        elif self.max < v:
            self.max = v
        self.samples += 1
        self.sum += v
        self.sum_of_squares += v * v

    def mean(self) -> float:
        if self.samples > 0 and isfinite(self.sum):
            r = self.sum / self.samples
            return max(self.min, min(self.max, r))
        else:
            return nan

    def variance(self) -> float:
        if self.samples > 1 and isfinite(self.sum_of_squares):
            n = self.samples
            r = (self.sum_of_squares - self.sum * self.sum / n) / (n - 1)
            return max(0, r)
        else:
            return nan

    def deviation(self) -> float:
        r = sqrt(self.variance())
        if isnan(r):
            return nan
        else:
            return min(self.max - self.min, r)
