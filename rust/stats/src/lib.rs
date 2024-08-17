#[derive(Clone, Default)]
pub struct SampleStatistics<T> {
    max: T,
    min: T,
    samples: u32,
    sum: f64,
    sum_of_squares: f64,
}

impl<T> SampleStatistics<T>
where
    T: Copy + PartialOrd + std::ops::Sub,
    f64: std::convert::From<T>,
    f64: std::convert::From<<T as std::ops::Sub>::Output>,
{
    pub fn is_empty(&self) -> bool {
        self.samples == 0
    }
    pub fn put(&mut self, v: T) {
        let vf = f64::from(v);
        if self.is_empty() {
            self.min = v;
            self.max = v;
        } else if self.min > v {
            self.min = v;
        } else if self.max < v {
            self.max = v;
        }
        self.samples += 1;
        self.sum += vf;
        self.sum_of_squares += vf.powi(2);
    }

    pub fn max(&self) -> T {
        self.max
    }
    pub fn min(&self) -> T {
        self.min
    }
    pub fn mean(&self) -> f64 {
        if self.samples < 1 {
            f64::NAN
        } else {
            let r = self.sum / (self.samples as f64);
            r.clamp(self.min.into(), self.max.into())
        }
    }
    pub fn variance(&self) -> f64 {
        if self.samples < 2 {
            f64::NAN
        } else if self.min == self.max {
            0.
        } else {
            let n = self.samples as f64;
            let r = (self.sum_of_squares - self.sum.powi(2) / n) / (n - 1.);
            r.max(0.)
        }
    }
    pub fn deviation(&self) -> f64 {
        let r = self.variance().sqrt();
        if r.is_nan() {
            r
        } else {
            r.min((self.max - self.min).into())
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]

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
        s.put(-1);
        assert_eq!(s.mean(), -1.0);
        assert!(s.variance().is_nan());
        assert!(s.deviation().is_nan());
    }

    #[test]
    fn stats_2_i32() {
        let mut s: SampleStatistics<i32> = Default::default();
        s.put(-1);
        s.put(1);
        assert_eq!(s.mean(), 0.0);
        assert_eq!(s.variance(), 2.0);
        assert_eq!(s.deviation(), 2.0_f64.sqrt());
    }

    #[test]
    fn stats_3_i32() {
        let mut s: SampleStatistics<i32> = Default::default();
        s.put(89);
        s.put(90);
        s.put(91);
        assert_eq!(s.mean(), 90.0);
        assert_eq!(s.variance(), 1.0);
        assert_eq!(s.deviation(), 1.0);
    }

    #[test]
    fn stats_9_u32() {
        let mut s: SampleStatistics<u32> = Default::default();
        s.put(2);
        s.put(4);
        s.put(4);
        s.put(4);
        s.put(5);
        s.put(5);
        s.put(5);
        s.put(7);
        s.put(9);
        assert_eq!(s.mean(), 5.0);
        assert_eq!(s.variance(), 4.0);
        assert_eq!(s.deviation(), 2.0);
    }

    #[test]
    fn stats_2_f64() {
        let mut s: SampleStatistics<f64> = Default::default();
        s.put(1.0);
        s.put(2.0);
        assert_eq!(s.mean(), 1.5);
        assert_eq!(s.variance(), 0.5);
        assert_eq!(s.deviation(), 0.5_f64.sqrt());
    }
}

#[cfg(all(test, not(miri)))]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn put_1_u32(x in proptest::num::u32::ANY) {
            let mut s: SampleStatistics<u32> = Default::default();
            s.put(x);
            assert!(s.mean() == x.into());
        }

        #[test]
        fn put_1_f64(x in proptest::num::f64::NORMAL) {
            let mut s: SampleStatistics<f64> = Default::default();
            s.put(x);
            assert!(s.mean() == x.into());
        }

        #[test]
        fn put_2_u32(x in proptest::num::u32::ANY, y in proptest::num::u32::ANY) {
            let mut s: SampleStatistics<u32> = Default::default();
            s.put(x);
            s.put(y);
            assert!(s.mean() >= s.min().into());
            assert!(s.mean() <= s.max().into());
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()).into());
        }

        #[test]
        fn put_2_f64(x in proptest::num::f64::NORMAL, y in proptest::num::f64::NORMAL) {
            let mut s: SampleStatistics<f64> = Default::default();
            s.put(x);
            s.put(y);
            assert!(s.mean() >= s.min());
            assert!(s.mean() <= s.max());
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()) * 1.5);
        }

        #[test]
        fn put_n_u32(i in 2..99, x in proptest::num::u32::ANY) {
            let mut s: SampleStatistics<u32> = Default::default();
            for _ in 0..i {
                s.put(x);
            }
            assert!(s.mean() >= s.min().into());
            assert!(s.mean() <= s.max().into());
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()).into());
        }

        #[test]
        fn put_n_f64(i in 2..99, x in proptest::num::f64::NORMAL) {
            let mut s: SampleStatistics<f64> = Default::default();
            for _ in 0..i {
                s.put(x);
            }
            assert!(s.mean() >= s.min());
            assert!(s.mean() <= s.max());
            assert!(s.variance() >= 0.);
            assert!(s.deviation() <= (s.max() - s.min()));
        }
    }
}
