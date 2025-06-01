const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    for (texts) |text| {
        try stdout.print("{s}\n", .{text});
    }
    try stdout.print("Still alive.\n", .{});
}

const texts = &[_][]const u8{ "tar", "flow" };
