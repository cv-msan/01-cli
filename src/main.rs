use clap::Parser;
use rcli::process_csv;
use rcli::{Opts, SubCommand};
// rcli csv -i input.csv -o output.json
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        //这里的opts是随便定义的代表的CsvOpts实例变量
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}
