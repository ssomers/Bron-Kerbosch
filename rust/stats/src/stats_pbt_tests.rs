use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn put_1_u32(x in proptest::num::u32::ANY) {
        let mut s: SampleStatistics<u32> = Default::default();
        s.put(x);
        assert_eq!(s.mean(), x.into());
    }

    #[test]
    fn put_1_f64(x in proptest::num::f64::NORMAL) {
        let mut s: SampleStatistics<f64> = Default::default();
        s.put(x);
        assert_eq!(s.mean(), x);
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
