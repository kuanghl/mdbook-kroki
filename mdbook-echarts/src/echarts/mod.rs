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

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                chapter.content = gen(chapter.content.as_str())
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}


pub fn gen(content: &str) -> String {
    let mut s = String::from(content);

    const TAG_START_1: &str = "```echarts";
    const TAG_END_1: &str = "```";
    // let re = Regex::new(r"(?m)^```chart((.*\n)+?)?```$").unwrap();
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
    for s in empty_str_vec {
        mat_string = mat_string.replace(s, "");
    }

    let v = String::from(format!("chart_{}", Uuid::new_v4()));
    let ids = v.split_at(14);
    let id = ids.0;
    // let div = format!("<div id=\"{}\" style=\"width: 600px;height:400px;\"></div>", id);
    let div = format!("<div id=\"{}\" style=\"height: 500px;\"></div>", id);
    let echarts_src = format!("document.addEventListener('DOMContentLoaded', function() {{\nvar my{} = echarts.init(document.getElementById('{}'), null, {{\nrenderer: 'svg', \nuseDirtyRect: false\n}});\nvar option = {}\nmy{}.setOption(option);\n}})", id, id, mat_string.trim(), id);
    let script = format!("<script type=\"text/javascript\">\n{};\n</script>", echarts_src);
    let buf = format!("<div>\n{}\n{}\n</div>", div, script);
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
