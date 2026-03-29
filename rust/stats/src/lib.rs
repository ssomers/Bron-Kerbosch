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
    f64: From<T>,
    f64: From<<T as std::ops::Sub>::Output>,
{
    pub fn is_empty(&self) -> bool {
        self.samples == 0
    }
    pub fn put(&mut self, v: T) {
        if self.is_empty() {
            self.min = v;
            self.max = v;
        } else if self.min > v {
            self.min = v;
        } else if self.max < v {
            self.max = v;
        }
        self.samples += 1;
        let vf = f64::from(v);
        self.sum += vf;
        self.sum_of_squares += vf * vf;
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
mod stats_lab_tests;
#[cfg(all(test, not(miri)))]
mod stats_pbt_tests;
