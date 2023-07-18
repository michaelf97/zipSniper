pub mod zip;

use reqwest;
use zip::*;

pub struct Client {
    remote_path: String,
    comment_buffer: u64,
    client: reqwest::Client,
}

impl Client {
    pub async fn new(
        remote_path: String,
        comment_buffer: u64,
        proxy_address: Option<String>,
    ) -> Client {
        Client {
            remote_path,
            comment_buffer,
            client: Client::build_client(proxy_address),
        }
    }

    fn build_client(proxy_address: Option<String>) -> reqwest::Client {
        proxy_address
            .map(|proxy_address| {
                let proxy = if proxy_address.starts_with("http://") {
                    reqwest::Proxy::http(proxy_address)
                } else if proxy_address.starts_with("https://") {
                    reqwest::Proxy::https(proxy_address)
                } else {
                    reqwest::Proxy::all(proxy_address)
                }
                .expect("Invalid proxy");

                reqwest::Client::builder()
                    .proxy(proxy)
                    .build()
                    .expect("Error building reqwest client")
            })
            .unwrap_or_else(reqwest::Client::new)
    }

    pub async fn build_zip(&self) -> Zip {
        let response = self
            .client
            .get(&self.remote_path)
            .header("Range", format!("bytes=-{}", &self.comment_buffer))
            .send()
            .await
            .expect("Error sending EOCD chunk request");

        let data = response.bytes().await.expect("Error pulling EOCD chunk");

        for (index, window) in data.windows(zip::eocd::SIGNATURE_32.len()).enumerate() {
            if window == zip::eocd::SIGNATURE_32 || window == zip::eocd::SIGNATURE_64 {
                return Zip::build(&data[index..]);
            }
        }
        panic!("EOCD Signature not found. Increase comment buffer");
    }
}
