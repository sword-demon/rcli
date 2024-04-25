use anyhow::Result;
use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_text_sign,
    process_text_verify, Base64SubCommand, Opts, SubCommand,
};

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
        SubCommand::GenPass(opts) => process_genpass(
            opts.length,
            opts.uppercase,
            opts.lowercase,
            opts.number,
            opts.symbol,
        )?,
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => process_encode(&opts.input, opts.format)?,
            Base64SubCommand::Decode(opts) => process_decode(&opts.input, opts.format)?,
        },
        SubCommand::Text(subcmd) => match subcmd {
            rcli::TextSubCommand::Sign(opts) => {
                process_text_sign(&opts.input, &opts.key, opts.format)?
            }
            rcli::TextSubCommand::Verify(opts) => {
                process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?
            }
        },
    }
    Ok(())
}
