use clap::Parser;
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

    #[arg(
        short,
        long,
        value_name = "SUB-STRING",
        help = "Output filter by filename substring"
    )]
    filter: Option<String>,

    #[arg(
        short,
        long,
        value_name = "SUB-STRING",
        help = "Download filter by filename substring"
    )]
    download: Option<String>,

    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Print all information (could be noisy)"
    )]
    all_information: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = zipSniper::Client::new(cli.remote_path, cli.comment_buffer, cli.proxy_address);

    let zip = client.await.build_zip().await;
    let mut cd_list = zip.cd_list;

    // If a filter is passed, remove any irrelevant CD records.
    if cli.filter.is_some() {
        let filter = cli.filter.unwrap();
        cd_list = cd_list
            .into_iter()
            .filter(|cd| cd.file_name().contains(&filter))
            .collect();
    }

    for cd in cd_list {
        if cli.all_information == true {
            println!("{}", cd)
        } else {
            println!("{}", cd.file_name())
        }
    }
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
