extern crate regex;
extern crate svgbob;
extern crate uuid;

#[path = "pikchr/pikchr.rs"] // 根据实际路径调整
mod pikchr;
use pikchr::Pikchr;

use regex::Regex;
use uuid::Uuid;

pub use svgbob::Settings;
use svgbob::{CellBuffer, Node};

use log::info;
use mdbook::book::Book;
use mdbook::book::BookItem;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

use std::env;
use std::process::Command;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::OnceLock;

use std::sync::atomic::{AtomicI32, Ordering};
static TABINDEX: AtomicI32 = AtomicI32::new(0);

static GLOBAL_ROOT_DIR: OnceLock<String> = OnceLock::new();
static PICTUREINDEX: AtomicI32 = AtomicI32::new(0);

pub struct ECharts;

impl ECharts {
    pub fn new() -> ECharts {
        ECharts
    }
}

impl Preprocessor for ECharts {
    fn name(&self) -> &str {
        "echarts"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        // 获取根目录路径
        GLOBAL_ROOT_DIR.get_or_init(|| ctx.root.to_string_lossy().into_owned());

        // 遍历书籍章节，替换 ECharts 代码块
        book.for_each_mut(|item: &mut BookItem| {
            PICTUREINDEX.store(0, Ordering::SeqCst);
            if let BookItem::Chapter(ref mut chapter) = *item {
                chapter.content = gen(&chapter.name, chapter.content.as_str())
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported" // 支持所有渲染器（示例逻辑）
    }
}

pub fn gen(name: &str, content: &str) -> String {
    let mut s = String::from(content);

    const TAG_START_1: &str = "```echarts";
    const TAG_END_1: &str = "```";
    // 匹配语法：```echarts 代码块
    let re = Regex::new(r"```echarts((.*\n)+?)?```").unwrap();
    for mat in re.find_iter(s.clone().as_str()) {
        info!("echarts prepocessor start");
        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_1, TAG_END_1];
        let buf = echarts_gen_html(mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    const TAG_START_2: &str = "```bob";
    const TAG_END_2: &str = "```";
    // 匹配语法：```bob 代码块
    let re = Regex::new(r"```bob((.*\n)+?)?```").unwrap();
    for mat in re.find_iter(s.clone().as_str()) {
        info!("svgbob prepocessor start");
        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_2, TAG_END_2];
        let buf = svgbob_gen_html(mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    const TAG_START_3: &str = "```bytefield";
    const TAG_END_3: &str = "```";
    // 匹配语法：```bytefield 代码块
    let re = Regex::new(r"```bytefield((.*\n)+?)?```").unwrap();
    for mat in re.find_iter(s.clone().as_str()) {
        info!("bytefield prepocessor start");
        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_3, TAG_END_3];
        let buf = bytefield_gen_html(mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    const TAG_START_4: &str = "```latex tex";
    const TAG_END_4: &str = "```";
    // 匹配语法：```latex tex代码块
    let re = Regex::new(r"```latex tex((.*\n)+?)?```").unwrap();
    for mat in re.find_iter(s.clone().as_str()) {
        info!("latex tex prepocessor start");
        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_4, TAG_END_4];
        let buf = latex_gen_html(mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    const TAG_START_5: &str = "```latex tikz";
    const TAG_END_5: &str = "```";
    // 匹配语法：```latex tikz 代码块
    let re = Regex::new(r"```latex tikz((.*\n)+?)?```").unwrap();
    for mat in re.find_iter(s.clone().as_str()) {
        info!("latex tikz prepocessor start");
        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_5, TAG_END_5];
        let buf = tikz_gen_file(name, mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    const TAG_START_6: &str = "```pikchr";
    const TAG_END_6: &str = "```";
    // 匹配语法：```pikchr 代码块
    let re = Regex::new(r"```pikchr((.*\n)+?)?```").unwrap();
    for mat in re.find_iter(s.clone().as_str()) {
        info!("pikchr prepocessor start");
        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_6, TAG_END_6];
        let buf = pikchr_gen_html(mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    const TAG_START_7: &str = "```typst";
    const TAG_END_7: &str = "```";
    // 匹配语法：```typst 代码块
    let re = Regex::new(r"```typst((.*\n)+?)?```").unwrap();
    for mat in re.find_iter(s.clone().as_str()) {
        info!("typst prepocessor start");
        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_7, TAG_END_7];
        let buf = typst_gen_file(name, mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    return s;
}

fn echarts_gen_html(mat_str: &str, empty_str_vec: Vec<&str>) -> String {
    let mut mat_string = String::from(mat_str);
    // 清理代码块标记
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    // 生成包含唯一 ID 的 HTML 和 JS 代码
    // 方案1
    // let uuid = Uuid::new_v4().to_simple().to_string();
    // let buf = format!(r#"<div>
    //                         <div id="{}" style="height: 500px;"></div>
    //                         <script type="text/javascript">
    //                             document.addEventListener('DOMContentLoaded', function() {{
    //                                 var chart_{} = echarts.init(
    //                                     document.getElementById('{}'),
    //                                     null,
    //                                     {{ renderer: 'svg', useDirtyRect: false }}
    //                                 );
    //                                 var app = {{}};
    //                                 var option;
    //                                 {}
    //                                 option && chart_{}.setOption(option);
    //                             }});
    //                         </script>
    //                      </div>"#, uuid, uuid, uuid, mat_string.trim(), uuid);

    // 方案2
    let uuid = Uuid::new_v4().to_simple().to_string();

    // 新增替换逻辑
    let mut content = mat_string.trim().to_string();
    content = content.replace("chartDom", &format!("chartDom_{}", uuid));
    content = content.replace("myChart", &format!("chart_{}", uuid));
    content = content.replace("main", &uuid);

    // 新增空行消除逻辑（结合网页[3][11]的跨平台处理经验）
    let re = Regex::new(r"(?m)^\s*\n").unwrap(); // 匹配空白行
    content = re.replace_all(&content, "\n").to_string(); // 替换为单个换行
    let re = Regex::new(r"\n{2,}").unwrap(); // 匹配连续换行
    content = re.replace_all(&content, "\n").to_string();

    let buf = format!(
        r#"
<div>
    <div id="{}" style="height: 500px" style="text-align: center;">
    <script type="text/javascript">
        document.addEventListener('DOMContentLoaded', function() {{
            {}
        }});
    </script>
    </div>
</div>
                         "#,
        uuid, content
    );

    return buf;
}

/// convert bob ascii diagrams to svg
pub fn bob_handler(s: &str, settings: &Settings) -> String {
    let cb = CellBuffer::from(s);
    let (svg, _, _): (Node<()>, f32, f32) = cb.get_node_with_size(settings);

    let mut source = String::new();
    svg.render_with_indent(&mut source, 0, true)
        .expect("html render");

    let uuid = Uuid::new_v4().to_simple().to_string();
    format!("{}", source).replace("svgbob", &format!("svgbob_{}", uuid))
}

fn svgbob_gen_html(mat_str: &str, empty_str_vec: Vec<&str>) -> String {
    let mut mat_string = String::from(mat_str);
    // 清理代码块标记
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    let settings = Settings::default();
    let svg = bob_handler(&mat_string, &settings);
    let buf = format!(
        r#"
<pre class="diagram-svgbob" style="text-align: center;">
{}
</pre>
                         "#,
        svg
    );

    return buf;
}

fn bytefield_gen_html(mat_str: &str, empty_str_vec: Vec<&str>) -> String {
    let mut mat_string = String::from(mat_str);
    // 清理代码块标记
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    let mut content = mat_string.trim().to_string();
    let re = Regex::new(r"(?m)^\s*\n").unwrap(); // 匹配空白行
    content = re.replace_all(&content, "\n").to_string(); // 替换为单个换行
    let re = Regex::new(r"\n{2,}").unwrap(); // 匹配连续换行
    content = re.replace_all(&content, "\n").to_string();

    let buf = format!(
        r#"
<div>
    <div id="CommonMark-bytefiled" style="text-align: center;">
    <pre tabindex="{}"><code class="language-bytefield" data-lang="bytefield">
{}
    </code></pre>
    </div>
</div>"#,
        TABINDEX.load(Ordering::Relaxed),
        content
    );
    let _ = TABINDEX.fetch_add(1, Ordering::SeqCst) + 1;

    return buf;
}

fn latex_gen_html(mat_str: &str, empty_str_vec: Vec<&str>) -> String {
    let mut mat_string = String::from(mat_str);

    // 清理代码块标记
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    let mut content = mat_string.trim().to_string();
    let re = Regex::new(r"(?m)^\s*\n").unwrap(); // 匹配空白行
    content = re.replace_all(&content, "\n").to_string(); // 替换为单个换行
    let re = Regex::new(r"\n{2,}").unwrap(); // 匹配连续换行
    content = re.replace_all(&content, "\n").to_string();

    let buf = format!(
        r##"
<div>
    <div id="CommonMark-latex"></div>
    <latex-js baseURL="https://cdn.jsdelivr.net/npm/latex.js/dist/"><code>
{}
    </code></latex-js>
</div>"##,
        content
    );

    return buf;
}

#[allow(dead_code)]
fn tikz_to_svg(tikzcode: &str) -> Result<String, String> {
    // 获取当前可执行文件的路径
    let exe_path = env::current_exe().map_err(|e| e.to_string())?;

    // 确定可执行文件所在的目录
    let exe_dir = exe_path
        .parent()
        .ok_or("<!-- Executable directory not found -->")?;

    // 根据操作系统确定转换器名称
    let converter_name = if cfg!(windows) {
        "tikz2svg.exe"
    } else {
        "tikz2svg"
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

#[allow(dead_code)]
fn tikz_gen_html(mat_str: &str, empty_str_vec: Vec<&str>) -> String {
    let mut mat_string = String::from(mat_str);
    // 清理代码块标记
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    let ret = match tikz_to_svg(&mat_string) {
        Ok(svg) => format!("{}", svg),
        Err(e) => format!("{}", e),
    };
    let buf = format!(
        r#"
<pre class='diagram-svgbob'>
{}
</pre>
                         "#,
        ret
    );

    return buf;
}

fn tikz_gen_file(name: &str, mat_str: &str, empty_str_vec: Vec<&str>) -> String {
    let mut mat_string = String::from(mat_str);
    let mut name_string = String::from(name);

    // 替换特殊字符
    let re = Regex::new(r"[\\/]| +").unwrap();                  // 匹配斜杠和连续空格
    name_string = re.replace_all(&name_string, "_").to_string();

    // 清理代码块标记
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    // 匹配以 % 开头的单词
    let re = Regex::new(r"^\s*%+\s*([[:word:]]+)").unwrap();
    let title = mat_string.lines().find_map(|line| {
        re.captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    });

    // 若无匹配则返回默认值
    let title = title.unwrap_or_else(|| "samples".to_string());

    // 创建目标目录结构
    let root_str = GLOBAL_ROOT_DIR
        .get()
        .expect("GLOBAL_ROOT_DIR not initialized");
    let dir_path = PathBuf::from(root_str)
        .join("src")
        .join("images")
        .join(&name_string);
    fs::create_dir_all(&dir_path).expect("Failed to create directories"); // 递归创建目录[1,6](@ref)

    // 文件名构建
    let filename = format!("{}_{}.tex", title, PICTUREINDEX.load(Ordering::Relaxed));
    let svgname = format!("{}_{}.svg", title, PICTUREINDEX.load(Ordering::Relaxed));
    let _ = PICTUREINDEX.fetch_add(1, Ordering::SeqCst) + 1;

    // 构建完整文件路径
    let mut file_path = dir_path.clone();
    file_path.push(&filename);

    // 写入文件内容
    if file_path.exists() {
        // 读取现有文件内容
        let existing_content = fs::read_to_string(&file_path)
            .expect("Failed to read existing file content");
        if existing_content != mat_string {
            let mut file = File::create(&file_path).expect("Failed to create file"); // 创建或覆盖文件[4](@ref)
            file.write_all(mat_string.as_bytes())
                .expect("Failed to write content"); // 二进制写入优化[8](@ref)
            file.flush().expect("Failed to flush data"); // 确保数据落盘
        }
    } 
    else {
        let mut file = File::create(&file_path).expect("Failed to create file"); // 创建或覆盖文件[4](@ref)
        file.write_all(mat_string.as_bytes())
            .expect("Failed to write content"); // 二进制写入优化[8](@ref)
        file.flush().expect("Failed to flush data"); // 确保数据落盘
    }

    // 查找对应svg文件
    let stem = file_path.file_stem().map(|s| s.to_os_string());
    if let Some(stem) = stem {
        file_path.set_file_name(&stem);
        file_path.set_extension("svg");
    }

    // 清理代码行中的<p></p>换行符
    let mut content = mat_string.trim().to_string();
    let re = Regex::new(r"(?m)^\s*\n").unwrap(); // 匹配空白行
    content = re.replace_all(&content, "\n").to_string(); // 替换为单个换行
    let re = Regex::new(r"\n{2,}").unwrap(); // 匹配连续换行
    content = re.replace_all(&content, "\n").to_string();

    if file_path.exists() {
        let buf = format!(
        r#"
<div><details><summary>{}</summary>
<div id="CommonMark-latex tikz"></div>
<pre><code class="language-latex tikz">
{}
</code></pre></details></div>
<div align=center>
<img src="./../images/{}/{}" alt="{}" class="miv_mdbook-image-viewer" onclick="miv_openModal(this.src)">
</div>"#, 
filename, content, name_string, svgname, name);
        return buf;
    } 
    else {
        let buf = format!(
        r#"
<div><details><summary>{}</summary>
<div id="CommonMark-latex tikz"></div>
<pre><code class="language-latex tikz">
{}
</code></pre></details></div>
"#, filename, content);
        return buf;
    }
}

fn pikchr_gen_html(mat_str: &str, empty_str_vec: Vec<&str>) -> String {
    let mut mat_string = String::from(mat_str);
    // 清理代码块标记
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    let buf = match Pikchr::render(&mat_string, None) {
        Ok(svg) => format!(
            r#"<div style="margin:0 auto; max-width: {}px ">{}</div>"#,
            svg.width,
            svg.to_string()
        ),
        Err(err) => format!(
            r#"
<div>
    <div id="CommonMark-pikchr"></div>
    <code>
    {}
    {}
    </code>
</div>"#,
            mat_string, err
        ),
    };

    return buf;
}

fn typst_gen_file(name: &str, mat_str: &str, empty_str_vec: Vec<&str>) -> String {
    let mut mat_string = String::from(mat_str);
    let mut name_string = String::from(name);

    // 替换特殊字符
    let re = Regex::new(r"[\\/]| +").unwrap();                  // 匹配斜杠和连续空格
    name_string = re.replace_all(&name_string, "_").to_string();

    // 清理代码块标记
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    // 匹配以 // 开头的单词
    let re = Regex::new(r"^\s*//+\s*([[:word:]]+)").unwrap();
    let title = mat_string.lines().find_map(|line| {
        re.captures(line)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    });

    // 若无匹配则返回默认值
    let title = title.unwrap_or_else(|| "samples".to_string());

    // 创建目标目录结构
    let root_str = GLOBAL_ROOT_DIR
        .get()
        .expect("GLOBAL_ROOT_DIR not initialized");
    let dir_path = PathBuf::from(root_str)
        .join("src")
        .join("images")
        .join(&name_string);
    fs::create_dir_all(&dir_path).expect("Failed to create directories"); // 递归创建目录[1,6](@ref)

    // 文件名构建
    let filename = format!("{}_{}.typ", title, PICTUREINDEX.load(Ordering::Relaxed));
    let svgname = format!("{}_{}.svg", title, PICTUREINDEX.load(Ordering::Relaxed));
    let _ = PICTUREINDEX.fetch_add(1, Ordering::SeqCst) + 1;

    // 构建完整文件路径
    let mut file_path = dir_path.clone();
    file_path.push(&filename);

    if file_path.exists() {
        // 读取现有文件内容
        let existing_content = fs::read_to_string(&file_path)
            .expect("Failed to read existing file content");
        if existing_content != mat_string {
            let mut file = File::create(&file_path).expect("Failed to create file"); // 创建或覆盖文件[4](@ref)
            file.write_all(mat_string.as_bytes())
                .expect("Failed to write content"); // 二进制写入优化[8](@ref)
            file.flush().expect("Failed to flush data"); // 确保数据落盘
        }
    } 
    else {
        // 写入文件内容
        let mut file = File::create(&file_path).expect("Failed to create file"); // 创建或覆盖文件[4](@ref)
        file.write_all(mat_string.as_bytes())
            .expect("Failed to write content"); // 二进制写入优化[8](@ref)
        file.flush().expect("Failed to flush data"); // 确保数据落盘
    }

    // 查找对应svg文件
    let stem = file_path.file_stem().map(|s| s.to_os_string());
    if let Some(stem) = stem {
        file_path.set_file_name(&stem);
        file_path.set_extension("svg");
    }

    // 清理代码行中的<p></p>换行符
    let mut content = mat_string.trim().to_string();
    let re = Regex::new(r"(?m)^\s*\n").unwrap(); // 匹配空白行
    content = re.replace_all(&content, "\n").to_string(); // 替换为单个换行
    let re = Regex::new(r"\n{2,}").unwrap(); // 匹配连续换行
    content = re.replace_all(&content, "\n").to_string();

    if file_path.exists() {
        let buf = format!(
        r#"
<div><details><summary>{}</summary>
<div id="CommonMark-typst"></div>
<pre><code class="language-typst">
{}
</code></pre></details></div>
<div align=center>
<img src="./../images/{}/{}" alt="{}" class="miv_mdbook-image-viewer" onclick="miv_openModal(this.src)">
</div>"#, 
filename, content, name_string, svgname, name);
        return buf;
    } 
    else {
        let buf = format!(
        r#"
<div><details><summary>{}</summary>
<div id="CommonMark-typst"></div>
<pre><code class="language-typst">
{}
</code></pre></details></div>
"#, filename, content);
        return buf;
    }
}

