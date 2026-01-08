package be.steinsomers.bron_kerbosch.study;

final class SampleStatistics {
    private long max;
    private long min;
    private int samples;
    private double sum;
    private double sum_of_squares;

    public void put(final long v) {
        if (samples == 0 || min > v) {
            min = v;
        }
        if (samples == 0 || max < v) {
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
            return Math.max(min, Math.min(max, sum / samples));
        } else {
            return Double.NaN;
        }
    }

    public double variance() {
        if (samples > 1) {
            final double n = samples;
            return Math.max(sum_of_squares - sum * sum / n, 0) / (n - 1.0);
        } else {
            return Double.NaN;
        }
    }

    public double deviation() {
        return Math.min(max - min, Math.sqrt(variance()));
    }
}
