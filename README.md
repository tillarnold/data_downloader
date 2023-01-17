# data_downloader

[![data_downloader crate](https://img.shields.io/crates/v/data_downloader.svg)](https://crates.io/crates/data_downloader)
[![data_downloader documentation](https://docs.rs/data_downloader/badge.svg)](https://docs.rs/data_downloader)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/data_downloader.svg)](./LICENSE-APACHE)

This crate provides a simple way to download files.
In particular this crate aims to make it easy to download and cache files
that do not change over time, for example reference image files, ML models,
example audio files or common password lists.


```rust
use data_downloader::{get, DownloadRequest};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define where to get the file from
    let rfc_link = &DownloadRequest {
        url: "https://www.rfc-editor.org/rfc/rfc2068.txt",
        name: "rfc2068.txt",
        sha256_hash: &hex_literal::hex!(
            "D6C4E471389F2D309AB1F90881576542C742F95B115336A346447D052E0477CF"
        ),
    };

    // Get the binary contents of the file
    let rfc: Vec<u8> = get(rfc_link)?;

    // Convert the file to a String
    let as_text = String::from_utf8(rfc)?;
    assert!(as_text.contains("The Hypertext Transfer Protocol (HTTP) is an application-level"));
    assert!(as_text.contains("protocol"));


    // There are also some handy built-in files 
    let rockyou_txt = get(data_downloader::files::misc::ROCKYOU_TXT)?;
    let pws: HashSet<&[u8]> = rockyou_txt.split(|e| *e == b'\n').collect();
    assert!(pws.contains(&b"hello".as_slice()));
    assert!(pws.contains(&b"goodpassword".as_slice()));
    assert!(!pws.contains(&b"correcthorsebatterystaple".as_slice()));

    Ok(())
}

```

Have a look at the [docs](https://docs.rs/data_downloader) for more examples.


## Alternatives
If you need to download files that might change over time or where you do not know the SHA-256 in advance consider using [cached-path](https://crates.io/crates/cached-path).


## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.