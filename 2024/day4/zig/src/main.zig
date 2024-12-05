const std = @import("std");
const fs = std.fs;
const mem = std.mem;

pub fn main() !void {
    // Open the file
    const file = try fs.cwd().openFile("../data/puzzle.txt", .{});
    defer file.close();

    // Read the entire file content
    const content = try file.readToEndAlloc(std.heap.page_allocator, 1024 * 1024);
    defer std.heap.page_allocator.free(content);

    // Split into lines and trim whitespace
    var lines = std.ArrayList([]const u8).init(std.heap.page_allocator);
    defer lines.deinit();

    var line_iter = std.mem.tokenize(u8, content, "\n");
    while (line_iter.next()) |line| {
        try lines.append(line);
    }

    const n = lines.items.len;
    const m = lines.items[0].len;

    // Generate directions
    const Coord = struct { dx: i32, dy: i32 };
    var dd = std.ArrayList(Coord).init(std.heap.page_allocator);
    defer dd.deinit();

    var dx: i32 = -1;
    while (dx <= 1) : (dx += 1) {
        var dy: i32 = -1;
        while (dy <= 1) : (dy += 1) {
            if (dx != 0 or dy != 0) {
                try dd.append(.{ .dx = dx, .dy = dy });
            }
        }
    }

    // Function to check for XMAS pattern
    const hasXmas = struct {
        pub fn check(lines_list: []const []const u8, i: usize, j: usize, d: Coord) bool {
            const xmas = "XMAS";

            for (0..4) |k| {
                const ii = @as(i32, @intCast(i)) + @as(i32, @intCast(k)) * d.dx;
                const jj = @as(i32, @intCast(j)) + @as(i32, @intCast(k)) * d.dy;

                if (ii < 0 or ii >= @as(i32, @intCast(lines_list.len)) or
                    jj < 0 or jj >= @as(i32, @intCast(lines_list[0].len)))
                {
                    return false;
                }

                // Use .get() method to safely index the string
                if (lines_list[@as(usize, @intCast(ii))][@as(usize, @intCast(jj))] != xmas[k]) {
                    return false;
                }
            }
            return true;
        }
    }.check;

    // Count XMAS patterns
    var total: usize = 0;
    for (0..n) |i| {
        for (0..m) |j| {
            for (dd.items) |d| {
                if (hasXmas(lines.items, i, j, d)) {
                    total += 1;
                }
            }
        }
    }

    // Print total
    const stdout = std.io.getStdOut().writer();
    try stdout.print("total: {}\n", .{total});
}
