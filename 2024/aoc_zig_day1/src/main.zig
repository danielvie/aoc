const std = @import("std");

pub const FileChoice = enum { TEST, RUN };

fn readFileValues(allocator: std.mem.Allocator, file_path: []const u8) !struct { left_values: std.ArrayList(i32), right_values: std.ArrayList(i32) } {
    // Open the file
    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    // Create a buffered reader
    var buffered_reader = std.io.bufferedReader(file.reader());
    var reader = buffered_reader.reader();

    // Slice to store left and right values
    var left_values = std.ArrayList(i32).init(allocator);
    var right_values = std.ArrayList(i32).init(allocator);

    // Buffer for reading lines
    var buf: [200]u8 = undefined;

    // Read file line by line
    while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        // Split the line by whitespace
        var it = std.mem.tokenize(u8, line, " \t\r\n");

        // Parse left value
        if (it.next()) |left_str| {
            const left = try std.fmt.parseInt(i32, left_str, 10);
            try left_values.append(left);

            // Parse right value
            if (it.next()) |right_str| {
                const right = try std.fmt.parseInt(i32, right_str, 10);
                try right_values.append(right);
            }
        }
    }

    return .{
        .left_values = left_values,
        .right_values = right_values,
    };
}

fn part1(left_values: *std.ArrayList(i32), right_values: *std.ArrayList(i32)) u32 {
    var total: u32 = 0;
    for (left_values.items, right_values.items) |left, right| {
        const distance = @abs(left - right);
        total += distance;
    }

    return total;
}

fn part2(left_values: *std.ArrayList(i32), right_values: *std.ArrayList(i32)) u32 {
    // compute similarity score
    var left_: i32 = -1;
    var count_: u32 = 0;
    var similarity: u32 = 0;

    for (left_values.items) |left| {
        // if the left is the same as before, we dont need to search again
        if (left_ == left) {
            similarity += @abs(left) * count_;
            continue;
        }

        var count: u32 = 0;
        for (right_values.items) |right| {
            // if the `right` is bigger than left, left will not be finded anymore
            if (right > left) {
                continue;
            }

            if (left == right) {
                count += 1;
            }
        }
        similarity += @abs(left) * count;

        left_ = left;
        count_ = count;
    }

    return similarity;
}

fn run(choice: FileChoice) !struct { distance: u32, similarity: u32 } {
    // Open the file
    const filename = switch (choice) {
        .TEST => "ref.txt",
        .RUN => "list.txt",
    };

    // Allocator for dynamic memory
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    // reading the txt file
    const result = try readFileValues(allocator, filename);
    defer result.left_values.deinit();
    defer result.right_values.deinit();

    // sorting arrays
    var sorted_left_values = try result.left_values.clone();
    defer sorted_left_values.deinit();
    std.mem.sort(i32, sorted_left_values.items, {}, std.sort.asc(i32));

    var sorted_right_values = try result.right_values.clone();
    defer sorted_right_values.deinit();
    std.mem.sort(i32, sorted_right_values.items, {}, std.sort.asc(i32));

    // running exercices
    const distance: u32 = part1(&sorted_left_values, &sorted_right_values);
    const similarity: u32 = part2(&sorted_left_values, &sorted_right_values);

    return .{
        .distance = distance,
        .similarity = similarity,
    };
}

pub fn main() !void {
    const result_test = try run(FileChoice.TEST);
    std.debug.print("test:\n", .{});
    std.debug.print("distance: {}\n", .{result_test.distance});
    std.debug.print("similarity: {}\n", .{result_test.similarity});

    const result_run = try run(FileChoice.TEST);
    std.debug.print("\nlist:\n", .{});
    std.debug.print("distance: {}\n", .{result_run.distance});
    std.debug.print("similarity: {}\n", .{result_run.similarity});
}
