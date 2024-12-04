const std = @import("std");

pub const FileChoice = enum { TEST, TEST2, PUZZLE };
pub const OperationMode = enum { PART1, PART2 };
pub const State = enum { DO, DONT };

pub fn readFile(allocator: std.mem.Allocator, file_path: []const u8) ![]u8 {
    // open the file
    const file = try std.fs.cwd().openFile(file_path, .{});
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
    const txt = try readFile(allocator, filename);
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

pub fn main() !void {
    const result_1 = try run(FileChoice.TEST, OperationMode.PART1);
    std.debug.print("\n", .{});
    std.debug.print("test part 1: {}\n", .{result_1});

    const result_2 = try run(FileChoice.TEST2, OperationMode.PART2);
    std.debug.print("\n", .{});
    std.debug.print("test part 2: {}\n", .{result_2});

    const result_3 = try run(FileChoice.PUZZLE, OperationMode.PART1);
    std.debug.print("\n", .{});
    std.debug.print("puzzle part 1: {}\n", .{result_3});

    const result_4 = try run(FileChoice.PUZZLE, OperationMode.PART2);
    std.debug.print("\n", .{});
    std.debug.print("puzzle part 2: {}\n", .{result_4});
}
