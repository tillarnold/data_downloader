//! Audio files

use crate::DownloadRequest;

/// John F. Kennedy quote "And so, my fellow Americans: ask not what your
/// country can do for you—ask what you can do for your country"
pub const JFK_ASK_NOT_WAV: &DownloadRequest = &DownloadRequest {
    name: "jfk.wav",
    sha256_hash: &hex_literal::hex!(
        "59DFB9A4ACB36FE2A2AFFC14BACBEE2920FF435CB13CC314A08C13F66BA7860E"
    ),
    url: "https://github.com/ggerganov/whisper.cpp/raw/master/samples/jfk.wav",
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::get;

    #[test]
    fn download_test() {
        get(JFK_ASK_NOT_WAV).unwrap();
    }
}
