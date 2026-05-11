package be.steinsomers.bron_kerbosch.study

import kotlin.math.sqrt
import kotlin.math.max
import kotlin.math.min

internal class SampleStatistics {
    private var max = 0L
    private var min = 0L
    private var samples = 0
    private var sum = 0.0
    private var sumOfSquares = 0.0

    fun put(v: Long) {
        if (samples == 0 || min > v) {
            min = v
        }
        if (samples == 0 || max < v) {
            max = v
        }
        samples += 1
        sum += v.toDouble()
        sumOfSquares += (v * v).toDouble()
    }

    fun min(): Long = min
    fun max(): Long = max

    fun mean(): Double = if (samples > 0) {
        max(min.toDouble(), min(max.toDouble(), sum / samples))
    } else {
        Double.NaN
    }

    fun variance(): Double = if (samples > 1) {
        val n = samples.toDouble()
        max(sumOfSquares - sum * sum / n, 0.0) / (n - 1.0)
    } else {
        Double.NaN
    }

    fun deviation(): Double = min((max - min).toDouble(), sqrt(variance()))
}
