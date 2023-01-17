//! [`DownloadRequest`]s for pictures

use crate::DownloadRequest;

/// The peppers test image
pub const PEPPERS_TIFF: &DownloadRequest = &DownloadRequest {
    name: "peppers.tiff",
    sha256_hash: &hex_literal::hex!(
        "676C21EDCC56B517EBD54764D6026D2BEFE7E15014EC3F0C9E6F2CE1D9AD74BF"
    ),
    url: "https://sipi.usc.edu/database/download.php?vol=misc&img=4.2.07",
};

/// The mandrill test image
pub const MANDRILL_TIFF: &DownloadRequest = &DownloadRequest {
    name: "mandrill.tiff",
    sha256_hash: &hex_literal::hex!(
        "3F590B52279FB59B81906F1E928AE713A5357B1AFC1A2017A103ADB563FB4494"
    ),
    url: "https://sipi.usc.edu/database/download.php?vol=misc&img=4.2.03",
};

/// <https://en.wikipedia.org/wiki/Tux_(mascot)> as an SVG
pub const TUX_SVG: &DownloadRequest = &DownloadRequest {
    name: "tux.svg",
    sha256_hash: &hex_literal::hex!(
        "CD503AD510E16FF2869F959CF57B892BB2175A6874FF696B495BD94FD7DB9743"
    ),
    url: "https://upload.wikimedia.org/wikipedia/commons/3/35/Tux.svg",
};

/// Raster image of <https://en.wikipedia.org/wiki/Tux_(mascot)>
pub const TUX_PNG: &DownloadRequest = &DownloadRequest {
    name: "tux.png",
    sha256_hash: &hex_literal::hex!(
        "E4F4C8010312078A37CA1797A8C9EAA5156D0FEC407D7875522CA31A3D78C586"
    ),
    url: "https://upload.wikimedia.org/wikipedia/commons/a/af/Tux.png",
};

/// An SVG of a tiger
pub const TIGER_SVG: &DownloadRequest = &DownloadRequest {
    name: "Ghostscript_Tiger.svg",
    sha256_hash: &hex_literal::hex!(
        "5211E169283F43AB8AD7EA7998D917D5FBB3C568AC85C1A0217E86792822684D"
    ),
    url: "https://upload.wikimedia.org/wikipedia/commons/f/fd/Ghostscript_Tiger.svg",
};

/// Rust logo on transparent background
pub const RUST_LOGO_PNG: &DownloadRequest = &DownloadRequest {
    name: "rust-logo-512x512.png",
    sha256_hash: &hex_literal::hex!(
        "38C08733444D8673B66E1E9E67420B462DD0E5567BEA18D84DA7B11D1C8CF118"
    ),
    url: "https://www.rust-lang.org/logos/rust-logo-512x512.png",
};

/// Black Rust logo on transparent background
pub const RUST_LOGO_BLK_PNG: &DownloadRequest = &DownloadRequest {
    name: "rust-logo-512x512-blk.png",
    sha256_hash: &hex_literal::hex!(
        "18968384B4AB73EA582AE44C81BC63351AD48BFD0AB56A156760C48204473496"
    ),
    url: "https://www.rust-lang.org/logos/rust-logo-512x512-blk.png",
};

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    use crate::{get, Downloader};

    #[test]
    fn download_test() {
        get(PEPPERS_TIFF).unwrap();
        get(MANDRILL_TIFF).unwrap();
        get(RUST_LOGO_PNG).unwrap();
        get(RUST_LOGO_BLK_PNG).unwrap();
    }

    #[test]
    fn download_test_2() {
        // Slightly flaky files that might need some more attempts and longer wait times

        let dl = Downloader::builder()
            .retry_attempts(6)
            .retry_wait_time(Duration::from_secs_f32(10.0))
            .build()
            .unwrap();

        dl.get(TUX_SVG).unwrap();
        dl.get(TUX_PNG).unwrap();
        dl.get(TIGER_SVG).unwrap();
    }
}
