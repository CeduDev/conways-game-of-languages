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

    pub fn new(comptime width: u16, comptime height: u16, comptime init_cells: []const Coordinate, text: []const u8, generations: u16, delay: u32, cells: []Cell) Conway {
        // Set the cells from init_cells to ALIVE in cells
        for (init_cells) |c| {
            cells[c.y * height + c.x] = Cell.ALIVE;
        }

        return Conway{ .width = width, .height = height, .text = text, .cells = cells, .generations = generations, .current_generation = 0, .delay = delay };
    }
};
