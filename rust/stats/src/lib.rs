extern crate num_traits;
use num_traits::ToPrimitive;

#[derive(Clone, Default)]
pub struct SampleStatistics<T> {
    max: T,
    min: T,
    samples: u32,
    sum: f64,
    sum_of_squares: f64,
}

#[derive(Debug)]
pub struct StatisticalTypeError<T> {
    uncastable_value: T,
}

impl<T> SampleStatistics<T>
where
    T: Clone + PartialOrd + ToPrimitive,
{
    pub fn put(&mut self, v: T) -> Result<(), StatisticalTypeError<T>> {
        if let Some(vf) = v.to_f64() {
            if self.samples == 0 {
                self.min = v.clone();
                self.max = v;
            } else if self.min > v {
                self.min = v;
            } else if self.max < v {
                self.max = v;
            }
            self.samples += 1;
            self.sum += vf;
            self.sum_of_squares += vf.powi(2);
            Ok(())
        } else {
            Err(StatisticalTypeError {
                uncastable_value: v,
            })
        }
    }

    pub fn max(&self) -> T {
        self.max.clone()
    }
    pub fn min(&self) -> T {
        self.min.clone()
    }
    pub fn mean(&self) -> f64 {
        if self.samples < 1 {
            std::f64::NAN
        } else if self.min == self.max {
            self.min.to_f64().unwrap()
        } else {
            self.sum / self.samples as f64
        }
    }
    pub fn variance(&self) -> f64 {
        if self.samples < 2 {
            std::f64::NAN
        } else if self.min == self.max {
            0.
        } else {
            let n = self.samples as f64;
            // may become slightly negative because of rounding:
            (self.sum_of_squares - self.sum.powi(2) / n).max(0.) / (n - 1.)
        }
    }
    pub fn deviation(&self) -> f64 {
        self.variance().sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_0_i32() {
        let s: SampleStatistics<i32> = Default::default();
        assert!(s.mean().is_nan());
        assert!(s.variance().is_nan());
        assert!(s.deviation().is_nan());
    }

    #[test]
    fn stats_1_i32() {
        let mut s: SampleStatistics<i32> = Default::default();
        s.put(-1).unwrap();
        assert_eq!(s.mean(), -1.0);
        assert!(s.variance().is_nan());
        assert!(s.deviation().is_nan());
    }

    #[test]
    fn stats_2_i32() {
        let mut s: SampleStatistics<i32> = Default::default();
        s.put(-1).unwrap();
        s.put(1).unwrap();
        assert_eq!(s.mean(), 0.0);
        assert_eq!(s.variance(), 2.0);
        assert_eq!(s.deviation(), 2.0_f64.sqrt());
    }

    #[test]
    fn stats_3_i32() {
        let mut s: SampleStatistics<i32> = Default::default();
        s.put(89).unwrap();
        s.put(90).unwrap();
        s.put(91).unwrap();
        assert_eq!(s.mean(), 90.0);
        assert_eq!(s.variance(), 1.0);
        assert_eq!(s.deviation(), 1.0);
    }

    #[test]
    fn stats_9_u32() {
        let mut s: SampleStatistics<u32> = Default::default();
        s.put(2).unwrap();
        s.put(4).unwrap();
        s.put(4).unwrap();
        s.put(4).unwrap();
        s.put(5).unwrap();
        s.put(5).unwrap();
        s.put(5).unwrap();
        s.put(7).unwrap();
        s.put(9).unwrap();
        assert_eq!(s.mean(), 5.0);
        assert_eq!(s.variance(), 4.0);
        assert_eq!(s.deviation(), 2.0);
    }

    #[test]
    fn stats_2_f64() {
        let mut s: SampleStatistics<f64> = Default::default();
        s.put(1.0).unwrap();
        s.put(2.0).unwrap();
        assert_eq!(s.mean(), 1.5);
        assert_eq!(s.variance(), 0.5);
        assert_eq!(s.deviation(), 0.5_f64.sqrt());
    }
}

#[cfg(test)]
mod proptests {
    extern crate proptest;
    use self::proptest::prelude::*;
    use super::*;

    proptest! {
        #[test]
        fn put_1_u32(x in proptest::num::u32::ANY) {
            let mut s: SampleStatistics<u32> = Default::default();
            s.put(x).unwrap();
            assert!(s.mean() >= s.min() as f64);
            assert!(s.mean() <= s.max() as f64);
        }

        #[test]
        fn put_1_f64(x in proptest::num::f64::NORMAL) {
            let mut s: SampleStatistics<f64> = Default::default();
            s.put(x).unwrap();
            assert!(s.mean() >= s.min());
            assert!(s.mean() <= s.max());
        }

        #[test]
        fn put_2_u32(x in proptest::num::u32::ANY, y in proptest::num::u32::ANY) {
            let mut s: SampleStatistics<u32> = Default::default();
            s.put(x).unwrap();
            s.put(y).unwrap();
            assert!(s.mean() >= s.min() as f64);
            assert!(s.mean() <= s.max() as f64);
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()) as f64);
        }

        #[test]
        fn put_2_f64(x in proptest::num::f64::NORMAL, y in proptest::num::f64::NORMAL) {
            let mut s: SampleStatistics<f64> = Default::default();
            s.put(x).unwrap();
            s.put(y).unwrap();
            assert!(s.mean() >= s.min());
            assert!(s.mean() <= s.max());
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()));
        }

        #[test]
        fn put_n_u32(i in 2..99, x in proptest::num::u32::ANY) {
            let mut s: SampleStatistics<u32> = Default::default();
            for _ in 0..i {
                s.put(x).unwrap();
            }
            assert!(s.mean() >= s.min() as f64);
            assert!(s.mean() <= s.max() as f64);
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()) as f64);
        }

        #[test]
        fn put_n_f64(i in 2..99, x in proptest::num::f64::NORMAL) {
            let mut s: SampleStatistics<f64> = Default::default();
            for _ in 0..i {
                s.put(x).unwrap();
            }
            assert!(s.mean() >= s.min());
            assert!(s.mean() <= s.max());
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()));
        }
    }
}
