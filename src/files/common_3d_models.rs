//! 3D models

use crate::DownloadRequest;

/// <https://en.wikipedia.org/wiki/Utah_teapot> obj
pub const TEAPOT_OBJ: &DownloadRequest = &DownloadRequest {
    sha256_hash: &hex_literal::hex!(
        "1B5396FEDD74B577E32CEF41146582C2F2E1A050D5B4915193C0AC1AD4187ED4"
    ),
    url: "https://raw.githubusercontent.com/alecjacobson/common-3d-test-models/f96f1e93ec315a43cc771d596fcdc4268bd8b047/data/teapot.obj",
};

/// Blender Suzanne
pub const SUZANNE_OBJ: &DownloadRequest = &DownloadRequest {
    sha256_hash: &hex_literal::hex!(
        "D8684326F9BD8CFC24D3D302C1042FA16F63D2E66E49ED56B413FA20BED271E6"
    ),
    url: "https://github.com/alecjacobson/common-3d-test-models/raw/ae0c8d85001509c6f349ddae6642081c3425b6ab/data/suzanne.obj",
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::get;

    #[test]
    fn download_test() {
        get(TEAPOT_OBJ).unwrap();
        get(SUZANNE_OBJ).unwrap();
    }
}
