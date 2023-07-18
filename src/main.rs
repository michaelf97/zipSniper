use clap::{Parser, Subcommand};
use zipSniper;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // Mandatory path of the remote ZIP file.
    #[arg(value_name = "PATH", help = "Remote path of the ZIP archive.")]
    remote_path: String,

    // Comment buffer
    #[arg(
        short,
        long,
        value_name = "BYTES",
        default_value_t = 56,
        help = "See https://w.wiki/73sX for details."
    )]
    comment_buffer: u64,

    // Proxy address
    #[arg(
        short,
        long,
        value_name = "PATH",
        value_parser = proxy_protocol_validation,
        help = "Address of a proxy (http, https, socks5h)."
    )]
    proxy_address: Option<String>,

    // List of files to download
    #[arg(
        short,
        long,
        value_name = "LIST",
        help = "List of absolute paths of files within the archive to download and extract."
    )]
    file_list: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = zipSniper::Client::new(cli.remote_path, cli.comment_buffer, cli.proxy_address);

    let zip = client.await.build_zip().await;
    println!("{:?}", zip.size_of_cd());
}

fn proxy_protocol_validation(proxy_address: &str) -> Result<String, String> {
    if proxy_address.starts_with("http://")
        || proxy_address.starts_with("https://")
        || proxy_address.starts_with("socks5h://")
    {
        Ok(String::from(proxy_address))
    } else {
        Err(format!(
            "Invalid Proxy Scheme: {}. Check --help",
            proxy_address
        ))
    }
}
