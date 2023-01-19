use std::fs::File;
use std::io::{Read, Write};
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, io, thread};

use reqwest::blocking::Client;

use crate::utils::{hex_str, sha256};
use crate::{utils, DownloadRequest, Downloadable, DownloaderBuilder, Error};

/// Configurable Downloader
///
/// If you just want to use the default settings for the [`Downloader`] you can
/// also use the free functions [`get`](crate::get), [`get_cached`](crate::get),
/// [`get_path`](crate::get) instead.
#[derive(Debug)]
pub struct Downloader {
    pub(super) storage_dir: PathBuf,
    /// Total number of attempts made to download a file. This is NonZero as it
    /// makes no sense to do 0 download attempts
    pub(super) download_attempts: NonZeroU64,
    /// How long to wait inbetween attempts do download a file
    pub(super) failed_download_wait_time: Duration,
    /// The HTTP Client that's used for all requests
    pub(super) client: Client,
}

impl Downloader {
    /// Create a [`Downloader`] that saves to the default storage directory
    /// The default storage directory is in the platform specific cache dir or
    /// if that is not available the temporary directory is used.
    ///
    /// Note that no guarantees about the permissions of the default storage
    /// directory are made. It is possible that this directory is accessible
    /// for other users on the system.
    pub fn new() -> Result<Self, Error> {
        Self::builder().build()
    }

    /// Creates a [`DownloaderBuilder`] to configure a Client.
    /// This is the same as [`DownloaderBuilder::new()`]
    pub fn builder() -> DownloaderBuilder {
        DownloaderBuilder::new()
    }

    fn compute_path(&self, r: &impl Downloadable) -> io::Result<PathBuf> {
        let hash_string = hex_str(r.sha256());
        let file_name = format!("{hash_string}_{}", r.file_name());
        let mut target_path = self.storage_dir.clone();
        target_path.push(file_name);

        Ok(target_path)
    }

    /// Computes the full path to the file and if the file has not yet been
    /// downloaded, download it.
    ///
    /// ## Security
    /// The underlying file could have been changed at any point by a malicious
    /// actor so there is no guarantee that if you pass this path that
    /// this will be the correct file.
    pub fn get_path(&self, r: &DownloadRequest) -> Result<PathBuf, Error> {
        let path = self.compute_path(r)?;

        if !path.exists() {
            self.get(r)?;
        }

        Ok(path)
    }

    /// Download the file even if it already exists.
    fn force_download(&self, r: &impl Downloadable) -> Result<Vec<u8>, Error> {
        let contents = r.compute(&self)?;
        let hash = sha256(&contents);

        if hash != r.sha256() {
            return Err(Error::DownloadHashMismatch {
                expected: hex_str(r.sha256()),
                was: hex_str(&hash),
            });
        }

        let target_path = self.compute_path(r)?;

        let (mut tmp_file, tmp_file_path) = utils::create_file_at(
            target_path
                .parent()
                .expect("target path is a file in a dir"),
            &format!("download_{}", r.file_name()),
        )?;

        tmp_file.write_all(&contents)?;

        fs::rename(tmp_file_path, target_path)?;

        Ok(contents.to_vec())
    }

    /// Get the file contents and if the file has not yet been downloaded,
    /// download it.
    pub fn get(&self, r: &impl Downloadable) -> Result<Vec<u8>, Error> {
        let target_path = self.compute_path(r)?;

        for i in 0..self.download_attempts.get() {
            // We recheck here in case somebody else has donwloaded it by now
            if target_path.exists() {
                return self.get_cached(r);
            }
            match self.force_download(r) {
                Ok(data) => return Ok(data),
                Err(e) => {
                    // This is the last loop iteration

                    //TODO: only retry here if the error is considered recoverable

                    if i == self.download_attempts.get() - 1 {
                        return Err(e);
                    }
                    if !self.failed_download_wait_time.is_zero() {
                        thread::sleep(self.failed_download_wait_time);
                    }
                }
            }
        }

        unreachable!()
    }

    /// Get the file contents and *fail* with an IO error if the file is not yet
    /// downloaded
    pub fn get_cached(&self, r: &impl Downloadable) -> Result<Vec<u8>, Error> {
        let target_path = self.compute_path(r)?;

        let mut res = vec![];
        let mut file = File::open(target_path)?;
        file.read_to_end(&mut res)?;

        let hash = sha256(&res);

        if hash != r.sha256() {
            return Err(Error::OnDiskHashMismatch {
                expected: hex_str(r.sha256()),
                was: hex_str(&hash),
            });
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::files::audio;

    #[test]
    fn download_test() {
        let downloader = Downloader::new().unwrap();
        downloader.force_download(audio::JFK_ASK_NOT_WAV).unwrap();

        downloader.get_cached(audio::JFK_ASK_NOT_WAV).unwrap();
    }
}
