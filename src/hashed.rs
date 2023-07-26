use crate::utils::sha256;

/// Internally used wrapper for a [`Vec`] that also keeps track of the SHA-256
/// sum of the data
#[derive(Debug)]
pub struct HashedVec {
    data: Vec<u8>,
    sha256: [u8; 32],
}

#[derive(Debug)]
pub struct HashedVecError {
    pub data: Vec<u8>,
    pub got: [u8; 32],
}

impl HashedVec {
    pub fn try_new(data: Vec<u8>, expected: &[u8]) -> Result<Self, HashedVecError> {
        let real = sha256(&data);

        if expected == real {
            Ok(Self { data, sha256: real })
        } else {
            Err(HashedVecError { data, got: real })
        }
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    pub fn sha256(&self) -> &[u8] {
        &self.sha256
    }

    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
}
