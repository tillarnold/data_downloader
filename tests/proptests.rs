use std::time::Duration;

use data_downloader::{DownloadRequest, DownloaderBuilder};
use proptest::prelude::*;

proptest!(
    #[test]
    fn get_doesnt_crash(name: String, sha256_hash: Vec<u8>, url: String) {
        let dr: &DownloadRequest = &DownloadRequest {
            name: &name,
            sha256_hash: &sha256_hash,
            url: &url,
        };

        let dl = DownloaderBuilder::new().retry_attempts(0).build().unwrap();
        let _error_ignored = dl.get(dr);
    }

    #[test]
    fn get_cached_doesnt_crash(name: String, sha256_hash: Vec<u8>, url: String) {
        let dr: &DownloadRequest = &DownloadRequest {
            name: &name,
            sha256_hash: &sha256_hash,
            url: &url,
        };

        let dl = DownloaderBuilder::new().retry_attempts(0).build().unwrap();
        let _error_ignored = dl.get_cached(dr);
    }

    #[test]
    fn get_path_doesnt_crash(name: String, sha256_hash: Vec<u8>, url: String) {
        let dr: &DownloadRequest = &DownloadRequest {
            name: &name,
            sha256_hash: &sha256_hash,
            url: &url,
        };

        let dl = DownloaderBuilder::new().retry_attempts(0).build().unwrap();
        let _error_ignored = dl.get_path(dr);
    }

    fn get_doesnt_crash_with_retry(name: String, sha256_hash: Vec<u8>, url: String) {
        let dr: &DownloadRequest = &DownloadRequest {
            name: &name,
            sha256_hash: &sha256_hash,
            url: &url,
        };

        let dl = DownloaderBuilder::new()
            .retry_attempts(2)
            .retry_wait_time(Duration::ZERO)
            .build()
            .unwrap();
        let _error_ignored = dl.get(dr);
    }

    #[test]
    fn get_cached_doesnt_crash_with_retry(name: String, sha256_hash: Vec<u8>, url: String) {
        let dr: &DownloadRequest = &DownloadRequest {
            name: &name,
            sha256_hash: &sha256_hash,
            url: &url,
        };

        let dl = DownloaderBuilder::new()
            .retry_attempts(2)
            .retry_wait_time(Duration::ZERO)
            .build()
            .unwrap();
        let _error_ignored = dl.get_cached(dr);
    }

    #[test]
    fn get_path_doesnt_crash_with_retry(name: String, sha256_hash: Vec<u8>, url: String) {
        let dr: &DownloadRequest = &DownloadRequest {
            name: &name,
            sha256_hash: &sha256_hash,
            url: &url,
        };

        let dl = DownloaderBuilder::new()
            .retry_attempts(2)
            .retry_wait_time(Duration::ZERO)
            .build()
            .unwrap();
        let _error_ignored = dl.get_path(dr);
    }

    #[test]
    fn set_doesnt_crash(name: String, sha256_hash: Vec<u8>, url: String, data: Vec<u8>) {
        let dr: &DownloadRequest = &DownloadRequest {
            name: &name,
            sha256_hash: &sha256_hash,
            url: &url,
        };

        let dl = DownloaderBuilder::new().build().unwrap();
        match dl.set(dr, &data) {
            Err(data_downloader::Error::ManualHashMismatch { .. }) => { /* expected */ }
            e => panic!("not expecting {e:?}"),
        }
    }
);
