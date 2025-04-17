extern crate regex;
extern crate uuid;

use regex::Regex;
use uuid::Uuid;

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
    // let re = Regex::new(r"(?m)^```chart((.*\n)+?)?```$").unwrap();
    // 匹配两种语法：```echarts 代码块 和 {% echarts %} 标签
    let re = Regex::new(r"```echarts((.*\n)+?)?```").unwrap();

    for mat in re.find_iter(s.clone().as_str()) {

        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_1, TAG_END_1];
        let buf = gen_html(mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    const TAG_START_2: &str = "{% echarts %}";
    const TAG_END_2: &str = "{% endecharts %}";

    // let re = Regex::new(r"(?m)^\{% chart %}((.*\n)+?)?\{% endchart %}$").unwrap();
    let re = Regex::new(r"\{% echarts %}((.*\n)+?)?\{% endecharts %}").unwrap();
    for mat in re.find_iter(s.clone().as_str()) {
        let mat_str = mat.as_str();
        let empty_str_vec = vec![TAG_START_2, TAG_END_2];
        let buf = gen_html(mat_str, empty_str_vec);
        s = s.replace(mat_str, buf.as_str());
    }

    return s;
}

fn gen_html(mat_str: &str, empty_str_vec: Vec<&str>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_gen() {

        let content_raw = r###"
```chart
{
    "data": {
    "type": "bar",
        "columns": [
            ["data1", 30, 200, 100, 400, 150, 250],
            ["data2", 50, 20, 10, 40, 15, 25]
        ],
        "axes": {
            "data2": "y2"
        }
    },
    "axis": {
        "y2": {
            "show": true
        }
    }
}
```

```chart
{
    "data": {
        "type": "bar",
        "columns": [
            ["data1", 30, 200, 100, 400, 150, 250],
            ["data2", 50, 20, 10, 40, 15, 25]
        ],
        "axes": {
            "data2": "y2"
        }
    },
    "axis": {
        "y2": {
            "show": true
        }
    }
}
```

{% chart %}
{
    "data": {
        "type": "bar",
        "columns": [
            ["data1", 30, 200, 100, 400, 150, 250],
        ["data2", 50, 20, 10, 40, 15, 25]
        ],
        "axes": {
            "data2": "y2"
        }
    },
    "axis": {
        "y2": {
            "show": true
        }
    }
}
{% endchart %}

{% chart %}
{
    "data": {
        "type": "bar",
        "columns": [
            ["data1", 30, 200, 100, 400, 150, 250],
        ["data2", 50, 20, 10, 40, 15, 25]
        ],
        "axes": {
            "data2": "y2"
        }
    },
    "axis": {
        "y2": {
            "show": true
        }
    }
}
{% endchart %}
        "###;

        let content_html_target = r###"
<div>
<div id="chart-bbc841c7-369e-462e-9132-08f6cd78cfe0"></div>

<link rel="stylesheet" href="/c3.min.css">
<script src="/d3.min.js"></script>
<script src="/c3.min.js"></script>

<script>
c3.generate(
{"bindto":"#chart-bbc841c7-369e-462e-9132-08f6cd78cfe0",
    "data": {
    "type": "bar",
        "columns": [
            ["data1", 30, 200, 100, 400, 150, 250],
            ["data2", 50, 20, 10, 40, 15, 25]
        ],
        "axes": {
            "data2": "y2"
        }
    },
    "axis": {
        "y2": {
            "show": true
        }
    }
}
);
</script>
</div>

<div>
<div id="chart-450545d5-8552-452d-9865-24e203489872"></div>

<link rel="stylesheet" href="/c3.min.css">
<script src="/d3.min.js"></script>
<script src="/c3.min.js"></script>

<script>
c3.generate(
{"bindto":"#chart-450545d5-8552-452d-9865-24e203489872",
    "data": {
        "type": "bar",
        "columns": [
            ["data1", 30, 200, 100, 400, 150, 250],
            ["data2", 50, 20, 10, 40, 15, 25]
        ],
        "axes": {
            "data2": "y2"
        }
    },
    "axis": {
        "y2": {
            "show": true
        }
    }
}
);
</script>
</div>

<div>
<div id="chart-13cf1dc8-0793-442a-88e0-c9b490f11efb"></div>

<link rel="stylesheet" href="/c3.min.css">
<script src="/d3.min.js"></script>
<script src="/c3.min.js"></script>

<script>
c3.generate(
{"bindto":"#chart-13cf1dc8-0793-442a-88e0-c9b490f11efb",
    "data": {
        "type": "bar",
        "columns": [
            ["data1", 30, 200, 100, 400, 150, 250],
        ["data2", 50, 20, 10, 40, 15, 25]
        ],
        "axes": {
            "data2": "y2"
        }
    },
    "axis": {
        "y2": {
            "show": true
        }
    }
}
);
</script>
</div>

<div>
<div id="chart-243396b1-e28f-4d49-b5c9-7c3d858f0c31"></div>

<link rel="stylesheet" href="/c3.min.css">
<script src="/d3.min.js"></script>
<script src="/c3.min.js"></script>

<script>
c3.generate(
{"bindto":"#chart-243396b1-e28f-4d49-b5c9-7c3d858f0c31",
    "data": {
        "type": "bar",
        "columns": [
            ["data1", 30, 200, 100, 400, 150, 250],
        ["data2", 50, 20, 10, 40, 15, 25]
        ],
        "axes": {
            "data2": "y2"
        }
    },
    "axis": {
        "y2": {
            "show": true
        }
    }
}
);
</script>
</div>
        "###;
        let content_html = gen(content_raw);
        println!("content_html: {}", content_html);

        let re = Regex::new(r"chart-.{36}").unwrap();

        let after_content_html = re.replace_all(content_html.as_str(), "chart-");
        println!("after_content_html: {}", after_content_html);

        let after_content_html_target = re.replace_all(content_html_target, "chart-");
        println!("after_content_html_target: {}", after_content_html_target);

        assert_eq!(after_content_html_target, after_content_html)
    }
}
