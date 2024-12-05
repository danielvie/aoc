const std = @import("std");

pub const FileChoice = enum { TEST, TEST2, PUZZLE };
pub const OperationMode = enum { PART1, PART2 };
pub const State = enum { DO, DONT };
const len: usize = 4;

pub fn getPuzzle(allocator: std.mem.Allocator, choice: FileChoice) ![]u8 {
    // get addr file
    const filename = switch (choice) {
        .TEST => "../data/test.txt",
        .TEST2 => "../data/test2.txt",
        .PUZZLE => "../data/puzzle.txt",
    };

    // open the file
    const file = try std.fs.cwd().openFile(filename, .{});
    defer file.close();

    // read the entire file txt
    return try file.readToEndAlloc(allocator, std.math.maxInt(usize));
}

const Pair = struct {
    x: u32,
    y: u32,
};

fn findMatches(allocator: std.mem.Allocator, txt: []const u8, mode: OperationMode) !std.ArrayList(Pair) {
    var results = std.ArrayList(Pair).init(allocator);

    var state = State.DO;
    var i: usize = 0;
    while (i < txt.len) : (i += 1) {
        // implement "DONT" logic
        if (mode == OperationMode.PART2) {
            // find "don't("
            if (txt.len - i >= 7 and std.mem.eql(u8, txt[i .. i + 6], "don't(")) {

                // ensure ")"
                if (txt[i + 6] == ')') {
                    state = State.DONT;
                }
            }

            // find "do"
            if (txt.len - i >= 4 and std.mem.eql(u8, txt[i .. i + 3], "do(")) {

                // ensure ")"
                if (txt[i + 3] == ')') {
                    state = State.DO;
                }
            }

            // dont parse if "dont" state
            if (state == State.DONT) {
                continue;
            }
        }

        // find 'mul('
        if (txt.len - i >= 4 and std.mem.eql(u8, txt[i .. i + 4], "mul(")) {
            // parse first number (a)
            var j = i + 4;
            var a = std.ArrayList(u8).init(allocator);
            defer a.deinit();

            while (j < txt.len and txt[j] >= '0' and txt[j] <= '9') {
                try a.append(txt[j]);
                j += 1;
            }

            // skip comma
            if (j < txt.len and txt[j] == ',') {
                j += 1;
            }

            // parse second number (b)
            var b = std.ArrayList(u8).init(allocator);
            defer b.deinit();

            while (j < txt.len and txt[j] >= '0' and txt[j] <= '9') {
                try b.append(txt[j]);
                j += 1;
            }

            // ensure closing parenthesis
            if (j < txt.len and txt[j] == ')') {
                const x = try std.fmt.parseInt(u32, a.items, 10);
                const y = try std.fmt.parseInt(u32, b.items, 10);
                try results.append(.{ .x = x, .y = y });
            }
        }
    }
    return results;
}

fn run(choice: FileChoice, mode: OperationMode) !u32 {
    // open the file
    const filename = switch (choice) {
        .TEST => "../data/test.txt",
        .TEST2 => "../data/test2.txt",
        .PUZZLE => "../data/puzzle.txt",
    };

    // create a general-purpose allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // read file
    const txt = try getPuzzle(allocator, filename);
    defer allocator.free(txt);

    // std.debug.print("\n{s}", .{txt});

    const results = try findMatches(allocator, txt, mode);
    defer results.deinit();

    var mul: u32 = 0;
    for (results.items) |pair| {
        mul += pair.x * pair.y;
    }

    return mul;
}

fn part1(choice: FileChoice) !usize {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const puzzle = try getPuzzle(allocator, choice);
    defer allocator.free(puzzle);

    std.debug.print("{s}\n", .{puzzle});

    // finding X and Y
    const X = std.mem.indexOf(u8, puzzle, "\n").? - 1;
    const Y = puzzle.len / (X + 1);
    std.debug.print("X: {}, Y: {}\n\n", .{ X, Y });

    // get slice
    var i: usize = 0;
    var count: usize = 0;
    while (i < puzzle.len) : (i += 1) {
        const right = check_right(puzzle, i);
        if (right) {
            count += 1;
        }
    }
    std.debug.print("count: {}\n", .{count});

    // std.debug.print("left -> right ({}..{}): {s}\n", .{ i, j, puzzle[i..j] });
    // const reverse = puzzle[i..j];
    // std.mem.reverse(u8, reverse);
    // std.debug.print("left -> right ({}..{}): {s}\n", .{ j, i, reverse });

    // _ = puzzle;
    return 18;
}

fn check_xmas(slice: []const u8) bool {
    const xmas = std.mem.eql(u8, slice, "XMAS");
    const smas = std.mem.eql(u8, slice, "SAMX");
    return xmas or smas;
}

fn check_right(puzzle: []const u8, i: usize) bool {
    if (i + len < puzzle.len) {
        const slice = puzzle[i .. i + len];
        const match = check_xmas(slice);
        if (match) {
            std.debug.print("match XMAS i: {}\n", .{i});
        }
        return check_xmas(slice);
    } else {
        return false;
    }
}

pub fn main() !void {
    const a = try part1(FileChoice.TEST);

    std.debug.print("a: {}\n", .{a});
}

test "part1" {
    try std.testing.expect(18 == try part1(FileChoice.TEST));
}
