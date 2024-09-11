use clap::Parser;
use std::path::Path;
#[derive(Debug, Parser)]
#[command(name = "rcli", version,author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "show csv or convert CSV to others")]
    Csv(CsvOpts),
}
//在 Rust 编程语言中，#[derive(Debug, Parser)] 是属性宏（attribute macro）的使用示例，
//它指示编译器自动为结构体或枚举类型实现特定的 trait。
//Debug 是 Rust 标准库中的一个 trait，它允许类型通过 fmt::Debug 格式化器进行格式化，通常用于打印和调试。
//当你为类型加上 #[derive(Debug)] 属性后，Rust 编译器会自动为你的类型生成 fmt::Debug 的实现，
//这样你就可以使用 {:?} 或 {:#?} 占位符来打印该类型的实例。
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser = verify_input_file)]
    pub input: String,
    //字符串类型的就用default_value
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("文件不存在".into())
    }
}
