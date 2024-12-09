const std = @import("std");
const fs = std.fs;
const mem = std.mem;

pub const FileChoice = enum { TEST, PUZZLE };

const Pair = struct {
    x: i32,
    y: i32,
};

pub fn readPuzzle(allocator: std.mem.Allocator, choice: FileChoice) ![]u8 {
    // get addr
    const filename = switch (choice) {
        .TEST => "../data/test.txt",
        .PUZZLE => "../data/puzzle.txt",
    };

    // open the file
    const file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    // read the entire file txt
    return try file.readToEndAlloc(allocator, std.math.maxInt(usize));
}

pub fn main() !void {

    // create a general-purpose allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // open the file
    const puzzle = try readPuzzle(allocator, FileChoice.TEST);
    defer allocator.free(puzzle);

    // removing '\n'
    const m: usize = std.mem.indexOf(u8, puzzle, "\n").? - 1;
    var n: usize = 0;
    for (puzzle) |c| {
        if (c == '\n') {
            n += 1;
        }
    }
    std.debug.print("n, m: {}, {}\n", .{ m, n });

    var buffer = try allocator.alloc(u8, puzzle.len);
    defer allocator.free(buffer);

    var counter: usize = 0;
    for (puzzle) |c| {
        if (std.ascii.isAlphabetic(c)) {
            if (counter < buffer.len) {
                buffer[counter] = c;
                counter += 1;
            } else {
                break;
            }
        }
    }

    const slice = buffer[0..counter];

    std.debug.print("\npuzzle (len:{}): \n{s}", .{ puzzle.len, puzzle });

    std.debug.print("\n", .{});
    std.debug.print("puzzle(i,j): {c}\n", .{get(slice, m, n, 7, 8)});

    std.debug.print("\n\n\n", .{});
    std.debug.print("buffer (len:{}, cont: {}): \n{s}\n", .{ buffer.len, counter, buffer });
    std.debug.print("slice (len:{}): \n{s}\n", .{ slice.len, slice });

    // create a dynamic array
    var Dir = std.ArrayList(Pair).init(allocator);
    defer Dir.deinit();

    // populating values
    var i: i32 = -1;
    while (i <= 1) : (i += 1) {
        var j: i32 = -1;
        while (j <= 1) : (j += 1) {
            try Dir.append(.{ .x = i, .y = j });
        }
    }

    // printing result
    for (Dir.items) |d| {
        std.debug.print("{}\n", .{d});
    }
}

fn get(puzzle: []u8, n: usize, m: usize, i: usize, j: usize) u8 {
    const idx = (i - 1) * n + (j - 1);
    std.debug.print("n, m: {}, {}; i, j: {}, {}, idx: {}\n", .{ n, m, i, j, idx });
    const c = puzzle[idx];
    std.debug.print("c: {c}\n", .{c});
    return c;
}
