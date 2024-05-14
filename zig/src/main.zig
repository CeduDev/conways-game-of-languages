const std = @import("std");
const print = std.debug.print;
const sleep = std.time.sleep;
const c = @import("conway.zig");
const Conway = c.Conway;
const Cell = c.Cell;
const Coordinate = c.Coordinate;

fn clear_screen() !void {
    print("\x1B[2J\x1B[H", .{});
}

pub fn main() !void {
    const init = comptime [_]Coordinate{ .{ .x = 1, .y = 0 }, .{ .x = 1, .y = 1 }, .{ .x = 1, .y = 2 } };
    const game: Conway = Conway.new(3, 3, &init, "jee", 2, 2);

    _ = game; // autofix
    // _ = game; // autofix

    // const a = [9]u16{ 0, 1, 0, 0, 1, 0, 0, 1, 0 };
    // const width = 3;
    // const height = 3;
    // _ = height; // autofix
    // var b: u16 = 1;
    // const ns_to_s: u32 = 1000000000;

    // try clear_screen();

    // inline for (0.., a) |i, elem| {
    //     print(if (elem == 1) "♥" else "‧", .{});
    //     if ((i + 1) % width == 0) {
    //         print("\n", .{});
    //     }
    // }

    // print("Generation: {}\n", .{b});

    // b += 1;

    // sleep(1 * ns_to_s);

    // try clear_screen();

    // inline for (0.., a) |i, elem| {
    //     print(if (elem == 1) "♥" else "‧", .{});
    //     if ((i + 1) % width == 0) {
    //         print("\n", .{});
    //     }
    // }

    // print("Generation: {}\n", .{b});
}

// pub fn main() !void {
//     // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
//     std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

//     // stdout is for the actual output of your application, for example if you
//     // are implementing gzip, then only the compressed bytes should be sent to
//     // stdout, not any debugging messages.
//     const stdout_file = std.io.getStdOut().writer();
//     var bw = std.io.bufferedWriter(stdout_file);
//     const stdout = bw.writer();

//     try stdout.print("Run `zig build test` to run the tests.\n", .{});

//     try bw.flush(); // don't forget to flush!
// }

// test "simple test" {
//     var list = std.ArrayList(i32).init(std.testing.allocator);
//     defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
//     try list.append(42);
//     try std.testing.expectEqual(@as(i32, 42), list.pop());
// }
