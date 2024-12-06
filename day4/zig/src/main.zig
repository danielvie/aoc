const std = @import("std");
const fs = std.fs;
const mem = std.mem;

pub const FileChoice = enum { TEST, PUZZLE };

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
    const n: usize = std.mem.indexOf(u8, puzzle, "\n").? - 1;
    var m: usize = 0;
    for (puzzle) |c| {
        if (c == '\n') {
            m += 1;
        }
    }
    std.debug.print("n, m: {}, {}\n", .{ n, m });

    var buffer = try allocator.alloc(u8, puzzle.len);
    defer allocator.free(buffer);

    var counter: usize = 0;
    for (puzzle) |c| {
        if (c != '\n' and c != '\r') {
            if (counter < buffer.len) {
                buffer[counter] = c;
                counter += 1;
            } else {
                break;
            }
        }
    }

    const slice = buffer[0..counter];

    std.debug.print("puzzle (len:{}): \n{s}", .{ puzzle.len, puzzle });
    std.debug.print("\n\n\n", .{});
    std.debug.print("buffer (len:{}, cont: {}): \n{s}\n", .{ buffer.len, counter, buffer });
    std.debug.print("slice (len:{}): \n{s}\n", .{ slice.len, slice });
}
