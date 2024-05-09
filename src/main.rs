use anyhow::Result;
use clap::Parser;
use rcli::{CmdExector, Opts};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();
    opts.cmd.execute().await?;
    // match opts.cmd {
    //     SubCommand::Csv(opts) => {
    //         // output 里有值就拿出来，否则就使用默认值 output.json
    //         let output = if let Some(output) = opts.output {
    //             output.clone()
    //         } else {
    //             // 要实现对应的 fmt::Display trait
    //             format!("output.{}", opts.format)
    //         };
    //         // String 所以这里使用引用类型 去借用一下
    //         process_csv(&opts.input, output, opts.format)?;
    //     }
    //     SubCommand::GenPass(opts) => {
    //         let pass = process_genpass(
    //             opts.length,
    //             opts.uppercase,
    //             opts.lowercase,
    //             opts.number,
    //             opts.symbol,
    //         )?;
    //         println!("pass: {}", pass);
    //     }
    //     SubCommand::Base64(subcmd) => match subcmd {
    //         Base64SubCommand::Encode(opts) => {
    //             let encoded = process_encode(&opts.input, opts.format)?;
    //             println!("encoded: {}", encoded);
    //         }
    //         Base64SubCommand::Decode(opts) => {
    //             let decoded = process_decode(&opts.input, opts.format)?;
    //             let decoded = String::from_utf8(decoded)?;
    //             println!("decoded: {}", decoded);
    //         }
    //     },
    //     SubCommand::Text(subcmd) => match subcmd {
    //         rcli::TextSubCommand::Sign(opts) => {
    //             let sign = process_text_sign(&opts.input, &opts.key, opts.format)?;
    //             println!("sign: {}", sign);
    //         }
    //         rcli::TextSubCommand::Verify(opts) => {
    //             let valid = process_text_verify(&opts.input, &opts.key, &opts.sig, opts.format)?;
    //             println!("{}", valid);
    //         }
    //         rcli::TextSubCommand::Generate(opts) => {
    //             let key = process_generate(opts.format)?;
    //             match opts.format {
    //                 TextSignFormat::Ed25519 => {
    //                     let name = &opts.output;
    //                     fs::write(name.join("ed25519.sk"), &key[0])?;
    //                     fs::write(name.join("ed25519.pk"), &key[1])?;
    //                 }
    //                 TextSignFormat::Blake3 => {
    //                     let name = opts.output.join("blake3.txt");
    //                     fs::write(name, &key[0])?;
    //                 }
    //             }
    //             println!("{:?}", opts);
    //         }
    //     },
    //     SubCommand::Http(cmd) => match cmd {
    //         HttpSubCommand::Serve(opts) => {
    //             process_http_serve(opts.dir, opts.port).await?;
    //         }
    //     },
    // }
    Ok(())
}
