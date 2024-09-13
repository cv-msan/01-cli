use clap::Parser;
use std::{fmt, path::Path, str::FromStr};
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
    #[command(name = "genpass", about = "generate a random password")]
    GenPass(GenPassOpts),
}
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("文件不存在".into())
    }
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
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long,value_parser=parser_format,default_value="json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}
fn parser_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    //parse里调用的是FromStr::from_str(self)，这样就会用下方的给OutputFormat实现的from_str
    format.parse::<OutputFormat>()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}
//这里给 OutputFormat实现了FromStr 里的from_str方法,
//所以在上边的parser_format方法里调用format.parse的时候会执行下方重写的方法
impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),

            v => anyhow::bail!("不支持{}的格式", v),
        }
    }
}
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
