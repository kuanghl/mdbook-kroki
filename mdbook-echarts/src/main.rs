mod echarts;

use std::io;
use std::process;

use clap::{App, Arg, ArgMatches, SubCommand};

use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};

use env_logger::Builder;
use log::LevelFilter;

use echarts::ECharts;

pub fn make_app() -> App<'static, 'static> {
    // 使用 clap 库构建命令行接口
    // ./mdbook-echarts --help
    App::new("echarts-preprocessor")
        .about("A mdbook preprocessor to auto generate book summary")
        .subcommand(    // 检查渲染器支持
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true)) 
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .subcommand(    // 显示帮助
            SubCommand::with_name("help")
                .about("help doc for use mdbook-echarts preprocessor"),
        )
}

fn main() {
    Builder::new()
        .filter_level(LevelFilter::Info)    // 强制设置默认级别为 info
        .init();                            // 初始化日志系统

    let matches = make_app().get_matches();

    let preprocessor = ECharts::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Some(_sub_args) = matches.subcommand_matches("help") {
        println!("please use mdbook-echarts as a preprocessor in your book.toml");
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = pre.supports_renderer(&renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
