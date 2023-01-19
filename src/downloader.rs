use std::fs::File;
use std::io::{Read, Write};
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, io};

use reqwest::blocking::Client;

use crate::checked::CheckedVec;
use crate::utils::hex_str;
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

    pub(crate) fn compute_path(&self, r: &impl Downloadable) -> io::Result<PathBuf> {
        let hash_string = hex_str(r.sha256());
        let file_name = format!("{hash_string}_{}", r.file_name());
        let mut target_path = self.storage_dir.clone();
        target_path.push(file_name);

        Ok(target_path)
    }

    /// Get the path and return the data if we loaded it anyways
    pub(crate) fn get_path_with_optional_data(
        &self,
        r: &DownloadRequest,
    ) -> Result<(PathBuf, Option<CheckedVec>), Error> {
        let path = self.compute_path(r)?;

        if path.exists() {
            Ok((path, None))
        } else {
            let dat = self.get(r)?;
            Ok((path, Some(dat)))
        }
    }

    /// Computes the full path to the file and if the file has not yet been
    /// downloaded, download it.
    ///
    /// ## Security
    /// The underlying file could have been changed at any point by a malicious
    /// actor so there is no guarantee that if you pass this path that
    /// this will be the correct file.
    pub fn get_path(&self, r: &DownloadRequest) -> Result<PathBuf, Error> {
        let (path, _) = self.get_path_with_optional_data(r)?;

        Ok(path)
    }

    /// Write these contents to disk
    ///
    /// # Panics
    /// if this CheckedVec does not match the Downloadable
    fn write_to_file_prechecked(
        &self,
        r: &impl Downloadable,
        contents: &CheckedVec,
    ) -> Result<(), Error> {
        assert_eq!(contents.sha256(), r.sha256());

        let target_path = self.compute_path(r)?;

        let (mut tmp_file, tmp_file_path) = utils::create_file_at(
            target_path
                .parent()
                .expect("target path is a file in a dir"),
            &format!("download_{}", r.file_name()),
        )?;

        tmp_file.write_all(contents.as_slice())?;

        fs::rename(tmp_file_path, target_path)?;

        Ok(())
    }

    /// Get the file contents and if the file has not yet been downloaded,
    /// download it.
    pub fn get(&self, r: &impl Downloadable) -> Result<CheckedVec, Error> {
        let target_path = self.compute_path(r)?;

        if target_path.exists() {
            return self.get_cached(r);
        }

        let contents = r.procure(self)?;

        self.write_to_file_prechecked(r, &contents)?;

        debug_assert_eq!(r.sha256(), contents.sha256());
        Ok(contents)
    }

    /// Get the file contents and *fail* with an IO error if the file is not yet
    /// downloaded
    pub fn get_cached(&self, r: &impl Downloadable) -> Result<CheckedVec, Error> {
        let target_path = self.compute_path(r)?;

        let mut res = vec![];
        let mut file = File::open(target_path)?;
        file.read_to_end(&mut res)?;

        match CheckedVec::try_new(res, r.sha256()) {
            Ok(vec) => {
                debug_assert_eq!(r.sha256(), vec.sha256());
                Ok(vec)
            }

            Err(err) => Err(Error::OnDiskHashMismatch {
                expected: hex_str(r.sha256()),
                was: hex_str(&err.got),
            }),
        }
    }
}
