#![doc = include_str!("../README.md")]
// add md_kroki folder
mod md_kroki;

use anyhow::{anyhow, bail, Result};
use futures::Future;
use md_kroki::MdKroki;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use std::path::PathBuf;
use std::pin::Pin;

/// 主函数，使用mdbook预处理器样板启动Kroki预处理
fn main() {
    boilerplate::run(
        KrokiPreprocessor,
        "An mdbook preprocessor for rendering kroki diagrams",
    );
}

/// Kroki预处理结构体
pub struct KrokiPreprocessor;

impl Preprocessor for KrokiPreprocessor {
    /// 预处理器名称
    fn name(&self) -> &'static str {
        "kroki-preprocessor"
    }

    /// 主处理逻辑
    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        // 获取Kroki端点配置
        let endpoint = if let Some(v) = ctx
            .config
            .get_preprocessor(self.name())
            .and_then(|config| config.get("endpoint"))
        {
            if let Some(s) = v.as_str() {
                let mut url = s.to_string();
                if !url.ends_with('/') {
                    url.push('/');
                }
                url
            } else {
                bail!("endpoint must be a string")
            }
        } else {
            "https://kroki.io/".to_string()
        };

        let source_root = &ctx.config.book.src;
        let book_root = ctx.root.clone();

        // 创建渲染器工厂闭包
        let renderer_factory = move |chapter_path: Option<PathBuf>| {
            let source_root = source_root.clone();
            let book_root = book_root.clone();
            let chapter_parent_path = chapter_path.map(|mut p| {
                p.pop();
                p
            });
            
            MdKroki::builder()
                .endpoint(endpoint.clone())
                .path_and_root_resolver(move |mut path, root: Option<&str>| {
                    // 根据root配置解析文件路径
                    let full_path = match root {
                        Some("system") => {
                            if path.is_relative() {
                                bail!("cannot use relative path with root=\"system\"");
                            }
                            path
                        }
                        Some("book") => {
                            if path.is_absolute() {
                                path = path.strip_prefix("/")?.into();
                            }
                            book_root.join(path)
                        }
                        Some("source" | "src") => {
                            if path.is_absolute() {
                                path = path.strip_prefix("/")?.into();
                            }
                            book_root.join(&source_root).join(path)
                        }
                        None | Some("this" | ".") => {
                            if path.is_absolute() {
                                bail!(r#"cannot use absolute path without setting `root` attribute to "system", "book", or "source""#);
                            }
                            book_root
                                .join(&source_root)
                                .join(
                                chapter_parent_path.as_deref().ok_or_else(|| anyhow!("cannot use local relative file references in chapters with no source path."))?
                                )
                                .join(path)
                        }
                        Some(other) => bail!("unrecognized root type: {other}")
                    };

                    Ok(std::fs::read_to_string(full_path)?)
                })
                .build()
        };

        // 收集所有渲染任务
        let mut index_stack = vec![];
        let render_futures =
            extract_render_futures(&mut book.sections, &mut index_stack, &renderer_factory);

        // 创建多线程运行时并执行所有任务
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create multi-threaded runtime");

        let rendered_files = rt.block_on(async {
            futures::future::join_all(render_futures.into_iter()).await
        }).into_iter().collect::<Result<Vec<RenderedFile>>>()?;

        // 更新处理后的内容到书籍
        for file in rendered_files {
            let chapter = get_chapter(&mut book.sections, &file.indices);
            chapter.content = file.content;
        }

        Ok(book)
    }

    /// 支持的渲染器类型
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

/// 递归收集所有章节的渲染任务
fn extract_render_futures<'a>(
    items: impl IntoIterator<Item = &'a mut BookItem> + 'a,
    indices: &mut Vec<usize>,
    renderer_factory: &'a impl Fn(Option<PathBuf>) -> MdKroki,
) -> Vec<Pin<Box<dyn Future<Output = Result<RenderedFile>> + 'a>>> {
    let mut files = Vec::new();
    indices.push(0);
    for (index, item) in items.into_iter().enumerate() {
        if let BookItem::Chapter(ref mut chapter) = item {
            let chapter_source = chapter.source_path.clone();
            let chapter_content = chapter.content.split_off(0);
            *indices.last_mut().unwrap() = index;
            let indices_clone = indices.clone();
            
            // 递归处理子章节
            files.extend(extract_render_futures(
                &mut chapter.sub_items,
                indices,
                renderer_factory,
            ));
            
            // 为当前章节创建渲染任务
            files.push(Box::pin(async move {
                let renderer = renderer_factory(chapter_source);
                let new_content = renderer.render(chapter_content).await?;
                Ok(RenderedFile {
                    indices: indices_clone,
                    content: new_content,
                })
            }));
        }
    }
    indices.pop();
    files
}

/// 根据索引路径获取对应章节的可变引用
fn get_chapter<'a>(mut items: &'a mut Vec<BookItem>, indices: &Vec<usize>) -> &'a mut Chapter {
    for index in &indices[..indices.len() - 1] {
        let item = items.get_mut(*index).expect("index disappeared");
        match item {
            BookItem::Chapter(ref mut chapter) => items = &mut chapter.sub_items,
            _ => panic!("indexed book item wasn't a chapter"),
        }
    }
    match items
        .get_mut(*indices.last().unwrap())
        .expect("chapter not found")
    {
        BookItem::Chapter(ref mut chapter) => chapter,
        _ => panic!("indexed book item wasn't a chapter"),
    }
}

/// 渲染结果结构，包含章节索引和处理后的内容
struct RenderedFile {
    indices: Vec<usize>,
    content: String,
}