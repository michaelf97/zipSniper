use bytes::{Buf, Bytes};
use reqwest::{Client, Response, StatusCode, Proxy};
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use log::{debug, error, info, trace, Level, LevelFilter};
use tokio_socks::{tcp::Socks5Stream, Error as SocksError, TargetAddr};
use tokio_tungstenite::client_async;


mod cd;
mod eocd;

pub struct ZipSniper {
    path: String,
    client: reqwest::Client,
}

impl ZipSniper {
    pub fn new(path: String, proxy_url: Option<String> ) -> Self {
        Self {
            path,
            client: Client::new(),
        }
    }

    pub async fn run(&self, comment_buffer: u64) -> Vec<cd::Cd> {
        debug!("running zipSniper against remote file: {}", {&self.path});
        let buffer = self.get_buffer(comment_buffer).await;
        let eocd = self.parse_out_eocd(buffer.unwrap());

        let cd_count = eocd.total_number_of_central_directory_records();
        let cd_offset = eocd.offset_of_start_of_central_directory();
        let cd_size = eocd.size_of_central_directory();

        let cd = self.get_cd(cd_size, cd_offset).await;
        let cd_list = self.parse_out_cds(cd.unwrap());

        if cd_list.len() as u64 != eocd.number_of_central_directory_records_on_this_disk() {
            error!("[WARNING] - Missing {} CD Records!", {
                eocd.number_of_central_directory_records_on_this_disk() - cd_list.len() as u64
            });
        }

        cd_list
    }

    async fn get_buffer(&self, comment_buffer: u64) -> Result<Bytes, ZipSniperError> {
        /*
        Grabs the last <comment_buffer> bytes of the ZIP archive self.path

        The EOCD is the last data structure in a ZIP archive with the start of this structure is identified
            with a signature.
        0x06054b50 with offset of atleast -22 Bytes + N
        0x06064b50 with offset of atleast -56 Bytes + N
        Where N is the size of the comment.
        We grab a small chunk near the end of the ZIP with a HTTP GET request using
            the Range header and hope the signature is present in the response.

        # Arguments
        * `comment_buffer: u64` The size of the chunk to take from the end of the
            archive.
        */

        debug!("pulling the last {} bytes from the file {}", comment_buffer, &self.path);
        let response = self
            .client
            .get(&self.path)
            .header("Range", format!("bytes=-{}", comment_buffer))
            .send()
            .await
            .map_err(ZipSniperError::HttpError)?;

        let status = response.status();
        if !status.is_success() {
            return Err(ZipSniperError::InvalidStatusCode(status));
        }
        
        let data = response.bytes().await.unwrap();
        debug!("Last {} bytes pulled from {}", data.len(), &self.path);
        Ok(data)
    }

    fn parse_out_cds(&self, buffer: Bytes) -> Vec<cd::Cd> {
        /*
        Giving a string of bytes like below where A is a data signature
        A B C D E A B C D E F A B C A,

        This function will split the string of bytes into sections where each section
        starts with A.
        */

        let signature: [u8; 4] = [0x50, 0x4b, 0x01, 0x02];

        let mut data_structures = Vec::new();
        let mut start_indices = Vec::new();
        //Find the start indices of the signature in the buffer
        debug!("CD Blob is {} bytes in size", buffer.len());
        for (index, win) in buffer.windows(signature.len()).enumerate() {
            if win == signature {
                start_indices.push(index);
            }
        }

        //Extract the cd for the buffer using the calculated indices
        for i in 0..start_indices.len() {
            let start = start_indices[i];
            let end = if i < start_indices.len() - 1 {
                start_indices[i + 1]
            } else {
                buffer.len()
            };

            let data_structure = buffer.slice(start..end);
            let cd = cd::Cd::from(data_structure);
            data_structures.push(cd);
        }
        data_structures
    }

    fn parse_out_eocd(&self, buffer: Bytes) -> eocd::Eocd {
        /*
        Looks for the start of the EOCD signature and returns the EOCD.

        The signatures to look for are:
        0x06054b50 with offset of atleast -22 Bytes + N
        0x06064b50 with offset of atleast -56 Bytes + N
        Where N is the size of the comment.

        # Arguments
        * `buffer: Bytes` The chunk of data to look in for the eocd data structure
        */
        const SIGNATURE_32: &[u8] = &[0x50, 0x4b, 0x05, 0x06];
        const SIGNATURE_64: &[u8] = &[0x50, 0x4b, 0x06, 0x06];

        debug!("Checking for the signatures {:?} or {:?} within the {} bytes", SIGNATURE_32, SIGNATURE_64, buffer.len());
        let pos = match ZipSniper::get_signature_position_reverse(SIGNATURE_32, &buffer) {
            Some(n) => n,
            None => match ZipSniper::get_signature_position_reverse(SIGNATURE_64, &buffer) {
                Some(m) => m,
                None => panic!("{}", ZipSniperError::EocdSignatureNotFound),
            },
        };

        debug!("Signature {:?} found at offset {}", buffer.slice(pos..pos+4), pos);
        eocd::Eocd::from(buffer.slice(pos..))
    }

    fn get_signature_position_reverse(signature: &[u8], data: &Bytes) -> Option<usize> {
        /*
        Returns index position of the supplied signature within the supplied Bytes object.

        Given a sequence of bytes, this function will return the index position
        of the sub-sequence of bytes. Useful to look for a signature signifying the start
        of a data structure.

        # Arguments
        * `signature: &[u8]` the signature to look for
        * `data: Bytes` the sequence of bytes to look in for the subset of bytes
        */
        data.windows(signature.len())
            .position(|window| window == signature)
    }

    async fn get_cd(&self, cd_size: u64, cd_offset: u64) -> Result<Bytes, ZipSniperError> {
        /*
        Grabs the Central Directory of the ZIP Archive hosted at self.path

        The central directory (cd) is a small section of data that contains a listing of
        all the files within the archive. Knowing the offset and the size of the cd, we can snipe
        it out with the Range header
        */

        debug!("Grabbing the CD blob between offsets {} and {}", cd_offset, cd_offset + cd_size);
        let response = self
            .client
            .get(&self.path)
            .header(
                "Range",
                format!("bytes={}-{}", cd_offset, cd_offset + cd_size),
            )
            .send()
            .await
            .map_err(ZipSniperError::HttpError)?;

        let status = response.status();
        if !status.is_success() {
            return Err(ZipSniperError::InvalidStatusCode(status));
        }

        let data = response.bytes().await.unwrap();
        debug!("Last {} bytes pulled from {}", data.len(), &self.path);
        Ok(data)
    }
}

#[derive(Debug)]
enum ZipSniperError {
    HttpError(reqwest::Error),
    InvalidStatusCode(StatusCode),
    EocdSignatureNotFound,
}

impl Error for ZipSniperError {}

impl Display for ZipSniperError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ZipSniperError::HttpError(err) => write!(f, "HTTP Error: {}", err),
            ZipSniperError::InvalidStatusCode(status) => {
                write!(f, "Invalid Status Code: {}", status)
            }
            ZipSniperError::EocdSignatureNotFound => {
                write!(f, "The comment buffer applied is too small!")
            }
        }
    }
}
