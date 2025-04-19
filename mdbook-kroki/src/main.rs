use anyhow::{Result};
use rayon::prelude::*;
use regex::Regex;
use std::borrow::Cow;

// 定义数据结构表示文本片段
enum Segment<'a> {
    Text(&'a str),
    CodeBlock { diagram_type: String, code: String }
}

pub fn render_kroki_blocks(input: &str) -> Result<String> {
    let re = Regex::new(r"(?s)```kroki-(\w+)\n(.*?)```").unwrap();
    let mut segments = Vec::new();
    let mut last_end = 0;

    // 第一步：分割文本为交替的普通文本和代码块
    for cap in re.captures_iter(input) {
        let full_match = cap.get(0).unwrap();
        let text_segment = &input[last_end..full_match.start()];
        if !text_segment.is_empty() {
            segments.push(Segment::Text(text_segment));
        }

        let diagram_type = cap[1].to_lowercase();
        let code = cap[2].trim().to_string();
        segments.push(Segment::CodeBlock { diagram_type, code });
        last_end = full_match.end();
    }

    // 添加最后的文本片段
    let remaining_text = &input[last_end..];
    if !remaining_text.is_empty() {
        segments.push(Segment::Text(remaining_text));
    }

    // 第二步：并行处理代码块
    let processed: Vec<Cow<str>> = segments.into_par_iter().map(|seg| {
        match seg {
            Segment::Text(t) => Cow::Borrowed(t),
            Segment::CodeBlock { diagram_type, code } => {
                // 每个线程使用独立的Client
                let client = reqwest::blocking::Client::new();
                let url = format!("https://kroki.io/{}/svg", diagram_type);
                
                match client.post(&url)
                    .body(code)
                    .header("Content-Type", "text/plain")
                    .send()
                    .and_then(|r| r.error_for_status())
                    .and_then(|r| r.text()) 
                {
                    Ok(svg) => Cow::Owned(svg),
                    Err(e) => {
                        eprintln!("diagram render failed ({}): {}", diagram_type, e);
                        Cow::Owned(format!("<!-- diagram render failed: {} -->", diagram_type))
                    }
                }
            }
        }
    }).collect();

    // 第三步：拼接最终结果
    Ok(processed.into_iter().collect())
}

/// 使用示例
fn main() -> anyhow::Result<()> {

    let code = r#"
```kroki-graphviz
digraph G {
a -> b [dir=both color="red:blue"]
c -> d [dir=none color="green:red;0.25:blue"]
}
```

## kroki-wavedrom

```kroki-wavedrom
{ signal: [
{ name: "clk",         wave: "p.....|..." },
{ name: "Data",        wave: "x.345x|=.x", data: ["head", "body", "tail", "data"] },
{ name: "Request",     wave: "0.1..0|1.0" },
{},
{ name: "Acknowledge", wave: "1.....|01." }
]}
```
## test1

```markdown
test1
```

## test2

```bob
    test2
```
    "#;

    let result = render_kroki_blocks(code)?;
    println!("{}", result);
    Ok(())
}