const std = @import("std");
const config = @import("config");
const token = @import("./tokenizer/token.zig");

const usage_flag = std.fmt.comptimePrint(
    \\Reap compiler version {s}
    \\Usage: ./reap [option]
    \\
    \\Options:
    \\  +help            Show help.
    \\  +version         Show version.
    \\  +build FILES...  Compile reap files.
    \\
, .{config.version});

const version_flag = std.fmt.comptimePrint("Reap compiler version {s}\n", .{config.version});

pub fn main() !void {
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = general_purpose_allocator.deinit();
    const allocator = general_purpose_allocator.allocator();
    const args = try std.process.argsAlloc(allocator);
    defer std.process.argsFree(allocator, args);

    // No args is equivalent as +help
    if (args.len == 1) {
        std.debug.print(usage_flag, .{});
        return;
    }

    for (args, 0..) |arg, i| {
        if (std.mem.eql(u8, arg, "+version")) {
            std.debug.print(version_flag, .{});
            return;
        } else if (std.mem.eql(u8, arg, "+help")) {
            std.debug.print(usage_flag, .{});
            return;
        }
        _ = i;
        //std.debug.print("{}: {s}\n", .{ i, arg });
    }

    const toto: token.Token = .{
        .pos = 1,
        .kind = token.Kind.OpenParen,
    };

    std.debug.print("{s}\n", .{@tagName(toto.kind)});

    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();

    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try bw.flush(); // don't forget to flush!
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
