use std::fs::{File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

pub fn hex_str(bytes: &[u8]) -> String {
    let mut res = vec![];
    for byte in bytes {
        write!(res, "{byte:02X}").unwrap();
    }

    String::from_utf8(res).unwrap()
}

pub fn sha256(contents: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();

    hasher.update(contents);
    let hash: [u8; 32] = hasher
        .finalize()
        .as_slice()
        .try_into()
        .expect("sha256 is 32 bytes");

    hash
}

/// Create a new File in the given directory where the file name starts with
/// `prefix` a counter is appended to the prefix until we succeed in creating
/// the file Returns the open [`File`] and the [`PathBuf`] to the file name that
/// succeeded
pub fn create_file_at(dir: &Path, prefix: &str) -> io::Result<(File, PathBuf)> {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 1000 {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Too many attemtps to create a temporary file failed",
            ));
        }

        let file_name = format!("{prefix}_{counter}");
        let mut path = dir.to_path_buf();
        path.push(file_name);

        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((file, path)),
            Err(e) => {
                if e.kind() == ErrorKind::AlreadyExists {
                    continue;
                }

                return Err(e);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hex_str_test() {
        assert_eq!(hex_str(&[]), "");
        assert_eq!(hex_str(&[0]), "00");
        assert_eq!(hex_str(&[1, 2, 3, 17]), "01020311");
        assert_eq!(
            hex_str(&hex_literal::hex!(
                "bd577a113a864445d4c299885e0cb97d4ba92b5f"
            )),
            "BD577A113A864445D4C299885E0CB97D4BA92B5F"
        );
    }
}
