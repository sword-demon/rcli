use anyhow::Result;
use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            // output 里有值就拿出来，否则就使用默认值 output.json
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // 要实现对应的 fmt::Display trait
                format!("output.{}", opts.format)
            };
            // String 所以这里使用引用类型 去借用一下
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}
