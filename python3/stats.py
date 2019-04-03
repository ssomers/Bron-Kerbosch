from math import nan, sqrt


class SampleStatistics(object):
    def __init__(self):
        self.max = nan
        self.min = nan
        self.samples = 0
        self.sum = 0
        self.sum_of_squares = 0

    def put(self, v):
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

    def mean(self):
        if self.samples > 0:
            return self.sum / self.samples
        else:
            return nan

    def variance(self):
        if self.samples > 1:
            n = self.samples
            return max(self.sum_of_squares - self.sum * self.sum / n,
                       0) / (n - 1.)
        else:
            return nan

    def deviation(self):
        return sqrt(self.variance())
