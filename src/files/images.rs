//! [`DownloadRequest`]s for pictures

use crate::DownloadRequest;

pub mod flags;

/// The peppers test image
pub const PEPPERS_TIFF: &DownloadRequest = &DownloadRequest {
    sha256_hash: &hex_literal::hex!(
        "676C21EDCC56B517EBD54764D6026D2BEFE7E15014EC3F0C9E6F2CE1D9AD74BF"
    ),
    // alternative https://sipi.usc.edu/database/download.php?vol=misc&img=4.2.07
    // https://github.com/TomographicImaging/CIL-Data/raw/5affe9b1c3bd20b28aee7756aa968d7c2a9eeff4/peppers.tiff
    url: "https://raw.githubusercontent.com/JuliaImages/TestImages.jl/images/images/peppers_color.tif",
};

/// The mandrill test image
pub const MANDRILL_TIFF: &DownloadRequest = &DownloadRequest {
    sha256_hash: &hex_literal::hex!(
        "3F590B52279FB59B81906F1E928AE713A5357B1AFC1A2017A103ADB563FB4494"
    ),
    //alternative https://sipi.usc.edu/database/download.php?vol=misc&img=4.2.03
    url: "https://raw.githubusercontent.com/JuliaImages/TestImages.jl/images/images/mandrill.tiff",
};

/// <https://en.wikipedia.org/wiki/Tux_(mascot)> as an SVG
pub const TUX_SVG: &DownloadRequest = &DownloadRequest {
    sha256_hash: &hex_literal::hex!(
        "CD503AD510E16FF2869F959CF57B892BB2175A6874FF696B495BD94FD7DB9743"
    ),
    url: "https://upload.wikimedia.org/wikipedia/commons/3/35/Tux.svg",
};

/// Raster image of <https://en.wikipedia.org/wiki/Tux_(mascot)>
pub const TUX_PNG: &DownloadRequest = &DownloadRequest {
    sha256_hash: &hex_literal::hex!(
        "E4F4C8010312078A37CA1797A8C9EAA5156D0FEC407D7875522CA31A3D78C586"
    ),
    url: "https://upload.wikimedia.org/wikipedia/commons/a/af/Tux.png",
};

/// An SVG of a tiger
pub const TIGER_SVG: &DownloadRequest = &DownloadRequest {
    sha256_hash: &hex_literal::hex!(
        "5211E169283F43AB8AD7EA7998D917D5FBB3C568AC85C1A0217E86792822684D"
    ),
    url: "https://upload.wikimedia.org/wikipedia/commons/f/fd/Ghostscript_Tiger.svg",
};

/// Rust logo on transparent background
pub const RUST_LOGO_PNG: &DownloadRequest = &DownloadRequest {
    sha256_hash: &hex_literal::hex!(
        "38C08733444D8673B66E1E9E67420B462DD0E5567BEA18D84DA7B11D1C8CF118"
    ),
    url: "https://www.rust-lang.org/logos/rust-logo-512x512.png",
};

/// Black Rust logo on transparent background
pub const RUST_LOGO_BLK_PNG: &DownloadRequest = &DownloadRequest {
    sha256_hash: &hex_literal::hex!(
        "18968384B4AB73EA582AE44C81BC63351AD48BFD0AB56A156760C48204473496"
    ),
    url: "https://www.rust-lang.org/logos/rust-logo-512x512-blk.png",
};

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::*;
    use crate::Downloader;

    #[test]
    fn download_test() {
        let dl = Downloader::builder()
            .retry_attempts(2)
            .retry_wait_time(Duration::from_secs_f32(10.0))
            .build()
            .unwrap();

        dl.get(PEPPERS_TIFF).unwrap();
        dl.get(TIGER_SVG).unwrap();
        dl.get(RUST_LOGO_PNG).unwrap();
        dl.get(TUX_PNG).unwrap();
        dl.get(MANDRILL_TIFF).unwrap();
    }
}
