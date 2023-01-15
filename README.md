# data_downloader

[![data_downloader crate](https://img.shields.io/crates/v/data_downloader.svg)](https://crates.io/crates/data_downloader)
[![data_downloader documentation](https://docs.rs/data_downloader/badge.svg)](https://docs.rs/data_downloader)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/data_downloader.svg)](./LICENSE-APACHE)

This crate provides a simple way to download files.
In particular this crate aims to make it easy to download and cache files
that do not change over time, for example reference image files, ML models,
example audio files or common password lists.


## Roadmap
- Test concurrency
- Add an expected_size: Optional<u64> size to DownloadRequests
    - If the download is bigger than that fail
    - If it is None no upper limit


