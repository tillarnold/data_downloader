//! Models for <https://github.com/ggerganov/whisper.cpp>

use crate::DownloadRequest;

/// approx. 390MB
pub const GGML_TINY: &DownloadRequest = &DownloadRequest {
    name: "ggml-tiny.bin",
    sha256_hash: &hex_literal::hex!(
        "be07e048e1e599ad46341c8d2a135645097a538221678b7acdd1b1919c6e1b21"
    ),
    url: "https://huggingface.co/datasets/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin",
};

/// approx. 390MB
pub const GGML_TINY_EN: &DownloadRequest = &DownloadRequest {
    name: "ggml-tiny.en.bin",
    sha256_hash: &hex_literal::hex!(
        "921e4cf8686fdd993dcd081a5da5b6c365bfde1162e72b08d75ac75289920b1f"
    ),
    url: "https://huggingface.co/datasets/ggerganov/whisper.cpp/resolve/main/ggml-tiny.en.bin",
};

/// approx. 500MB
pub const GGML_BASE: &DownloadRequest = &DownloadRequest {
    name: "ggml-base.bin",
    sha256_hash: &hex_literal::hex!(
        "60ed5bc3dd14eea856493d334349b405782ddcaf0028d4b5df4088345fba2efe"
    ),
    url: "https://huggingface.co/datasets/ggerganov/whisper.cpp/resolve/main/ggml-base.bin",
};

/// approx. 500MB
pub const GGML_BASE_EN: &DownloadRequest = &DownloadRequest {
    name: "ggml-base.en.bin",
    sha256_hash: &hex_literal::hex!(
        "a03779c86df3323075f5e796cb2ce5029f00ec8869eee3fdfb897afe36c6d002"
    ),
    url: "https://huggingface.co/datasets/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin",
};

/// approx. 1GB
pub const GGML_SMALL_EN: &DownloadRequest = &DownloadRequest {
    name: "ggml-small.en.bin",
    sha256_hash: &hex_literal::hex!(
        "c6138d6d58ecc8322097e0f987c32f1be8bb0a18532a3f88f734d1bbf9c41e5d"
    ),
    url: "https://huggingface.co/datasets/ggerganov/whisper.cpp/resolve/main/ggml-small.en.bin",
};

/// approx. 1GB
pub const GGML_SMALL: &DownloadRequest = &DownloadRequest {
    name: "ggml-small.bin",
    sha256_hash: &hex_literal::hex!(
        "1be3a9b2063867b937e64e2ec7483364a79917e157fa98c5d94b5c1fffea987b"
    ),
    url: "https://huggingface.co/datasets/ggerganov/whisper.cpp/resolve/main/ggml-small.bin",
};

/// approx. 4.7GB
pub const GGML_LARGE: &DownloadRequest = &DownloadRequest {
    name: "ggml-large.bin",
    sha256_hash: &hex_literal::hex!(
        "9a423fe4d40c82774b6af34115b8b935f34152246eb19e80e376071d3f999487"
    ),
    url: "https://huggingface.co/datasets/ggerganov/whisper.cpp/resolve/main/ggml-large.bin",
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::{get, get_cached};

    #[test]
    fn download_test() {
        let models = [GGML_TINY_EN, GGML_BASE];

        for m in models {
            println!("Downloading {m:?}");
            get(m).unwrap();
        }

        for m in models {
            println!("Checking {m:?}");
            get_cached(m).unwrap();
        }
    }
}
