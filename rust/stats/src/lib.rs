extern crate num_traits;
use num_traits::{ToPrimitive, Zero};

#[derive(Clone, Copy)]
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
    T: Clone + PartialOrd + ToPrimitive + Zero,
{
    pub fn new() -> Self {
        SampleStatistics {
            samples: 0,
            sum: 0.0,
            sum_of_squares: 0.0,
            min: Zero::zero(),
            max: Zero::zero(),
        }
    }

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

    pub fn is_populated(&self) -> bool {
        self.samples > 1
    }

    pub fn mean(&self) -> f64 {
        if self.is_populated() {
            self.sum / self.samples as f64
        } else {
            std::f64::NAN
        }
    }
    pub fn variance(&self) -> f64 {
        if self.is_populated() {
            let n = self.samples as f64;
            let m = self.sum / n;
            (self.sum_of_squares - 2.0 * m * self.sum + m.powi(2) * n) / (n - 1.0)
        } else {
            std::f64::NAN
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
        let s: SampleStatistics<i32> = SampleStatistics::new();
        assert!(!s.is_populated());
        assert!(s.mean().is_nan());
        assert!(s.variance().is_nan());
        assert!(s.deviation().is_nan());
    }

    #[test]
    fn stats_1_i32() {
        let mut s: SampleStatistics<i32> = SampleStatistics::new();
        s.put(-1).unwrap();
        assert!(!s.is_populated());
        assert!(s.mean().is_nan());
        assert!(s.variance().is_nan());
        assert!(s.deviation().is_nan());
    }

    #[test]
    fn stats_2_i32() {
        let mut s: SampleStatistics<i32> = SampleStatistics::new();
        s.put(-1).unwrap();
        s.put(1).unwrap();
        assert!(s.is_populated());
        assert_eq!(s.mean(), 0.0);
        assert_eq!(s.variance(), 2.0);
        assert_eq!(s.deviation(), 2.0_f64.sqrt());
    }

    #[test]
    fn stats_3_i32() {
        let mut s: SampleStatistics<i32> = SampleStatistics::new();
        s.put(89).unwrap();
        s.put(90).unwrap();
        s.put(91).unwrap();
        assert!(s.is_populated());
        assert_eq!(s.mean(), 90.0);
        assert_eq!(s.variance(), 1.0);
        assert_eq!(s.deviation(), 1.0);
    }

    #[test]
    fn stats_9_u32() {
        let mut s: SampleStatistics<u32> = SampleStatistics::new();
        s.put(2).unwrap();
        s.put(4).unwrap();
        s.put(4).unwrap();
        s.put(4).unwrap();
        s.put(5).unwrap();
        s.put(5).unwrap();
        s.put(5).unwrap();
        s.put(7).unwrap();
        s.put(9).unwrap();
        assert!(s.is_populated());
        assert_eq!(s.mean(), 5.0);
        assert_eq!(s.variance(), 4.0);
        assert_eq!(s.deviation(), 2.0);
    }

    #[test]
    fn stats_2_f32() {
        let mut s: SampleStatistics<f32> = SampleStatistics::new();
        s.put(1.0).unwrap();
        s.put(2.0).unwrap();
        assert!(s.is_populated());
        assert_eq!(s.mean(), 1.5);
        assert_eq!(s.variance(), 0.5);
        assert_eq!(s.deviation(), 0.5_f64.sqrt());
    }

    #[test]
    fn stats_2_f64() {
        let mut s: SampleStatistics<f64> = SampleStatistics::new();
        s.put(1.0).unwrap();
        s.put(2.0).unwrap();
        assert!(s.is_populated());
        assert_eq!(s.mean(), 1.5);
        assert_eq!(s.variance(), 0.5);
        assert_eq!(s.deviation(), 0.5_f64.sqrt());
    }
}
