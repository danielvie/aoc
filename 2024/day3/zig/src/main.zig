const std = @import("std");

pub const FileChoice = enum { TEST, PUZZLE };
pub const Options = enum { PART1, IMPLEMENT_DONT };

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

// fn part1(allocator: std.mem.Allocator, txt: []const u8) !std.ArrayList(struct { x: i32, y: i32 }) {
fn part1(allocator: std.mem.Allocator, txt: []const u8) !u32 {
    var results = std.ArrayList(Pair).init(allocator);
    defer results.deinit();

    // Find all matches
    var i: usize = 0;

    while (i < txt.len) : (i += 1) {
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

    // compute multiplication
    var mul: u32 = 0;
    for (results.items) |pair| {
        mul += pair.x * pair.y;
    }

    return mul;
}

fn run(choice: FileChoice) !struct { a: u32, b: u32 } {
    // open the file
    const filename = switch (choice) {
        .TEST => "../data/test.txt",
        .PUZZLE => "../data/puzzle.txt",
    };

    // create a general-purpose allocator
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // read file
    const txt = try readFile(allocator, filename);
    defer allocator.free(txt);

    // part1
    const a = try part1(allocator, txt);

    return .{
        .a = a,
        .b = 0,
    };
}

pub fn main() !void {
    const result_1 = try run(FileChoice.TEST);
    std.debug.print("\n", .{});
    std.debug.print("test:\n", .{});
    std.debug.print("a: {}\n", .{result_1.a});
    std.debug.print("b: {}\n", .{result_1.b});

    const result_2 = try run(FileChoice.PUZZLE);
    std.debug.print("\n", .{});
    std.debug.print("test:\n", .{});
    std.debug.print("a: {}\n", .{result_2.a});
    std.debug.print("b: {}\n", .{result_2.b});
}
