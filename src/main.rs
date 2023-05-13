use clap::{Parser, Subcommand};
use zipSniper::ZipSniper;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use log::{debug, error, info, trace, Level, LevelFilter};
use std::str::FromStr;

mod cd;
mod eocd;

#[derive(Parser, Debug)]
#[command(name = "zipSniper")]
#[command(author = "Michael Forret <michael.forret@quorumcyber.com>")]
#[command(version = "0.1")]
#[command(about = "Extracts a file list within a zip archive remotely", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "url")]
    path: String,

    #[arg(
        short,
        long,
        default_value_t = 56,
        value_name = "BYTES",
        help = "Number of bytes to pull from the end of the file.\nThe EOCD checksum needs to land in this data chunk.\nZIP:0x06054b50 \tZIP64:0x06054b50\n"
    )]
    comment_buffer: u64,

    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Sets an optional output file"
    )]
    output_file: Option<String>,

    #[arg(
        short,
        long,
        default_value_t = String::from("info"),
        value_name("LEVEL"),
        help("Sets the log level (error, warn, info, debug, trace)"),
    )]
    log_level: String,

    #[arg(
        long,
        value_name("PROXY_URL"),
        help("Sets the proxy to route HTTP requests through"),
    )]
    proxy: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    //Set up logging
    let log_level = Level::from_str(&args.log_level).unwrap_or(Level::Info);
    env_logger::Builder::new()
        .filter(None, log_level.to_level_filter())
        .init();

    let sniper = ZipSniper::new(args.path, args.proxy);
    let cd_list = sniper.run(args.comment_buffer).await;

    if args.output_file.is_some() {
        let file = File::create(args.output_file.unwrap()).unwrap();
        let mut buf_writer = BufWriter::new(file);

        for cd in cd_list.iter() {
            writeln!(buf_writer, "{}", cd.file_name().unwrap());
        }

        buf_writer.flush().unwrap();
    } else {
        for cd in cd_list.iter() {
            println!("{}", cd.file_name().unwrap());
        }
    }
}
