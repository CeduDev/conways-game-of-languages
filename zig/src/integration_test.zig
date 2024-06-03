const std = @import("std");
const conway = @import("conway.zig");
const Conway = conway.Conway;
const Cell = conway.Cell;
const Coordinate = conway.Coordinate;

test "initializing a Conway structure with a Blinker pattern works" {
    const allocator = std.testing.allocator;
    const init_len: comptime_int = 3;
    const init: [init_len]Coordinate = [init_len]Coordinate{ .{ .x = 1, .y = 0 }, .{ .x = 1, .y = 1 }, .{ .x = 1, .y = 2 } };

    // Initialize a 3 * 3 array of DEAD Cells
    const cells_len: comptime_int = 9;
    var cells: [cells_len]Cell = .{Cell.DEAD} ** cells_len;
    const ms: u32 = 500;

    const game = try Conway.init(allocator, 3, 3, &init, "jee", 10, ms, &cells);
    defer allocator.destroy(game);

    try std.testing.expect(game.cells.len == 9);

    const blinker: [cells_len]Cell = .{
        Cell.DEAD,
        Cell.ALIVE,
        Cell.DEAD,
        Cell.DEAD,
        Cell.ALIVE,
        Cell.DEAD,
        Cell.DEAD,
        Cell.ALIVE,
        Cell.DEAD,
    };

    try std.testing.expectEqualSlices(Cell, &blinker, game.cells);
}

test "initializing a Conway structure with a Blinker pattern works and has the correct pattern after two runs (same as original patterns)" {
    const allocator = std.testing.allocator;
    const init_len: comptime_int = 3;
    const init: [init_len]Coordinate = [init_len]Coordinate{ .{ .x = 1, .y = 0 }, .{ .x = 1, .y = 1 }, .{ .x = 1, .y = 2 } };

    // Initialize a 3 * 3 array of DEAD Cells
    const cells_len: comptime_int = 9;
    var cells: [cells_len]Cell = .{Cell.DEAD} ** cells_len;
    const ms: u32 = 500;

    const game = try Conway.init(allocator, 3, 3, &init, "jee", 10, ms, &cells);
    defer allocator.destroy(game);

    try std.testing.expect(game.cells.len == 9);

    try game.evolve();
    try game.evolve();

    const blinker: [cells_len]Cell = .{
        Cell.DEAD,
        Cell.ALIVE,
        Cell.DEAD,
        Cell.DEAD,
        Cell.ALIVE,
        Cell.DEAD,
        Cell.DEAD,
        Cell.ALIVE,
        Cell.DEAD,
    };

    try std.testing.expectEqualSlices(Cell, &blinker, game.cells);
}

test "initializing a Conway structure with a Blinker pattern works and has the correct pattern after three runs (Blinker should be flipped)" {
    const allocator = std.testing.allocator;
    const init_len: comptime_int = 3;
    const init: [init_len]Coordinate = [init_len]Coordinate{ .{ .x = 1, .y = 0 }, .{ .x = 1, .y = 1 }, .{ .x = 1, .y = 2 } };

    // Initialize a 3 * 3 array of DEAD Cells
    const cells_len: comptime_int = 9;
    var cells: [cells_len]Cell = .{Cell.DEAD} ** cells_len;
    const ms: u32 = 500;

    const game = try Conway.init(allocator, 3, 3, &init, "jee", 10, ms, &cells);
    defer allocator.destroy(game);

    try std.testing.expect(game.cells.len == 9);

    try game.evolve();
    try game.evolve();
    try game.evolve();

    const blinker: [cells_len]Cell = .{
        Cell.DEAD,
        Cell.DEAD,
        Cell.DEAD,
        Cell.ALIVE,
        Cell.ALIVE,
        Cell.ALIVE,
        Cell.DEAD,
        Cell.DEAD,
        Cell.DEAD,
    };

    try std.testing.expectEqualSlices(Cell, &blinker, game.cells);
}
