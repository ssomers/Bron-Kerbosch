package bron_kerbosch

import (
	"math"
)

type SampleStatistics struct {
	max            float64
	min            float64
	samples        int
	sum            float64
	sum_of_squares float64
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
	s.samples += 1
	s.sum += v
	s.sum_of_squares += v * v
}

func (s *SampleStatistics) Mean() float64 {
	return s.sum / float64(s.samples)
}

func (s *SampleStatistics) Variance() float64 {
	n := float64(s.samples)
	m := s.sum / n
	return (s.sum_of_squares - 2.0*m*s.sum + m*m*n) / (n - 1)
}

func (s *SampleStatistics) Deviation() float64 {
	return math.Sqrt(s.Variance())
}
