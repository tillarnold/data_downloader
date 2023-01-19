#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![deny(missing_debug_implementations)]

//! This crate provides a simple way to download files.
//! In particular this crate aims to make it easy to download and cache files
//! that do not change over time, for example reference image files, ML models,
//! example audio files or common password lists.
//!
//! # Downloading a file
//! As an example: To download the plaintext version of RFC 2068 you construct a
//! [`DownloadRequest`] with the URL and SHA-256 checksum and then use the
//! [`get`] function.
//!
//! If you know that the file was already downloaded you can use [`get_cached`].
//! ```
//! use data_downloader::{get, get_cached, DownloadRequest};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Define where to get the file from
//! let rfc_link = &DownloadRequest {
//!     url: "https://www.rfc-editor.org/rfc/rfc2068.txt",
//!     name: "rfc2068.txt",
//!     sha256_hash: &hex_literal::hex!(
//!         "D6C4E471389F2D309AB1F90881576542C742F95B115336A346447D052E0477CF"
//!     ),
//! };
//!
//! // Get the binary contents of the file
//! let rfc: Vec<u8> = get(rfc_link)?;
//!
//! // Convert the file to a String
//! let as_text = String::from_utf8(rfc)?;
//! assert!(as_text.contains("The Hypertext Transfer Protocol (HTTP) is an application-level"));
//! assert!(as_text.contains("protocol for distributed, collaborative, hypermedia information"));
//! assert!(as_text.contains("systems."));
//!
//! // Get the binary contents of the file directly from disk
//! let rfc: Vec<u8> = get_cached(rfc_link)?;
//! # let as_text = String::from_utf8(rfc)?;
//! # assert!(as_text.contains("The Hypertext Transfer Protocol (HTTP) is an application-level"));
//! # assert!(as_text.contains("protocol for distributed, collaborative, hypermedia information"));
//! # assert!(as_text.contains("systems."));
//! # Ok(())
//! # }
//! ```
//!
//!
//! [`get_path`] can be used to get a [`PathBuf`] to the file.
//!
//! One of the design goals of this crate is to verify the integrity of the
//! downloaded files, as such the SHA-256 checksum of the downloads are checked.
//! If a file is loaded from the cache on disk the SHA-256 checksum is also
//! verified. However for [`get_path`] the checksum is not verified because even
//! if it was you would still be vulnerable to a [TOC/TOU vulnerability](https://en.wikipedia.org/wiki/Time-of-check_to_time-of-use).
//!
//! The [`get`], [`get_cached`] and [`get_path`] function use a default
//! directory to cache the downloads, this allows multiple application to share
//! their cached downloads. If you need more configurability you can use
//! [`DownloaderBuilder`] and set the storage directory manually using
//! [`DownloaderBuilder::storage_dir`]. The default storage directory is a
//! platform specific cache directory or a platform specific temporary directory
//! if the cache directory is not available.
//!
//! # Included [`DownloadRequest`]s
//! The [`files`] module contains some predefined [`DownloadRequest`] for your
//! convenience.
//!
//! # Pitfalls
//! When manually changing a [`DownloadRequest`], inherently the SHA-256 sum
//! needs to be changed too. If this is not done this can result in a
//! [`DownloadRequest`] that looks as if it is downloading a specific file but
//! the download will never succeed because of the checksum mismatch, however
//! the wrong file can be loaded from cache. For example here the above
//! [`DownloadRequest`] was changed but only the `url` was adapted. Since
//! neither the `name` nor `sha256_hash` are set to the correct value this will
//! return `rfc2068.txt` from the cache. This is a user error, as the developer
//! has to ensure that they specify the correct SHA-256 checksum for a
//! [`DownloadRequest`].
//!
//! ```
//! # use data_downloader::{get, get_cached, DownloadRequest};
//! &DownloadRequest {
//!     url: "https://www.rfc-editor.org/rfc/rfc7168.txt",
//!     name: "rfc2068.txt",
//!     sha256_hash: &hex_literal::hex!(
//!         "D6C4E471389F2D309AB1F90881576542C742F95B115336A346447D052E0477CF"
//!     ),
//! };
//! ```
//!
//! # Status of this crate
//! This is an early release. As such breaking changes are expected at some
//! point. There are also some implementation limitations including but not
//! limited to:
//! - The downloading is rather primitive. Failed downloads are simply retried a
//!   fixed number of times and no continuation of interrupted downloads is
//!   implemented.
//! - Only one URL is used per [`DownloadRequest`], it's not currently possible
//!   to specify multiple possible locations for a file.
//! - Only single files are supported, no unpacking of archives is supported.
//! - The crate uses blocking IO. As such there is no currently no WASM support.
//! - If a file on disk is corrupted or changed in another way it will not be
//!   automatically redownloaded.
//!
//! Contributions to improve this are welcome.
//!
//! Nevertheless this crate should be suitable for simple use cases.
//!
//! ## Dependencies
//! This crate uses the following dependencies:
//! - `dirs` to find platform specific temporary and cache directories
//!     - Implementing this manually would only cause incompatibilities
//! - `reqwest` to issue HTTP requests
//!     - A HTTP library is definitely required to allow this crate to download
//!       files. `reqwest` is widely used in the Rust community, it is however a
//!       rather big dependency as it is very fully featured. It might be worth
//!       investigating smaller HTTP client libraries in the future.
//! - `sha2` to hash files
//!     - To ensure the integrity of the files a collision resistant
//!       cryptographic hash function is required. SHA-256 is generally
//!       considered as the standard for such a use case. The `sha2` crate by
//!       the `RustCrypto` organization is the defacto standard implementation
//!       of SHA-2 for Rust.
//! - `hex-literal` to conveniently specify the SHA-256 sums
//!     - Technically this dependency could be removed if we specified the
//!       SHA-256 in the predefined [`DownloadRequest`] directly as `&[u8]`
//!       slice literals. However the library is maintained by the `RustCrypto`
//!       organization and as such can be regarded as trustworthy
//! - `thiserror` to conveniently derive `Error`
//!     - This library is also very widely used and maintained by David Tolnay ,
//!       a highly regarded member of the Rust community. Once `data_downloader`
//!       has sufficiently matured it might be a good idea to stop using
//!       `thiserror` and instead directly use the generated implementations in
//!       the code. This would potentially reduce build times. This has however
//!       low priority, especially while the [`enum@crate::Error`] type is still
//!       changing frequently.

use std::io::{self, Cursor};
use std::path::PathBuf;

use thiserror::Error;

mod builder;
mod downloader;
pub mod files;
mod utils;

pub use builder::DownloaderBuilder;
pub use downloader::Downloader;
use zip::ZipArchive;

/// A file to be downloaded
#[derive(Debug)]
pub struct DownloadRequest<'a> {
    /// URL the file is at
    pub url: &'a str,
    /// Expected SHA-256 checksum
    pub sha256_hash: &'a [u8],
    /// The name of the file.
    /// If two files with the same SHA-256 but different names are
    /// reqeuested this will result in two separate downloads
    pub name: &'a str,
}

/// A file in a zip to be downloaded
#[derive(Debug)]
pub struct InZipDownloadRequest<'a> {
    /// Path inside the zip
    pub path: &'a str,
    /// Expected SHA-256 checksum
    pub sha256_hash: &'a [u8],
    /// The name of the file.
    /// If two files with the same SHA-256 but different names are
    /// reqeuested this will result in two separate downloads
    pub name: &'a str,
    /// The zip this is in
    pub parent: &'a DownloadRequest<'a>,
}

/// A thing that can be downloaded
pub trait Downloadable {
    /// name of the file for user
    fn file_name(&self) -> &str;
    /// Expected sha256
    fn sha256(&self) -> &[u8];
    /// Download this or extract it
    fn compute(&self, client: &Downloader) -> Result<Vec<u8>, Error>;
}

impl Downloadable for DownloadRequest<'_> {
    fn file_name(&self) -> &str {
        return self.name;
    }

    fn sha256(&self) -> &[u8] {
        self.sha256_hash
    }

    fn compute(&self, client: &Downloader) -> Result<Vec<u8>, Error> {
        let response = client.client.get(self.url).send()?;
        let contents = response.bytes()?;

        Ok(contents.to_vec())
    }
}

impl Downloadable for InZipDownloadRequest<'_> {
    fn file_name(&self) -> &str {
        self.name
    }

    fn sha256(&self) -> &[u8] {
        self.sha256_hash
    }

    fn compute(&self, client: &Downloader) -> Result<Vec<u8>, Error> {
        use std::io::Read;

        // TODO we read the entire file because we use get. It's probably ok not to
        // verify the sha of the zip because we verify the sha of the inner file.
        // assuming that we don't suffer from some malicous ZIP attack making the extact
        // take forever also this will do a lot of retires.

        let zip_bytes = client.get(self.parent)?;
        let mut buf = Cursor::new(zip_bytes);
        let mut archive = ZipArchive::new(&mut buf).expect("TODO");

        let files = archive.file_names().collect::<Vec<&str>>();
        println!("files {files:?}");

        let mut fl = archive.by_name(self.path).expect("TODO");
        let mut res = vec![]; //TOOD with expected capacyit

        fl.read_to_end(&mut res).expect("TODO");

        Ok(res)
    }
}

/// Get the file contents and if the file has not yet been downloaded, download
/// it.
///
/// This is equivalent to calling [`Downloader::get`] on the default
/// [`Downloader`]
pub fn get(r: &DownloadRequest) -> Result<Vec<u8>, Error> {
    Downloader::new()?.get(r)
}

/// Get the file contents and *fail* with an IO error if the file is not yet
/// downloaded
///
/// This is equivalent to calling [`Downloader::get_cached`] on the default
/// [`Downloader`]
pub fn get_cached(r: &DownloadRequest) -> Result<Vec<u8>, Error> {
    Downloader::new()?.get_cached(r)
}

/// Computes the full path to the file and if the file has not yet been
/// downloaded, download it.
///
/// This is equivalent to calling [`Downloader::get_path`] on the default
/// [`Downloader`]
pub fn get_path(r: &DownloadRequest) -> Result<PathBuf, Error> {
    Downloader::new()?.get_path(r)
}

/// Error type for `data_downloader`
#[derive(Error, Debug)]
pub enum Error {
    /// A HTTP request failed
    #[error("HTTP Request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("IO failed: {0}")]
    /// An io Error
    Io(#[from] io::Error),
    /// The hash of a downloaded file did not match
    #[error("Wrong hash! Expected {expected} got {was}")]
    DownloadHashMismatch {
        /// The hash that was expected
        expected: String,
        /// The hash that the actual file had
        was: String,
    },
    /// The hash of a file from the on disk cache did not match
    #[error("Wrong hash on disk! Expected {expected} got {was}")]
    OnDiskHashMismatch {
        /// The hash that was expected
        expected: String,
        /// The hash that the actual file had
        was: String,
    },
}

// For testing the readme
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[cfg(test)]
mod test {
    use hex_literal::hex;

    use crate::{DownloadRequest, Downloader, InZipDownloadRequest};

    #[test]
    fn zip_test() {
        let z = InZipDownloadRequest {
            parent: &DownloadRequest {
                url: "https://github.com/tillarnold/data_downloader/archive/refs/tags/v0.1.0.zip",
                sha256_hash: &hex!(
                    "3A1929ABF26E7E03A93206D1D068B36C3F7182F304CF317FD71766113DDA5C4E"
                ),
                name: "data_downloader-v0.1.0.zip",
            },
            path: "data_downloader-0.1.0/src/files/ml/whisper_cpp.rs",
            sha256_hash: &hex!("a6e18802876c198b9b99c33ce932890e57f01e0eab9ec19ac8ab2908025d1ae2"),
            name: "whisper_cpp.rs",
        };

        let dl = Downloader::builder().retry_attempts(0).build().unwrap();

        let res = dl.get(&z).unwrap();
        let str = String::from_utf8(res).unwrap();
        println!("{}", str);
    }
}
