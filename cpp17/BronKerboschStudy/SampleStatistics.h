#pragma once

namespace BronKerboschStudy {
    template <typename T>
    class SampleStatistics {
    private:
        T maxOrZero = 0;
        T minOrZero = 0;
        unsigned samples = 0;
        double sum = 0;
        double sum_of_squares = 0;

    public:
        void put(T v) {
            if (samples == 0) {
                minOrZero = v;
                maxOrZero = v;
            } else if (minOrZero > v) {
                minOrZero = v;
            } else if (maxOrZero < v) {
                maxOrZero = v;
            }
            samples += 1;
            sum += v;
            sum_of_squares += std::pow(v, 2);
        }

        T max() const {
            return maxOrZero;
        }
        T min() const {
            return minOrZero;
        }

        double mean() const {
            static_assert(std::numeric_limits<double>::has_quiet_NaN);
            if (samples < 1) {
                return std::numeric_limits<double>::quiet_NaN();
            } else {
                auto r = sum / double(samples);
                return std::max(double(minOrZero), std::min(double(maxOrZero), r));
            }
        }

        double variance() const {
            static_assert(std::numeric_limits<double>::has_quiet_NaN);
            if (samples < 2) {
                return std::numeric_limits<double>::quiet_NaN();
            } else if (minOrZero == maxOrZero) {
                return 0.;
            } else {
                auto n = double(samples);
                auto r = (sum_of_squares - std::pow(sum, 2) / n) / (n - 1.);
                return std::max(0., r);
            }
        }

        double deviation() const {
            static_assert(std::numeric_limits<double>::has_quiet_NaN);
            auto r = std::sqrt(variance());
            if (std::isnan(r)) {
                return r;
            } else {
                return std::min(double(maxOrZero - minOrZero), r);
            }
        }
    };
}
