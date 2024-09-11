use clap::Parser;
use rcli::process_csv;
use rcli::{Opts, SubCommand};
// rcli csv -i input.csv -o output.json
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        //这里的opts是随便定义的代表的CsvOpts实例变量
        SubCommand::Csv(opts) => {
            //此处返回值为Result类型所以要?处理掉
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
