use std::num::NonZeroU64;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, io};

use reqwest::blocking::ClientBuilder;

use crate::downloader::InnerDownloader;
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
///     .retry_attempts(7)
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
    retry_attempts: u32,
    storage_dir: Option<PathBuf>,
    retry_wait_time: Duration,
}

fn default_storage_dir() -> io::Result<PathBuf> {
    const DEFAULT_DIR_NAME: &str = "data_downloader_default_storage_directory";

    let mut cache_dir = dirs::cache_dir().unwrap_or_else(std::env::temp_dir);
    cache_dir.push(DEFAULT_DIR_NAME);
    fs::create_dir_all(&cache_dir)?;
    Ok(cache_dir)
}

const DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("CARGO_PKG_REPOSITORY"),
    ") reqwest",
);

impl DownloaderBuilder {
    /// Create a new builder
    ///
    /// This is the same as calling [`Downloader::builder()`]
    pub fn new() -> Self {
        DownloaderBuilder {
            client: ClientBuilder::new()
                .timeout(None)
                .user_agent(DEFAULT_USER_AGENT),
            retry_attempts: 4,
            storage_dir: None,
            retry_wait_time: Duration::from_millis(500),
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

    /// Configure how oftent the [`Downloader`] should retry a download if it
    /// fails
    ///
    /// If set to zero only one request will be sent for each call to
    /// [`Downloader::get`]
    pub fn retry_attempts(mut self, retry: u32) -> Self {
        self.retry_attempts = retry;
        self
    }

    /// Set how long the [`Downloader`] should wait between tries to download
    /// a file
    pub fn retry_wait_time(mut self, time: Duration) -> Self {
        self.retry_wait_time = time;
        self
    }

    /// Construct a [`Downloader`] from this builder
    ///
    /// # Errors
    /// Fails if the [`ClientBuilder`] fails to build or in the case that no
    /// [`Self::storage_dir`] was set and the default storage dir could not be
    /// created or accessed
    pub fn build(self) -> Result<Downloader, Error> {
        let storage_dir = match self.storage_dir {
            Some(dir) => dir,
            None => default_storage_dir()?,
        };

        Ok(Downloader {
            inner: InnerDownloader {
                storage_dir,
                client: self.client.build()?,
                download_attempts: NonZeroU64::new(u64::from(self.retry_attempts) + 1)
                    .expect("Cannot fail because 1 + u64 > 0"),
                failed_download_wait_time: self.retry_wait_time,
            },
        })
    }
}

impl Default for DownloaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    use super::*;

    proptest!(
        #[test]
        fn builder_doesnt_crash(
            retry_wait_time: Duration,
            retry_attempts: u32,
            timeout: Option<Duration>,
        ) {
            DownloaderBuilder::new()
                .retry_wait_time(retry_wait_time)
                .retry_attempts(retry_attempts)
                .timeout(timeout)
                .build()
                .unwrap();
        }
    );
}
