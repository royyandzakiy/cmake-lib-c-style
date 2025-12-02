// build.zig - Updated version
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "complex_test",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Add include path for the header file
    exe.addIncludePath(.{ .path = "." });

    // For Windows, you might need to add library path
    exe.addLibraryPath(.{ .path = "." });

    // Link with the DLL
    exe.linkSystemLibraryName("complexNumbers");

    // Or use linkLibC if it's a C library
    exe.linkLibC();

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run the app");
    run_step.dependOn(&run_cmd.step);
}
