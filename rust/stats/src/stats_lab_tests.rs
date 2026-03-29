use super::*;

#[test]
fn empty_i32() {
    let s: SampleStatistics<i32> = Default::default();
    assert!(s.mean().is_nan());
    assert!(s.variance().is_nan());
    assert!(s.deviation().is_nan());
}

#[test]
fn of_1_i32() {
    let mut s: SampleStatistics<i32> = Default::default();
    s.put(-1);
    assert_eq!(s.mean(), -1.0);
    assert!(s.variance().is_nan());
    assert!(s.deviation().is_nan());
}

#[test]
fn of_2_i32() {
    let mut s: SampleStatistics<i32> = Default::default();
    s.put(-1);
    s.put(1);
    assert_eq!(s.mean(), 0.0);
    assert_eq!(s.variance(), 2.0);
    assert_eq!(s.deviation(), 2.0_f64.sqrt());
}

#[test]
fn of_3_i32() {
    let mut s: SampleStatistics<i32> = Default::default();
    s.put(89);
    s.put(90);
    s.put(91);
    assert_eq!(s.mean(), 90.0);
    #[cfg(not(miri))]
    assert_eq!(s.variance(), 1.0);
    #[cfg(not(miri))]
    assert_eq!(s.deviation(), 1.0);
}

#[test]
fn of_9_u32() {
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
    #[cfg(not(miri))]
    assert_eq!(s.variance(), 4.0);
    #[cfg(not(miri))]
    assert_eq!(s.deviation(), 2.0);
}

#[test]
fn of_2_f64() {
    let mut s: SampleStatistics<f64> = Default::default();
    s.put(1.0);
    s.put(2.0);
    assert_eq!(s.mean(), 1.5);
    #[cfg(not(miri))]
    assert_eq!(s.variance(), 0.5);
    #[cfg(not(miri))]
    assert_eq!(s.deviation(), 0.5_f64.sqrt());
}
