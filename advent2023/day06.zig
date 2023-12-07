const std = @import("std");
const print = std.debug.print;

const Race = struct {
    time: i64,
    distance: i64,
};

// Test input
// Time:      7  15   30
// Distance:  9  40  200
const test_races = [_]Race{
    Race{ .time = 7, .distance = 9 },
    Race{ .time = 15, .distance = 40 },
    Race{ .time = 30, .distance = 200 },
};
const test_race = Race{
    .time = 71530,
    .distance = 940200,
};

// Full input
// Time:        49     78     79     80
// Distance:   298   1185   1066   1181
const full_races = [_]Race{
    Race{ .time = 49, .distance = 298 },
    Race{ .time = 78, .distance = 1185 },
    Race{ .time = 79, .distance = 1066 },
    Race{ .time = 80, .distance = 1181 },
};
const full_race = Race{
    .time = 49787980,
    .distance = 298118510661181,
};

fn solve(r: Race) i64 {
    const a2 = r.time * r.time - 4 * r.distance;
    const a: f64 = std.math.sqrt(@as(f64, @floatFromInt(a2)));
    const t: f64 = @floatFromInt(r.time);
    const lb = (t - a) / 2.0 + 1e-9;
    const ub = (t + a) / 2.0 - 1e-9;
    const delta = std.math.floor(ub) - std.math.ceil(lb);
    return @as(i64, @intFromFloat(delta)) + 1;
}

pub fn main() void {
    var part1: i64 = 1;
    for (full_races) |r| {
        part1 *= solve(r);
    }
    print("Part 1: {}\n", .{part1});
    print("Part 2: {}\n", .{solve(full_race)});
}
