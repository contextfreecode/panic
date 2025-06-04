const recover = @import("./recover.zig");
const std = @import("std");

pub const panic = recover.panic;

pub fn main() void {
    const allocator = std.heap.page_allocator;
    run(allocator);
    std.debug.print("Still alive.\n", .{});
}

fn run(allocator: std.mem.Allocator) void {
    recover.call(struct {
        pub fn loop(alloc: std.mem.Allocator) void {
            for (1..4) |id| {
                const text = processText(alloc, id);
                defer alloc.free(text);
                std.debug.print("{s}\n", .{text});
            }
        }
    }.loop, .{allocator}) catch |err| {
        std.debug.print("error: {}\n", .{err});
    };
}

fn processText(allocator: std.mem.Allocator, id: usize) []const u8 {
    const text = retrieveText(id) catch @panic("retrieveText failed");
    const codes = stringToCodes(allocator, text) catch @panic("stringToCodes failed");
    defer allocator.free(codes);
    std.mem.reverse(u21, codes);
    return codesToString(allocator, codes) catch @panic("codesToString failed");
}

const texts = [_][]const u8{ "tar", "flow" };
// const texts = [_][]const u8{ "tar", "flow", "🐻‍❄️❤️🦭" };

fn retrieveText(id: usize) error{NotFound}![]const u8 {
    if (id == 0 or id > texts.len) return error.NotFound;
    return texts[id - 1];
}

fn codesToString(allocator: std.mem.Allocator, codes: []const u21) ![]const u8 {
    var buffer = std.ArrayList(u8).init(allocator);
    for (codes) |code| {
        var temp: [4]u8 = undefined;
        const len = try std.unicode.utf8Encode(code, &temp);
        try buffer.appendSlice(temp[0..len]);
    }
    return buffer.toOwnedSlice();
}

fn stringToCodes(allocator: std.mem.Allocator, text: []const u8) ![]u21 {
    var list = std.ArrayList(u21).init(allocator);
    var view = try std.unicode.Utf8View.init(text);
    var it = view.iterator();
    while (it.nextCodepoint()) |code| {
        try list.append(code);
    }
    return list.toOwnedSlice();
}
