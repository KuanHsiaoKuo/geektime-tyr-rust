use std::process::Command;

fn main() {
    // 在编译时可选择检查环境变量。
    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);
    // 如果没有找到环境变量的对应值，就直接return，不再进行后续编译
    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }
    // 使用 prost_build 把 abi.proto 编译到 src/pb 目录下
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
    Command::new("cargo")
        .args(&["fmt", "--", "src/*.rs"])
        .status()
        .expect("cargo fmt failed");
}
