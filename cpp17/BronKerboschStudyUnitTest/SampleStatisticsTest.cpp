#include "pch.h"

#include "BronKerboschStudy/SampleStatistics.h"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace BronKerboschStudy {
    TEST_CLASS(BronKerboschStudyUnitTest) {
public:
    TEST_METHOD(stats_0_int) {
        auto s = SampleStatistics<int>{};
        Assert::IsTrue(std::isnan(s.mean()));
        Assert::IsTrue(std::isnan(s.variance()));
        Assert::IsTrue(std::isnan(s.deviation()));
    }

    TEST_METHOD(stats_1_int) {
        auto s = SampleStatistics<int>{};
        s.put(-1);
        Assert::AreEqual(s.mean(), -1.0);
        Assert::IsTrue(std::isnan(s.variance()));
        Assert::IsTrue(std::isnan(s.deviation()));
    }

    TEST_METHOD(stats_2_int) {
        auto s = SampleStatistics<int>{};
        s.put(-1);
        s.put(1);
        Assert::AreEqual(s.mean(), 0.0);
        Assert::AreEqual(s.variance(), 2.0);
        Assert::AreEqual(s.deviation(), std::sqrt(2.0));
    }

    TEST_METHOD(stats_3_int) {
        auto s = SampleStatistics<int>{};
        s.put(89);
        s.put(90);
        s.put(91);
        Assert::AreEqual(s.mean(), 90.0);
        Assert::AreEqual(s.variance(), 1.0);
        Assert::AreEqual(s.deviation(), 1.0);
    }

    TEST_METHOD(stats_9_int) {
        auto s = SampleStatistics<int>{};
        s.put(2);
        s.put(4);
        s.put(4);
        s.put(4);
        s.put(5);
        s.put(5);
        s.put(5);
        s.put(7);
        s.put(9);
        Assert::AreEqual(s.mean(), 5.0);
        Assert::AreEqual(s.variance(), 4.0);
        Assert::AreEqual(s.deviation(), 2.0);
    }

    TEST_METHOD(stats_2_double) {
        auto s = SampleStatistics<double>{};
        s.put(1.0);
        s.put(2.0);
        Assert::AreEqual(s.mean(), 1.5);
        Assert::AreEqual(s.variance(), 0.5);
        Assert::AreEqual(s.deviation(), std::sqrt(0.5));
    }

    /*
            proptest!{
                #[test]
                fn put_1_u32(x in proptest::num::u32::ANY) {
                    let mut s : SampleStatistics<u32> = Default::default();
                    s.put(x);
                    Assert::IsTrue(s.mean() >= f64::from(s.min()));
                    Assert::IsTrue(s.mean() <= f64::from(s.max()));
                }

                #[test]
                fn put_1_f64(x in proptest::num::f64::NORMAL) {
                    let mut s : SampleStatistics<f64> = Default::default();
                    s.put(x);
                    Assert::IsTrue(s.mean() >= s.min());
                    Assert::IsTrue(s.mean() <= s.max());
                }

                #[test]
                fn put_2_u32(x in proptest::num::u32::ANY, y in proptest::num::u32::ANY) {
                    let mut s : SampleStatistics<u32> = Default::default();
                    s.put(x);
                    s.put(y);
                    Assert::IsTrue(s.mean() >= f64::from(s.min()));
                    Assert::IsTrue(s.mean() <= f64::from(s.max()));
                    Assert::IsTrue(s.variance() >= 0.);
                    Assert::IsTrue(s.deviation() <= f64::from(s.max() - s.min()));
                }

                #[test]
                fn put_2_f64(x in proptest::num::f64::NORMAL, y in proptest::num::f64::NORMAL) {
                    let mut s : SampleStatistics<f64> = Default::default();
                    s.put(x);
                    s.put(y);
                    Assert::IsTrue(s.mean() >= s.min());
                    Assert::IsTrue(s.mean() <= s.max());
                    Assert::IsTrue(s.variance() >= 0.);
                    Assert::IsTrue(s.deviation() <= (s.max() - s.min()) * 1.5);
                }

                #[test]
                fn put_n_u32(i in 2..99, x in proptest::num::u32::ANY) {
                    let mut s : SampleStatistics<u32> = Default::default();
                    for _ in 0..i {
                        s.put(x);
                    }
                    Assert::IsTrue(s.mean() >= f64::from(s.min()));
                    Assert::IsTrue(s.mean() <= f64::from(s.max()));
                    Assert::IsTrue(s.variance() >= 0.);
                    Assert::IsTrue(s.deviation() <= f64::from(s.max() - s.min()));
                }

                #[test]
                fn put_n_f64(i in 2..99, x in proptest::num::f64::NORMAL) {
                    let mut s : SampleStatistics<f64> = Default::default();
                    for _ in 0..i {
                        s.put(x);
                    }
                    Assert::IsTrue(s.mean() >= s.min());
                    Assert::IsTrue(s.mean() <= s.max());
                    Assert::IsTrue(s.variance() >= 0.);
                    Assert::IsTrue(s.deviation() <= (s.max() - s.min()));
                }
            }
            */
    };
}
