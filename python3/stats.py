from math import sqrt


class SampleStatistics(object):
    def __init__(self):
        self.max = None
        self.min = None
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

    def is_populated(self) -> bool:
        return self.samples > 1

    def mean(self):
        return self.sum / self.samples

    def variance(self):
        n = self.samples
        m = self.sum / n
        return (self.sum_of_squares - 2.0 * m * self.sum + m * m * n) / (
            n - 1.0)

    def deviation(self):
        return sqrt(self.variance())
