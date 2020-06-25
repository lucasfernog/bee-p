mod proto;

use proto::ProtoSalt;

use ring::rand::{SecureRandom, SystemRandom};
use thiserror::Error;

use std::time::{Duration, Instant};

/// Specifies the number of bytes used for the salt.
const SALT_BY_SIZE: usize = 20;

/// Encapsulates high level functions around salt management.
pub struct Salt {
    bytes: [u8; SALT_BY_SIZE],
    expiration_time: Instant,
}

impl Salt {
    /// Generates a new salt given a lifetime duration.
    pub fn new(lifetime: Duration) -> Result<Self, Error> {
        let mut bytes = [0u8; SALT_BY_SIZE];
        let csrng = SystemRandom::new();
        csrng
            .fill(&mut bytes)
            .map_err(|_| Error::FillingWithRandomBytesFailure)?;

        Ok(Self {
            bytes,
            expiration_time: Instant::now() + lifetime,
        })
    }

    /// Returns the random bytes of this salt.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the expiration time of this salt.
    pub fn expiration(&self) -> &Instant {
        &self.expiration_time
    }

    /// Returns whether this salt has expired.
    pub fn expired(&self) -> bool {
        Instant::now() > self.expiration_time
    }

    #[cfg(test)]
    pub fn size(&self) -> usize {
        self.bytes.len()
    }
}

impl From<ProtoSalt> for Salt {
    fn from(salt: ProtoSalt) -> Self {
        todo!()
    }
}
#[derive(Error, Debug)]
pub enum Error {
    #[error("An error occurred while filling the salt with random bytes")]
    FillingWithRandomBytesFailure,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_salt() {
        let salt = Salt::new(Duration::from_secs(60)).unwrap();

        assert_eq!(SALT_BY_SIZE, salt.size());
        assert!(!salt.expired());
    }
}