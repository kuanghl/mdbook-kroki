use std::env;
use std::process::Command;

// use env_logger::Builder;
// use log::LevelFilter;
// use log::{info, warn};

fn tikz_to_svg(tikzcode: &str) -> Result<String, String> {
    // 获取当前可执行文件的路径
    let exe_path = env::current_exe().map_err(|e| e.to_string())?;

    // 确定可执行文件所在的目录
    let exe_dir = exe_path
        .parent()
        .ok_or("<!-- Executable directory not found -->")?;

    // 根据操作系统确定转换器名称
    let converter_name = if cfg!(windows) {
        "tikz-converter.exe"
    } else {
        "tikz-converter"
    };
    // 构建转换器的完整路径
    let converter_path = exe_dir.join(converter_name);

    // 调用转换器并捕获输出
    let output = Command::new(converter_path)
        .arg("--string")
        .arg(tikzcode)
        .output()
        .map_err(|e| e.to_string())?;

    // 将输出转换为UTF-8字符串
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Builder::new()
    //     .filter_level(LevelFilter::Info) // 强制设置默认级别为 info
    //     .init(); // 初始化日志系统

    // info!("预处理开始"); // 信息级日志
    // warn!("检测到空章节"); // 警告级日志

    let tikz_code = r#"\begin{document} \begin{tikzpicture} \draw (0,0) circle (1in); \end{tikzpicture} \end{document}"#;
    let ret = match tikz_to_svg(tikz_code) {
        Ok(svg) => format!("{}", svg),
        Err(e) => format!("{}", e),
    };
    println!("{}", ret);
    Ok(())
}