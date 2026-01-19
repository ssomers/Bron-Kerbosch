const std = @import("std");
const math = std.math;
const testing = std.testing;

pub fn SampleStatistics(comptime T: type) type {
    return struct {
        max: T = 0,
        min: T = 0,
        samples: u32 = 0,
        sum: f64 = 0,
        sum_of_squares: f64 = 0,

        const Self = @This();

        fn widen(val: T) f64 {
            return switch (@typeInfo(T)) {
                .int => @floatFromInt(val),
                .float => val,
                else => unreachable,
            };
        }

        pub fn is_empty(self: Self) bool {
            return self.samples == 0;
        }

        pub fn put(self: *Self, v: T) void {
            if (self.is_empty()) {
                self.min = v;
                self.max = v;
            } else if (self.min > v) {
                self.min = v;
            } else if (self.max < v) {
                self.max = v;
            }
            self.samples += 1;
            const vf = widen(v);
            self.sum += vf;
            self.sum_of_squares += vf * vf;
        }

        pub fn mean(self: Self) f64 {
            if (self.samples < 1) {
                return math.nan(f64);
            } else {
                const n: f64 = @floatFromInt(self.samples);
                return math.clamp(self.sum / n, widen(self.min), widen(self.max));
            }
        }

        pub fn variance(self: Self) f64 {
            if (self.samples < 2) {
                return math.nan(f64);
            } else if (self.min == self.max) {
                return 0;
            } else {
                const n: f64 = @floatFromInt(self.samples);
                const r = (self.sum_of_squares - self.sum * self.sum / n) / (n - 1);
                return @max(r, 0);
            }
        }

        pub fn deviation(self: Self) f64 {
            const r = math.sqrt(self.variance());
            if (math.isNan(r)) {
                return r;
            } else {
                return @min(r, widen(self.max - self.min));
            }
        }
    };
}

test "stats_0_i32" {
    const s = SampleStatistics(i32){};
    try testing.expect(math.isNan(s.mean()));
    try testing.expect(math.isNan(s.variance()));
    try testing.expect(math.isNan(s.deviation()));
}

test "stats_1_i32" {
    var s = SampleStatistics(i32){};
    s.put(-1);
    try testing.expectEqual(s.mean(), -1.0);
    try testing.expect(math.isNan(s.variance()));
    try testing.expect(math.isNan(s.deviation()));
}

test "stats_2_i32" {
    var s = SampleStatistics(i32){};
    s.put(-1);
    s.put(1);
    try testing.expectEqual(s.mean(), 0.0);
    try testing.expectEqual(s.variance(), 2.0);
    try testing.expectEqual(s.deviation(), math.sqrt(2.0));
}

test "stats_3_i32" {
    var s = SampleStatistics(i32){};
    s.put(89);
    s.put(90);
    s.put(91);
    try testing.expectEqual(s.mean(), 90.0);
    try testing.expectEqual(s.variance(), 1.0);
    try testing.expectEqual(s.deviation(), 1.0);
}

test "stats_9_u32" {
    var s = SampleStatistics(u32){};
    s.put(2);
    s.put(4);
    s.put(4);
    s.put(4);
    s.put(5);
    s.put(5);
    s.put(5);
    s.put(7);
    s.put(9);
    try testing.expectEqual(s.mean(), 5.0);
    try testing.expectEqual(s.variance(), 4.0);
    try testing.expectEqual(s.deviation(), 2.0);
}

test "stats_2_f64" {
    var s = SampleStatistics(f64){};
    s.put(1.0);
    s.put(2.0);
    try testing.expectEqual(s.mean(), 1.5);
    try testing.expectEqual(s.variance(), 0.5);
    try testing.expectEqual(s.deviation(), math.sqrt(0.5));
}
