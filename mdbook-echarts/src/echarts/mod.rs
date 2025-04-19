extern crate regex;
extern crate uuid;
extern crate svgbob;

use regex::Regex;
use uuid::Uuid;

use svgbob::{CellBuffer, Node};
pub use svgbob::Settings;

use mdbook::book::Book;
use mdbook::book::BookItem;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

#[derive(Debug)]
pub struct MdFile {
    pub name: String,
    pub path: String,
}

#[derive(Debug)]
pub struct MdGroup {
    pub name: String,
    pub path: String,
    pub has_readme: bool,
    pub group_list: Vec<MdGroup>,
    pub md_list: Vec<MdFile>,
}

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

        // 遍历书籍章节，替换 ECharts 代码块
        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                chapter.content = gen(chapter.content.as_str())
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"  // 支持所有渲染器（示例逻辑）
    }
}


pub fn gen(content: &str) -> String {
    let mut s = String::from(content);

    const TAG_START_1: &str = "```echarts";
    const TAG_END_1: &str = "```";
    // 匹配语法：```echarts 代码块
    let re = Regex::new(r"```echarts((.*\n)+?)?```").unwrap();
    for mat in re.find_iter(s.clone().as_str()) {
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
        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_2, TAG_END_2];
        let buf = svgbob_gen_html(mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    return s;
}

/// convert bob ascii diagrams to svg
pub fn bob_handler(s: &str, settings: &Settings) -> String {
	let cb = CellBuffer::from(s);
	let (svg, _, _): (Node<()>, f32, f32) = cb.get_node_with_size(settings);

	let mut source = String::new();
	svg.render_with_indent(&mut source, 0, true).expect("html render");

    let uuid = Uuid::new_v4().to_simple().to_string();
	format!("{}", source).replace("svgbob", &format!("svgbob_{}", uuid))
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
    let re = Regex::new(r"(?m)^\s*\n").unwrap();            // 匹配空白行
    content = re.replace_all(&content, "\n").to_string();   // 替换为单个换行
    let re = Regex::new(r"\n{2,}").unwrap();                // 匹配连续换行
    content = re.replace_all(&content, "\n").to_string();

    let buf = format!(r#"
<div>
    <div id="{}" style="height: 500px;"></div>
    <script type="text/javascript">
        document.addEventListener('DOMContentLoaded', function() {{
            {}
        }});
    </script>
</div>
                         "#, uuid, content);

    return buf;
}

fn svgbob_gen_html(mat_str: &str, empty_str_vec: Vec<&str>) -> String {
    let mut mat_string = String::from(mat_str);
    // 清理代码块标记
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    let settings = Settings::default();
    let svg = bob_handler(&mat_string, &settings);
    let buf = format!(r#"
<pre class='diagram-svgbob'>
{}
</pre>
                         "#, svg);

    return buf;
}