const std = @import("std");
const sleep = std.time.sleep;
const print = std.debug.print;

pub const Coordinate = struct { x: u16, y: u16 };

pub const Cell = enum { DEAD, ALIVE };

const DELTAS: [8]struct { col: i8, row: i8 } = .{
    .{ .row = -1, .col = -1 },
    .{ .row = -1, .col = 0 },
    .{ .row = -1, .col = 1 },
    .{ .row = 0, .col = -1 },
    .{ .row = 0, .col = 1 },
    .{ .row = 1, .col = -1 },
    .{ .row = 1, .col = 0 },
    .{ .row = 1, .col = 1 },
};

const ns_to_s: u64 = 1000000;

pub const Conway = struct {
    allocator: std.mem.Allocator,
    height: u16,
    width: u16,
    text: []const u8,
    cells: []Cell,
    generations: u16,
    current_generation: u16,
    delay: u64,
    const Self = @This();

    fn clear_screen() !void {
        print("\x1B[2J\x1B[H", .{});
    }

    pub fn init(allocator: std.mem.Allocator, width: u16, height: u16, init_cells: []const Coordinate, text: []const u8, generations: u16, delay_in_milli_seconds: u32, cells: []Cell) !*Self {
        // Set the cells from init_cells to ALIVE in cells
        for (init_cells) |c| {
            cells[c.y * height + c.x] = Cell.ALIVE;
        }

        const res = try allocator.create(Self);

        res.* = Self{ .allocator = allocator, .width = width, .height = height, .text = text, .cells = cells, .generations = generations, .current_generation = 0, .delay = delay_in_milli_seconds * ns_to_s };

        return res;
    }

    fn print_game(self: Self) void {
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

    fn get_cells_index(self: Self, row: usize, col: usize) usize {
        return (row * self.width + col);
    }

    fn get_live_neighors(self: Self, row: usize, col: usize) u8 {
        var count: u8 = 0;

        for (DELTAS) |d| {
            const drow = d.row;
            const dcol = d.col;

            const r_ = @as(i8, @intCast(row));
            const c_ = @as(i8, @intCast(col));

            if (r_ + drow < 0 or
                c_ + dcol < 0 or
                r_ + drow > self.height - 1 or
                c_ + dcol > self.width - 1)
            {
                continue;
            }

            const n_r: usize = @intCast(r_ + drow);
            const n_c: usize = @intCast(c_ + dcol);

            const n_idx = self.get_cells_index(n_r, n_c);
            const cell = self.cells[n_idx];
            if (cell == Cell.ALIVE) {
                count += 1;
            }
        }

        return count;
    }

    pub fn evolve(self: *Self) !void {
        const next_generation: []Cell = try self.allocator.alloc(Cell, self.cells.len);
        defer self.allocator.free(next_generation);

        for (0..self.height) |row| {
            for (0..self.width) |col| {
                const idx = self.get_cells_index(row, col);
                const cell = self.cells[idx];
                const live_neighbors = self.get_live_neighors(row, col);

                var next_cell: Cell = Cell.DEAD;

                switch (cell) {
                    Cell.ALIVE => {
                        if (live_neighbors == 2 or live_neighbors == 3) {
                            next_cell = Cell.ALIVE;
                        }
                    },
                    Cell.DEAD => {
                        if (live_neighbors == 3) {
                            next_cell = Cell.ALIVE;
                        }
                    },
                }

                next_generation[idx] = next_cell;
            }
        }

        for (next_generation, 0..) |c, i| {
            self.cells[i] = c;
        }
    }

    pub fn run(self: *Self) !void {
        for (0..self.generations) |_| {
            self.print_game();
            sleep(self.delay);
            try self.evolve();
            self.current_generation += 1;
        }
    }
};
