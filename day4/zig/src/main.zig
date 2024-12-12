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
    const puzzle = try readPuzzle(allocator, FileChoice.PUZZLE);
    defer allocator.free(puzzle);

    // removing '\n'
    const col: usize = std.mem.indexOf(u8, puzzle, "\n").? - 1;
    const lin = col;
    std.debug.print("lin, col: {}, {}\n", .{ lin, col });

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

    // counting number of 'xmas'
    var total: u32 = 0;
    for (0..lin) |ii| {
        for (0..col) |jj| {
            for (Dir.items) |d| {
                const res = has_xmas(slice, @intCast(col), @intCast(ii), @intCast(jj), d.x, d.y);
                total += @intFromBool(res);
            }
        }
    }
    std.debug.print("total: {}\n", .{total});
}

fn get(puzzle: []u8, col: i32, i: i32, j: i32) u8 {
    // converting idx
    const idx: i32 = i * col + j;

    if (idx >= 0 and idx < puzzle.len) {
        return puzzle[@intCast(idx)];
    } else {
        return '.';
    }
}

fn has_xmas(puzzle: []u8, col: i32, i: i32, j: i32, di: i32, dj: i32) bool {
    const word = "XMAS";

    var k: i32 = 0;
    for (word) |c| {
        const ii = i + k * di;
        const jj = j + k * dj;
        const is_in_range = (ii >= 0 and ii < col) and (jj >= 0 and jj < col);

        // std.debug.print("ii, jj, k, inrange: {}, {}, {}, {}\n", .{ ii, jj, k, is_in_range });
        if (!is_in_range) {
            return false;
        }
        const x = get(puzzle, col, ii, jj);
        // std.debug.print("x: {c}\n", .{x});
        if (x != c) {
            return false;
        }
        k += 1;
    }

    return true;
}
