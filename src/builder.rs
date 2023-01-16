use std::path::PathBuf;
use std::time::Duration;
use std::{fs, io};

use reqwest::blocking::ClientBuilder;

use crate::{Downloader, Error};

/// A builder for constructing a [`Downloader`]
///
/// This allows you to for example set a custom directory to cache files in or
/// configure the internally used [`reqwest::Client`].
///
/// ```
/// use data_downloader::{Downloader, DownloaderBuilder};
/// use reqwest::blocking::ClientBuilder;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let dl: Downloader = DownloaderBuilder::new()
///     .retry_failed_download(false)
///     .client_builder(
///         ClientBuilder::new()
///             .user_agent("My custom User agent")
///             .min_tls_version(reqwest::tls::Version::TLS_1_2)
///             .https_only(true),
///     )
///     .build()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct DownloaderBuilder {
    client: ClientBuilder,
    retry_failed_download: bool,
    storage_dir: Option<PathBuf>,
}

fn default_storage_dir() -> io::Result<PathBuf> {
    const LIBRARY_DIR_NAME: &str = "data_downloader_default_storage_directory";

    let mut cache_dir = dirs::cache_dir().unwrap_or_else(std::env::temp_dir);
    cache_dir.push(LIBRARY_DIR_NAME);
    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

impl DownloaderBuilder {
    /// Create a new builder
    ///
    /// This is the same as calling [`Downloader::builder()`]
    pub fn new() -> Self {
        DownloaderBuilder {
            client: ClientBuilder::new().timeout(None),
            retry_failed_download: true,
            storage_dir: None,
        }
    }

    // Create a [`Downloader`] that saves to a custom storage directory
    /// you have to ensure the directory exists
    pub fn storage_dir(mut self, dir: PathBuf) -> Self {
        self.storage_dir = Some(dir);
        self
    }

    /// Set the timeout for http requests. By default there is no timeout
    pub fn timeout<T>(mut self, timeout: T) -> Self
    where
        T: Into<Option<Duration>>,
    {
        self.client = self.client.timeout(timeout);
        self
    }

    /// Set the reqwest [`ClientBuilder`]
    ///
    /// This allows you to configure everything about the [`reqwest::Client`]
    /// that will be used internally by the [`Downloader`]. Note that
    /// setting the [`ClientBuilder`] after setting the timeout with
    /// [`Self::timeout`] will overwrite that with the timeout value configured
    /// in the `builder`.
    pub fn client_builder(mut self, builder: ClientBuilder) -> Self {
        self.client = builder;
        self
    }

    /// Configure wheter the [`Downloader`] should retry a download if it fails
    pub fn retry_failed_download(mut self, retry: bool) -> Self {
        self.retry_failed_download = retry;
        self
    }

    /// Construct a [`Downloader`] from this builder
    pub fn build(self) -> Result<Downloader, Error> {
        let storage_dir = match self.storage_dir {
            Some(dir) => dir,
            None => default_storage_dir()?,
        };

        Ok(Downloader {
            storage_dir,
            retry_failed_download: self.retry_failed_download,
            client: self.client.build()?,
        })
    }
}
