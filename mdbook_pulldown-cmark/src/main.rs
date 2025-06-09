use std::fs;
use pulldown_cmark::{Parser, Options, html, Event, CowStr};
use katex;

/// 数学公式处理函数生成器（闭包工厂）
pub fn katex_math_processor() -> impl Fn(Event) -> Event {
    let display_opts = katex::Opts::builder()
        .display_mode(true)
        .build()
        .expect("Invalid KaTeX options");

    move |event| match event {
        // 处理行内公式（需要双重转换）
        Event::InlineMath(formula) => {
            katex::render(&formula)
                .map(|s| Event::InlineHtml(CowStr::from(s)))  // String → CowStr
                .unwrap_or_else(|_| Event::Text(formula.into()))
        }
        
        // 处理块级公式（类型转换+错误处理）
        Event::DisplayMath(formula) => {
            katex::render_with_opts(&formula, &display_opts)
                .map(|s| Event::Html(CowStr::from(s)))        // 显式类型转换
                .unwrap_or_else(|_| Event::Text(formula.into()))
        }

        other => other,
    }
}

fn main() {
    // 读取文件（如果失败直接 panic）
    let card0 = fs::read_to_string("card3.md")
        .expect("找不到 card3.md 文件");
 
    // 设置选项，启用删除线
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    // options.insert(Options::ENABLE_SMART_PUNCTUATION);
    // options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    // options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    // options.insert(Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS);
    // options.insert(Options::ENABLE_OLD_FOOTNOTES);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_GFM);
    // options.insert(Options::ENABLE_DEFINITION_LIST);
    options.insert(Options::ENABLE_SUPERSCRIPT);
    options.insert(Options::ENABLE_SUBSCRIPT);
    options.insert(Options::ENABLE_WIKILINKS);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // 创建katex处理管道
    let parser = Parser::new_ext(&card0, options).map(katex_math_processor());

    // 生成 HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // 包装完整 HTML 结构（添加 GitHub CSS 样式）
    let full_html = format!(
r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Enhanced Cards</title>
    <!-- <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.2.0/github-markdown.min.css"> -->
    <link href="https://cdn.jsdelivr.net/npm/github-markdown-css@5.8.1/github-markdown.min.css" rel="stylesheet">
    <script async src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/2.7.1/MathJax.js?config=TeX-AMS-MML_HTMLorMML"></script>

    <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.16.22/dist/katex.min.css">
</head>
<body>
    <div>
        <div class="cards"></div>
        {}
    </div>
    <script>
        // 动态光晕追踪
        document.querySelectorAll('.card').forEach(card => {{
            card.addEventListener('mousemove', (e) => {{
                const rect = card.getBoundingClientRect();
                const x = e.clientX - rect.left;
                const y = e.clientY - rect.top; // 添加 y 变量定义
                card.style.setProperty('--x', `${{x}}px`); // 双重转义
                card.style.setProperty('--y', `${{y}}px`); // 双重转义
            }});
        }});
    </script>
</body>
</html>"#, html_output);

    println!(r#"{}"#, full_html);
}