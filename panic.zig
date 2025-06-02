const std = @import("std");

pub fn main() void {
    const allocator = std.heap.page_allocator;
    run(allocator);
    std.debug.print("Still alive.\n", .{});
}

fn run(allocator: std.mem.Allocator) void {
    // For panic recovery, if we want to be abusive, see:
    // https://github.com/dimdin/zig-recover/blob/main/src/recover.zig
    runLoop(allocator) catch |err| {
        std.debug.print("Error: {}\n", .{err});
    };
}

fn runLoop(allocator: std.mem.Allocator) !void {
    var i: usize = 1;
    while (i <= 3) : (i += 1) {
        const text = try processText(allocator, i);
        defer allocator.free(text);
        std.debug.print("{s}\n", .{text});
    }
}

fn processText(allocator: std.mem.Allocator, id: usize) ![]const u8 {
    const text = try retrieveText(id);
    const codes = try stringToCodes(allocator, text);
    defer allocator.free(codes);
    std.mem.reverse(u21, codes);
    return try codesToString(allocator, codes);
}

const texts = [_][]const u8{ "tar", "flow" };
// const texts = [_][]const u8{ "tar", "flow", "🐻‍❄️❤️🦭" };

fn retrieveText(id: usize) ![]const u8 {
    if (id == 0 or id > texts.len) return error.NotFound;
    return texts[id - 1];
}

fn codesToString(allocator: std.mem.Allocator, codes: []const u21) ![]const u8 {
    var buffer = std.ArrayList(u8).init(allocator);
    for (codes) |cp| {
        var temp: [4]u8 = undefined;
        const len = try std.unicode.utf8Encode(cp, &temp);
        try buffer.appendSlice(temp[0..len]);
    }
    return buffer.toOwnedSlice();
}

fn stringToCodes(allocator: std.mem.Allocator, text: []const u8) ![]u21 {
    var list = std.ArrayList(u21).init(allocator);
    var view = try std.unicode.Utf8View.init(text);
    var it = view.iterator();
    while (it.nextCodepoint()) |cp| {
        try list.append(cp);
    }
    return list.toOwnedSlice();
}
