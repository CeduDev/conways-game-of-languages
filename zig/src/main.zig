const std = @import("std");
const print = std.debug.print;
const sleep = std.time.sleep;
const conway = @import("conway.zig");
const Conway = conway.Conway;
const Cell = conway.Cell;
const Coordinate = conway.Coordinate;

pub fn main() !void {
    const a = @embedFile("data.txt");
    print("{s}", .{a});
    //
    //
    //

    // Open dir
    // var dir: std.fs.Dir = try std.fs.cwd().openDir("data", .{});
    // defer dir.close();

    // // Open file
    // const file = try dir.openFile("data.txt", .{});
    // defer file.close();

    // var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    // defer _ = gpa.deinit();
    // const allocator = gpa.allocator();

    // // const fileData = try allocator.create([]u8);
    // // defer allocator.destroy(fileData);
    // const mb = (1 << 10) << 10;
    // const file_content = try file.readToEndAlloc(allocator, mb);
    // std.debug.print("{s}\n", .{file_content});
    //
    //
    //
    //

    // var content = [_]u8{'A'} ** 64;

    // const bytesRead = try std.fs.File.read(file, &content);

    // std.debug.print("Successfully Read {d} bytes: {s}\n", .{
    //     bytesRead,
    //     content[0..bytesRead], // change this line only
    // });

    // const init_len = 3;
    // const init: [init_len]Coordinate = [init_len]Coordinate{ .{ .x = 1, .y = 0 }, .{ .x = 1, .y = 1 }, .{ .x = 1, .y = 2 } };

    // // Initialize a 3 * 3 array of DEAD Cells
    // const cells_len = 9;
    // var cells: [cells_len]Cell = .{Cell.DEAD} ** cells_len;
    // const ms: u32 = 500;

    // const game = try Conway.init(allocator, 3, 3, &init, "jee", 10, ms, &cells);

    // try game.run();

    // allocator.destroy(game);
}
