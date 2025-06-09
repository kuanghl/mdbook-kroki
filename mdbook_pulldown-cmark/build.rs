// build.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 获取当前构建模式 (debug/release)
    let _profile = env::var("PROFILE").unwrap();

    // 获取二进制输出目录
    let out_dir = Path::new(&env::var("OUT_DIR").unwrap())
        .join("../../..") // 向上回溯到 target/debug 或 target/release
        .canonicalize()
        .unwrap();

    // 需要复制的文件列表 (相对项目根目录)
    let files_to_copy = vec![
        "card0.md",
        "card1.md",
        "card2.md",
        "card3.md",
    ];

    // 执行文件复制
    for file in files_to_copy {
        let src = Path::new(file);
        let dest = out_dir.join(src);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::copy(src, dest).unwrap();
    }
}