//! Other misc files

use crate::DownloadRequest;

/// <https://github.com/dwyl/english-words>
pub const DWYL_ENGLISH_WORDS_TXT: &DownloadRequest = &DownloadRequest {
    name: "words.txt",
    sha256_hash: &hex_literal::hex!(
        "3797c8dd6adf523e6c1ac8fbb59a7aec6fbc69d723a4af62972eda2e33ec331f"
    ),
    url: "https://github.com/dwyl/english-words/raw/7cb484da5de560c11109c8f3925565966015e5a9/words.txt",
};

/// The famous <https://en.wikipedia.org/wiki/RockYou> password list
pub const ROCKYOU_TXT: &DownloadRequest = &DownloadRequest {
    name: "rockyou.txt",
    sha256_hash: &hex_literal::hex!(
        "6dfa76aa0e02303994fd1062d0ac983f0b69ece5474d85a5bba36362e19c1076"
    ),
    url: "https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt",
};

/// Data from 93 Cars on Sale in the USA in 1993
pub const CARS93_CSV: &DownloadRequest = &DownloadRequest {
    name: "cars93.csv",
    sha256_hash: &hex_literal::hex!(
        "27fef6999ea0ba753e1ae292db98d3d0f1027bb915661fce60c22d5449d2e0ae"
    ),
    url: "http://vincentarelbundock.github.io/Rdatasets/csv/MASS/Cars93.csv",
};

/// The first PDF of the <https://shattered.io/> attack
pub const SHATTERED_PDF_1: &DownloadRequest = &DownloadRequest {
    name: "shattered-1.pdf",
    sha256_hash: &hex_literal::hex!(
        "2BB787A73E37352F92383ABE7E2902936D1059AD9F1BA6DAAA9C1E58EE6970D0"
    ),
    url: "https://shattered.io/static/shattered-1.pdf",
};

/// The second PDF of the <https://shattered.io/> attack
pub const SHATTERED_PDF_2: &DownloadRequest = &DownloadRequest {
    name: "shattered-2.pdf",
    sha256_hash: &hex_literal::hex!(
        "D4488775D29BDEF7993367D541064DBDDA50D383F89F0AA13A6FF2E0894BA5FF"
    ),
    url: "https://shattered.io/static/shattered-2.pdf",
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::{get, get_cached};

    #[test]
    fn download_test() {
        get(CARS93_CSV).unwrap();
    }

    #[test]
    fn shattered_download_test() {
        get(SHATTERED_PDF_1).unwrap();
        get(SHATTERED_PDF_2).unwrap();

        get_cached(SHATTERED_PDF_1).unwrap();
        get_cached(SHATTERED_PDF_2).unwrap();
    }
}
