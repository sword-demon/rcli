use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            // String 所以这里使用引用类型 去借用一下
            process_csv(&opts.input, &opts.output)?;
        }
    }
    Ok(())
}
