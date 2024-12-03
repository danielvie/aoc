const std = @import("std");

pub const FileChoice = enum { TEST, PUZZLE };

pub fn readFile(allocator: std.mem.Allocator, file_path: []const u8) ![]u8 {
    // open the file
    const file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    // read the entire file content
    return try file.readToEndAlloc(allocator, std.math.maxInt(usize));
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
    const content = readFile(allocator, filename) catch |err| {
        std.debug.print("Error reading file: {}\n", .{err});
        return err;
    };
    defer allocator.free(content);

    // print file contents
    std.debug.print("File contents:\n{s}\n", .{content});

    return .{
        .a = 0,
        .b = 0,
    };
}

pub fn main() !void {
    const result_1 = try run(FileChoice.TEST);
    std.debug.print("\n", .{});
    std.debug.print("test:\n", .{});
    std.debug.print("a: {}\n", .{result_1.a});
    std.debug.print("b: {}\n", .{result_1.b});
}
