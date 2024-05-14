const std = @import("std");
const print = std.debug.print;

pub const Coordinate = struct { x: u16, y: u16 };

pub const Cell = enum { DEAD, ALIVE };

pub const Conway = struct {
    height: u16,
    width: u16,
    text: []const u8,
    cells: []Cell,
    generations: u16,
    current_generation: u16,
    delay: u32,

    pub fn new(comptime width: u16, comptime height: u16, comptime init_cells: []const Coordinate, text: []const u8, generations: u16, delay: u32) Conway {
        var init_cells_index: [init_cells.len]u16 = undefined;
        var cells: [width * height]Cell = undefined;

        for (0.., init_cells) |i, c| {
            init_cells_index[i] = c.y * height + c.x;
        }

        for (init_cells_index) |index| {
            cells[index] = Cell.ALIVE;
        }

        for (&cells) |*cell| {
            if (cell == undefined) {
                cell.* = Cell.DEAD;
            }
        }

        return Conway{ .width = width, .height = height, .text = text, .cells = &cells, .generations = generations, .current_generation = 0, .delay = delay };
    }
};
