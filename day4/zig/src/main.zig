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

    // // create a dynamic array
    // var Dir = std.ArrayList(Pair).init(allocator);
    // defer Dir.deinit();

    // // populating values
    // var i: i32 = -1;
    // while (i <= 1) : (i += 1) {
    //     var j: i32 = -1;
    //     while (j <= 1) : (j += 1) {
    //         try Dir.append(.{ .x = i, .y = j });
    //     }
    // }

    // checking if is valid
    std.debug.print("source: \n{s}\n", .{puzzle});

    std.debug.print("lin, col: {}, {}; slice.len: {}\n", .{ lin, col, slice.len });
    var res: usize = 0;
    for (1..lin + 1) |ii| {
        for (1..col + 1) |jj| {
            res += check_xmas(slice, lin, col, ii, jj);
        }
    }

    std.debug.print("res: {}", .{res});
}

fn check_xmas(puzzle: []u8, lin: usize, col: usize, i: usize, j: usize) usize {
    const idx = (i - 1) * lin + (j - 1);

    // check right
    const is_xmas_r = if (j + 3 < col) puzzle[idx] == 'X' and
        puzzle[idx + 1] == 'M' and
        puzzle[idx + 2] == 'A' and
        puzzle[idx + 3] == 'S' else false;

    // check down
    const is_xmas_d = if (i + 3 < lin) puzzle[idx] == 'X' and
        puzzle[idx + lin] == 'M' and
        puzzle[idx + lin * 2] == 'A' and
        puzzle[idx + lin * 3] == 'S' else false;

    // check up
    const is_xmas_u = if (i > 3) puzzle[idx] == 'X' and
        puzzle[idx - lin] == 'M' and
        puzzle[idx - lin * 2] == 'A' and
        puzzle[idx - lin * 3] == 'S' else false;

    // check left
    const is_xmas_l = if (j > 3) puzzle[idx] == 'X' and
        puzzle[idx - 1] == 'M' and
        puzzle[idx - 2] == 'A' and
        puzzle[idx - 3] == 'S' else false;

    // check right down
    const is_xmas_rd = if (i + 3 < lin and j + 3 < col) puzzle[idx] == 'X' and
        puzzle[idx + lin * 1 + 1] == 'M' and
        puzzle[idx + lin * 2 + 2] == 'A' and
        puzzle[idx + lin * 3 + 3] == 'S' else false;

    // check right up
    const is_xmas_ru = if (i > 3 and j + 3 < col) puzzle[idx] == 'X' and
        puzzle[idx - lin * 1 + 1] == 'M' and
        puzzle[idx - lin * 2 + 2] == 'A' and
        puzzle[idx - lin * 3 + 3] == 'S' else false;

    // check left up
    const is_xmas_lu = if (i > 3 and j > 3) puzzle[idx] == 'X' and
        puzzle[idx - lin * 1 - 1] == 'M' and
        puzzle[idx - lin * 2 - 2] == 'A' and
        puzzle[idx - lin * 3 - 3] == 'S' else false;

    // check left down
    const is_xmas_ld = if (i + 3 < col and j > 3) puzzle[idx] == 'X' and
        puzzle[idx + lin * 1 - 1] == 'M' and
        puzzle[idx + lin * 2 - 2] == 'A' and
        puzzle[idx + lin * 3 - 3] == 'S' else false;

    var res: usize = 0;
    res += @intFromBool(is_xmas_r);
    res += @intFromBool(is_xmas_l);
    res += @intFromBool(is_xmas_u);
    res += @intFromBool(is_xmas_d);
    res += @intFromBool(is_xmas_ru);
    res += @intFromBool(is_xmas_rd);
    res += @intFromBool(is_xmas_lu);
    res += @intFromBool(is_xmas_ld);

    return res;
}
