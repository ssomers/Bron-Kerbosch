class SampleStatistics {
  var max: Double = 0
  var min: Double = 0
  var samples: Int = 0
  var sum: Double = 0
  var sum_of_squares: Double = 0

  def put(v: Double): Unit = {
    if (samples == 0) {
      min = v
      max = v
    } else if (min > v) {
      min = v
    } else if (max < v) {
      max = v
    }
    samples += 1
    sum += v
    sum_of_squares += v * v
  }

  def mean(): Double = {
    if (samples > 0) {
      math.max(min, math.min(max, sum / samples))
    } else {
      Double.NaN
    }
  }
  def variance(): Double = {
    if (samples > 1) {
      val n = samples
      math.max(sum_of_squares - sum * sum / n, 0) / (n - 1.0)
    } else {
      Double.NaN
    }
  }
  def deviation(): Double = {
    math.min(max - min, math.sqrt(variance()))
  }
}
