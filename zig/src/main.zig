const std = @import("std");
const print = std.debug.print;
const sleep = std.time.sleep;
const conway = @import("conway.zig");
const Conway = conway.Conway;
const Cell = conway.Cell;
const Coordinate = conway.Coordinate;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();

    const init_len: comptime_int = 3;
    const init: [init_len]Coordinate = [init_len]Coordinate{ .{ .x = 1, .y = 0 }, .{ .x = 1, .y = 1 }, .{ .x = 1, .y = 2 } };

    // Initialize a 3 * 3 array of DEAD Cells
    const cells_len: comptime_int = 9;
    var cells: [cells_len]Cell = .{Cell.DEAD} ** cells_len;
    const ms: u32 = 500;

    const game = try Conway.init(allocator, 3, 3, &init, "jee", 10, ms, &cells);

    try game.run();

    allocator.destroy(game);
}
