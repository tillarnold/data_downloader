use std::fs::File;
use std::io::Write;

use data_downloader::{DownloadRequest, Downloader, HashMismatch};

#[test]
fn corruption_test() {
    let rfc_link = &DownloadRequest {
        url: "https://www.rfc-editor.org/rfc/rfc1068.txt",
        sha256_hash: &hex_literal::hex!(
            "A2D8F002CCAADE5C92E8ED98011554F80F3A5B7324A8902017AE2E6A97E4D6B0"
        ),
    };

    let downloader = Downloader::new().unwrap();

    downloader.get(rfc_link).unwrap();
    downloader.get_cached(rfc_link).unwrap();
    let path = downloader.get_path(rfc_link).unwrap();
    {
        let mut f = File::create(path).unwrap();
        f.write_all(b"something else").unwrap();
    }

    match downloader.get_cached(rfc_link) {
        Err(data_downloader::Error::OnDiskHashMismatch(HashMismatch { expected, was })) => {
            assert_eq!(
                expected,
                "A2D8F002CCAADE5C92E8ED98011554F80F3A5B7324A8902017AE2E6A97E4D6B0"
            );
            assert_eq!(
                was,
                "F41F3FA625FF120DDCA7EF456BF66371ECEA23C129F4E4C32367101EDB516CF8"
            );
        }
        e => panic!("unexpected {e:?}"),
    }

    downloader.get(rfc_link).unwrap();
}
