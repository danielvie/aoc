const std = @import("std");

pub const FileChoice = enum { TEST, RUN };

fn readFileValues(allocator: std.mem.Allocator, file_path: []const u8) !std.ArrayList((std.ArrayList(i32))) {
    // Open the file
    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    // Create a buffered reader
    var buffered_reader = std.io.bufferedReader(file.reader());
    var reader = buffered_reader.reader();

    // ArrayList to store sequences
    var sequences = std.ArrayList(std.ArrayList(i32)).init(allocator);

    // Buffer for reading lines
    var buf: [200]u8 = undefined;

    // Read file line by line
    while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        // Create a new sequence for each line
        var current_sequence = std.ArrayList(i32).init(allocator);

        // Split the line by whitespace
        var it = std.mem.tokenize(u8, line, " \t\r\n");

        // Parse and add all numbers in the line
        while (it.next()) |num_str| {
            const num = try std.fmt.parseInt(i32, num_str, 10);
            try current_sequence.append(num);
        }

        // Only add non-empty sequences
        if (current_sequence.items.len > 0) {
            try sequences.append(current_sequence);
        }
    }

    return sequences;
}

fn is_save(level: *const std.ArrayList(i32)) bool {

    // if the length of the array is 1, return false
    if (level.items.len == 1) {
        return false;
    }

    // check increasing
    var is_inc = true;
    for (level.items[0 .. level.items.len - 1], 0..) |val, i| {
        const diff = level.items[i + 1] - val;
        if ((diff < 0 or diff < 1 or diff > 3)) {
            is_inc = false;
            break;
        }
    }

    // check decreasing
    var is_dec = true;
    for (level.items[0 .. level.items.len - 1], 0..) |val, i| {
        const diff = level.items[i + 1] - val;
        if ((diff > 0 or diff < -3 or diff > -1)) {
            is_dec = false;
            break;
        }
    }

    // Return true if either increasing or decreasing sequence is found
    return is_inc or is_dec;
}

fn is_save_damped(level: *const std.ArrayList(i32)) bool {
    // if all elements is save, then it is true
    if (is_save(level)) {
        return true;
    }

    // else test with one less element
    for (0..level.items.len) |i| {
        // create a copy of the list
        var level_ = level.clone() catch return false;
        defer level_.deinit();
        _ = level_.orderedRemove(i);

        // if any condition is save, then it is true
        if (is_save(&level_)) {
            return true;
        }
    }

    // if no condition were met to be safe, then it is 'false'
    return false;
}

fn part1(levels: *const std.ArrayList(std.ArrayList(i32))) u32 {
    var count: u32 = 0;

    for (levels.items) |level| {
        count += if (is_save(&level)) 1 else 0;
    }

    return count;
}

fn part2(levels: *const std.ArrayList(std.ArrayList(i32))) u32 {
    var count: u32 = 0;

    for (levels.items) |level| {
        count += if (is_save_damped(&level)) 1 else 0;
        // std.debug.print("numel: {}\n", .{level.items.len});
    }

    return count;
}

fn run(choice: FileChoice) !struct { a: u32, b: u32 } {
    // Open the file
    const filename = switch (choice) {
        .TEST => "./data/test.txt",
        .RUN => "./data/list.txt",
    };

    // Allocator for dynamic memory
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // reading the txt file
    const levels = try readFileValues(allocator, filename);
    defer {
        for (levels.items) |lev| {
            lev.deinit();
        }
        levels.deinit();
    }

    // Part1
    const numel_is_safe = part1(&levels);
    const numel_is_safe_damped = part2(&levels);

    return .{
        .a = numel_is_safe,
        .b = numel_is_safe_damped,
    };
}

pub fn main() !void {
    const result_1 = try run(FileChoice.TEST);
    std.debug.print("\n", .{});
    std.debug.print("test:\n", .{});
    std.debug.print("a: {}\n", .{result_1.a});
    std.debug.print("b: {}\n", .{result_1.b});

    const start_time = std.time.nanoTimestamp();
    const result_2 = try run(FileChoice.RUN);
    const end_time = std.time.nanoTimestamp();
    const duration = end_time - start_time;

    std.debug.print("\n", .{});
    std.debug.print("run:\n", .{});
    std.debug.print("a: {}\n", .{result_2.a});
    std.debug.print("b: {}\n", .{result_2.b});

    std.debug.print("duration: {d:.3} ms", .{@as(f32, @floatFromInt(duration)) / 1_000_000.0});
}
