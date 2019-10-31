package be.steinsomers.bron_kerbosch.study;

final class SampleStatistics {
    private long max = 0L;
    private long min = 0L;
    private int samples = 0;
    private double sum = 0.0;
    private double sum_of_squares = 0.0;

    public void put(long v) {
        if (samples == 0) {
            min = v;
            max = v;
        } else if (min > v) {
            min = v;
        } else if (max < v) {
            max = v;
        }
        samples += 1;
        sum += v;
        sum_of_squares += v * v;
    }

    public long min() {
        return min;
    }

    public long max() {
        return max;
    }

    public double mean() {
        if (samples > 0) {
            return sum / samples;
        } else {
            return Double.NaN;
        }
    }

    public double variance() {
        if (samples > 1) {
            double n = samples;
            return Math.max(sum_of_squares - sum * sum / n, 0) / (n - 1.0);
        } else {
            return Double.NaN;
        }
    }

    public double deviation() {
        return Math.sqrt(variance());
    }
}
