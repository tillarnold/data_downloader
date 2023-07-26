use std::fs::File;
use std::io::{Read, Write};
use std::num::NonZeroU64;
use std::path::PathBuf;
use std::time::Duration;
use std::{fs, io};

use reqwest::blocking::Client;

use crate::hashed::HashedVec;
use crate::utils::{hex_str, sha256};
use crate::{
    utils, DownloadRequest, Downloadable, DownloaderBuilder, Error, HashMismatch, InnerDownloadable,
};

#[derive(Debug)]
pub(crate) struct DowloadContext {
    pub(crate) path: PathBuf,
}

#[derive(Debug)]
pub(crate) struct InnerDownloader {
    pub(super) storage_dir: PathBuf,
    /// Total number of attempts made to download a file. This is NonZero as it
    /// makes no sense to do 0 download attempts
    pub(super) download_attempts: NonZeroU64,
    /// How long to wait in between attempts do download a file
    pub(super) failed_download_wait_time: Duration,
    /// The HTTP Client that's used for all requests
    pub(super) client: Client,
}

impl InnerDownloader {
    pub(crate) fn compute_path(&self, r: &InnerDownloadable) -> io::Result<PathBuf> {
        let mut target_path = self.storage_dir.clone();
        target_path.push(hex_str(r.sha256()));

        Ok(target_path)
    }

    /// Get the path and return the data if we loaded it anyways
    pub(crate) fn get_path_with_optional_data(
        &self,
        mut ctx: DowloadContext,
        r: &InnerDownloadable,
    ) -> Result<(PathBuf, Option<HashedVec>), Error> {
        if ctx.path.exists() {
            Ok((ctx.path, None))
        } else {
            let dat = self.get(&mut ctx, r)?;
            Ok((ctx.path, Some(dat)))
        }
    }

    pub(crate) fn make_context(&self, r: &InnerDownloadable) -> Result<DowloadContext, Error> {
        let path = self.compute_path(r)?;
        Ok(DowloadContext { path })
    }

    /// Write these contents to disk
    ///
    /// # Panics
    /// If this [`HashedVec`] does not match the [`Downloadable`].
    fn write_to_file_prechecked(
        &self,
        ctx: &mut DowloadContext,
        r: &InnerDownloadable,
        contents: &HashedVec,
    ) -> Result<(), io::Error> {
        assert_eq!(contents.sha256(), r.sha256());

        self.write_to_file_unchecked(ctx, r, contents.as_slice())
    }

    fn write_to_file_unchecked(
        &self,
        ctx: &DowloadContext,
        r: &InnerDownloadable,
        contents: &[u8],
    ) -> Result<(), io::Error> {
        let (mut tmp_file, tmp_file_path) = utils::create_file_at(
            ctx.path.parent().expect("target path is a file in a dir"),
            &format!("download_{}", hex_str(r.sha256())),
        )?;

        tmp_file.write_all(contents)?;

        fs::rename(tmp_file_path, &ctx.path)?;

        Ok(())
    }

    /// Write these contents to disk
    /// Errors if SHA-256 doesn't match
    fn write_to_file(
        &self,
        ctx: &DowloadContext,
        r: &InnerDownloadable,
        contents: &[u8],
    ) -> Result<(), Error> {
        let real = sha256(contents);
        if r.sha256() != real {
            return Err(Error::ManualHashMismatch(HashMismatch {
                expected: hex_str(r.sha256()),
                was: hex_str(&real),
            }));
        }

        Ok(self.write_to_file_unchecked(ctx, r, contents)?)
    }

    fn procure_and_write(
        &self,
        ctx: &mut DowloadContext,
        r: &InnerDownloadable,
    ) -> Result<HashedVec, Error> {
        let contents = r.procure(self, ctx)?;

        self.write_to_file_prechecked(ctx, r, &contents)?;

        debug_assert_eq!(r.sha256(), contents.sha256());
        Ok(contents)
    }

    pub(crate) fn get(
        &self,
        ctx: &mut DowloadContext,
        r: &InnerDownloadable,
    ) -> Result<HashedVec, Error> {
        if ctx.path.exists() {
            match self.get_cached(ctx, r) {
                Err(Error::OnDiskHashMismatch { .. }) => self.procure_and_write(ctx, r),
                e => e,
            }
        } else {
            self.procure_and_write(ctx, r)
        }
    }

    pub(crate) fn get_cached(
        &self,
        ctx: &DowloadContext,
        r: &InnerDownloadable,
    ) -> Result<HashedVec, Error> {
        let mut res = vec![];
        let mut file = File::open(&ctx.path)?;
        file.read_to_end(&mut res)?;

        match HashedVec::try_new(res, r.sha256()) {
            Ok(vec) => {
                debug_assert_eq!(r.sha256(), vec.sha256());
                Ok(vec)
            }

            Err(err) => Err(Error::OnDiskHashMismatch(HashMismatch {
                expected: hex_str(r.sha256()),
                was: hex_str(&err.got),
            })),
        }
    }
}

/// Configurable Downloader
///
/// If you just want to use the default settings for the [`Downloader`] you can
/// also use the free functions [`get`](crate::get), [`get_cached`](crate::get),
/// [`get_path`](crate::get) instead.
#[derive(Debug)]
pub struct Downloader {
    pub(crate) inner: InnerDownloader,
}

impl Downloader {
    /// Create a [`Downloader`] that saves to the default storage directory
    /// The default storage directory is in the platform specific cache
    /// directory or if that is not available the temporary directory is
    /// used.
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

    /// Get the file contents and if the file has not yet been downloaded,
    /// download it.
    pub fn get<'a>(&self, r: impl Into<Downloadable<'a>>) -> Result<Vec<u8>, Error> {
        let r = r.into().0;
        let mut ctx = self.inner.make_context(&r)?;
        Ok(self.inner.get(&mut ctx, &r)?.into_vec())
    }

    /// Get the file contents and *fail* with an IO error if the file is not yet
    /// downloaded
    pub fn get_cached<'a>(&self, r: impl Into<Downloadable<'a>>) -> Result<Vec<u8>, Error> {
        let r = &r.into().0;

        let ctx = self.inner.make_context(r)?;

        Ok(self.inner.get_cached(&ctx, r)?.into_vec())
    }

    /// Get the file contents and *fail* with an IO error if the file is not yet
    /// downloaded
    pub fn get_cached_by_hash(&self, hash: &[u8]) -> Result<Vec<u8>, Error> {
        let dr = DownloadRequest {
            url: "", // Since we are never downloading this the url doesn't matter
            sha256_hash: hash,
        };

        self.get_cached(&dr)
    }

    /// Insert this data for this [`Downloadable`]. Will error if the SHA-256 is
    /// wrong.
    ///
    /// This is the same as calling [`Downloader::set_by_hash`] with the
    /// sha256_hash ini the [`Downloadable`]
    pub fn set<'a>(&self, r: impl Into<Downloadable<'a>>, data: &[u8]) -> Result<(), Error> {
        let r = &r.into().0;
        let ctx = self.inner.make_context(r)?;

        self.inner.write_to_file(&ctx, r, data)
    }

    /// Insert this data for this [`Downloadable`]. Will error if the SHA-256 is
    /// wrong.
    ///
    ///
    /// ```
    /// use data_downloader::{DownloadRequest, Downloader};
    /// let test =
    ///     &hex_literal::hex!("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
    ///
    /// let dl = Downloader::new().unwrap();
    /// assert!(dl.set_by_hash(test, b"something else").is_err());
    /// let data = b"test";
    /// dl.set_by_hash(test, data).unwrap();
    ///
    /// let g = dl.get_cached_by_hash(test).unwrap();
    /// assert_eq!(data, &g[..]);
    /// ```
    pub fn set_by_hash(&self, hash: &[u8], data: &[u8]) -> Result<(), Error> {
        let dr = DownloadRequest {
            url: "", // Since we are never downloading this the url doesn't matter
            sha256_hash: hash,
        };

        self.set(&dr, data)
    }

    /// Computes the full path to the file and if the file has not yet been
    /// downloaded, download it.
    ///
    /// ## Security
    /// The underlying file could have been changed at any point by a malicious
    /// actor so there is no guarantee that if you pass this path that
    /// this will be the correct file.
    pub fn get_path<'a>(&self, r: impl Into<Downloadable<'a>>) -> Result<PathBuf, Error> {
        let r = &r.into().0;

        let ctx = self.inner.make_context(r)?;
        let (path, _) = self.inner.get_path_with_optional_data(ctx, r)?;

        Ok(path)
    }
}
