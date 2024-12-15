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

fn get_slice(allocator: std.mem.Allocator, puzzle: []const u8) ![]u8 {
    // allocating buffer
    var buffer = try allocator.alloc(u8, puzzle.len);

    // writing flat string
    var counter: usize = 0;
    for (puzzle) |c| {
        if (std.ascii.isAlphabetic(c)) {
            if (counter < puzzle.len) {
                buffer[counter] = c;
                counter += 1;
            } else {
                break;
            }
        }
    }

    buffer = try allocator.realloc(buffer, counter);
    return buffer;
}

fn get_directions(allocator: std.mem.Allocator) !std.ArrayList(Pair) {
    var directions = std.ArrayList(Pair).init(allocator);

    var i: i32 = -1;
    while (i <= 1) : (i += 1) {
        var j: i32 = -1;
        while (j <= 1) : (j += 1) {
            try directions.append(.{ .x = i, .y = j });
        }
    }

    return directions;
}

pub fn main() !void {

    // create a general-purpose allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // open the file
    const puzzle = try readPuzzle(allocator, FileChoice.TEST);
    defer allocator.free(puzzle);

    // getting number of columns
    const col: usize = std.mem.indexOf(u8, puzzle, "\n").? - 1;

    // getting number of lines
    var lin: usize = 0;
    for (puzzle) |c| {
        if (c == '\n') {
            lin += 1;
        }
    }
    std.debug.print("lin, col: {}, {}\n", .{ lin, col });

    // getting flat slice of the puzzle
    const slice = try get_slice(allocator, puzzle);
    defer allocator.free(slice);

    // create a dynamic array with the possible directions
    const directions = try get_directions(allocator);
    defer directions.deinit();

    // PART 1
    var total: u32 = 0;
    for (0..lin) |i| {
        for (0..col) |j| {
            for (directions.items) |d| {
                const res = has_xmas(slice, @intCast(col), @intCast(i), @intCast(j), d.x, d.y);
                total += @intFromBool(res);
            }
        }
    }
    std.debug.print("part1: {}\n", .{total});

    // PART 2
    total = 0;
    for (0..lin) |i| {
        for (0..col) |j| {
            const res = has_x_mas(slice, @intCast(col), @intCast(i), @intCast(j));
            total += @intFromBool(res);
        }
    }
    std.debug.print("part2: {}\n", .{total});
}

fn get(puzzle: []const u8, col: i32, i: i32, j: i32) u8 {
    // converting idx
    const idx: i32 = i * col + j;

    if (idx >= 0 and idx < puzzle.len) {
        return puzzle[@intCast(idx)];
    } else {
        return '.';
    }
}

fn has_x_mas(puzzle: []u8, n: i32, i: i32, j: i32) bool {
    // return if out of range if center not in range
    const is_in_range = (1 <= i and i < n - 1) and (1 <= j and j < n - 1);
    if (!is_in_range) {
        return false;
    }

    // return if not the center of the MAS
    if (get(puzzle, n, i, j) != 'A') {
        return false;
    }

    // reading the diagonals
    const a = [_]u8{ get(puzzle, n, i - 1, j - 1), get(puzzle, n, i + 1, j + 1) };
    const b = [_]u8{ get(puzzle, n, i - 1, j + 1), get(puzzle, n, i + 1, j - 1) };

    // testing if diagonals are valid (conditions)
    const ca = std.mem.eql(u8, &a, "MS") or std.mem.eql(u8, &a, "SM");
    const cb = std.mem.eql(u8, &b, "MS") or std.mem.eql(u8, &b, "SM");
    const res = ca and cb;

    return res;
}

fn has_xmas(puzzle: []u8, col: i32, i: i32, j: i32, di: i32, dj: i32) bool {
    const word = "XMAS";

    var k: i32 = 0;
    for (word) |c| {
        const ii = i + k * di;
        const jj = j + k * dj;
        const is_in_range = (ii >= 0 and ii < col) and (jj >= 0 and jj < col);

        if (!is_in_range) {
            return false;
        }

        if (get(puzzle, col, ii, jj) != c) {
            return false;
        }
        k += 1;
    }

    return true;
}
