package main

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
	r := s.sum / float64(s.samples)
	if r < s.min {
		return s.min
	}
	if r > s.max {
		return s.max
	}
	return r
}

func (s *SampleStatistics) Variance() float64 {
	n := float64(s.samples)
	r := (s.sumOfSquares - s.sum*s.sum/n) / (n - 1)
	if r < 0 {
		return 0
	}
	return r
}

func (s *SampleStatistics) Deviation() float64 {
	r := math.Sqrt(s.Variance())
	m := s.max - s.min
	if r > m {
		return m
	}
	return r

}
