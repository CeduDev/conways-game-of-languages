const std = @import("std");
const sleep = std.time.sleep;
const print = std.debug.print;

pub const Coordinate = struct { x: u16, y: u16 };

pub const Cell = enum { DEAD, ALIVE };

const ns_to_s: u32 = 1000000;

pub const Conway = struct {
    height: u16,
    width: u16,
    text: []const u8,
    cells: []Cell,
    generations: u16,
    current_generation: u16,
    delay: u32,
    const Self = @This();

    fn clear_screen() !void {
        print("\x1B[2J\x1B[H", .{});
    }

    pub fn init(allocator: std.mem.Allocator, comptime width: u16, comptime height: u16, comptime init_cells: []const Coordinate, text: []const u8, generations: u16, delay_in_milli_seconds: u32, cells: []Cell) !*Self {
        // Set the cells from init_cells to ALIVE in cells
        for (init_cells) |c| {
            cells[c.y * height + c.x] = Cell.ALIVE;
        }

        const res = try allocator.create(Self);

        res.* = Self{ .width = width, .height = height, .text = text, .cells = cells, .generations = generations, .current_generation = 0, .delay = delay_in_milli_seconds * ns_to_s };

        return res;
    }

    pub fn print_game(self: Self) void {
        try clear_screen();

        print("{s}, Generation: {}\n", .{ self.text, self.current_generation + 1 });

        for (self.cells, 0..) |cell, i| {
            if (cell == Cell.ALIVE) {
                print("♥", .{});
            } else {
                print("‧", .{});
            }
            if ((i + 1) % self.width == 0) {
                print("\n", .{});
            }
        }
    }

    pub fn run(self: *Self) void {
        for (0..self.generations) |_| {
            self.print_game();
            sleep(self.delay);
            self.current_generation += 1;
        }
    }
};
