package BronKerbosch

import (
	"math"
)

type SampleStatistics struct {
	max          float64
	min          float64
	samples      int
	sum          float64
	sumOfSquares float64
}

func (s *SampleStatistics) Put(v float64) {
	if s.samples == 0 {
		s.min = v
		s.max = v
	} else if s.min > v {
		s.min = v
	} else if s.max < v {
		s.max = v
	}
	s.samples++
	s.sum += v
	s.sumOfSquares += v * v
}

func (s *SampleStatistics) Max() float64 {
	return s.max
}

func (s *SampleStatistics) Min() float64 {
	return s.min
}

func (s *SampleStatistics) Mean() float64 {
	return s.sum / float64(s.samples)
}

func (s *SampleStatistics) Variance() float64 {
	n := float64(s.samples)
	numerator := s.sumOfSquares - s.sum*s.sum/n
	if numerator < 0 {
		return 0
	}
	return numerator / (n - 1)
}

func (s *SampleStatistics) Deviation() float64 {
	return math.Sqrt(s.Variance())
}
