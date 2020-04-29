use crate::message::Message;

use std::ops::Range;

const HASH_SIZE: usize = 49;
const CONSTANT_SIZE: usize = HASH_SIZE;

#[derive(Clone)]
pub(crate) struct TransactionRequest {
    pub(crate) hash: [u8; HASH_SIZE],
}

impl TransactionRequest {
    pub(crate) fn new(hash: &[u8]) -> Self {
        let mut new = Self::default();

        new.hash.copy_from_slice(hash);

        new
    }
}

impl Default for TransactionRequest {
    fn default() -> Self {
        Self { hash: [0; HASH_SIZE] }
    }
}

impl Message for TransactionRequest {
    const ID: u8 = 0x05;

    fn size_range() -> Range<usize> {
        (CONSTANT_SIZE)..(CONSTANT_SIZE + 1)
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        let mut message = Self::default();

        message.hash.copy_from_slice(&bytes[0..HASH_SIZE]);

        message
    }

    fn size(&self) -> usize {
        CONSTANT_SIZE
    }

    fn to_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.hash)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::message::{
        Header,
        MessageError,
        Tlv,
        HEADER_SIZE,
    };

    use bee_test::slices::slice_eq;

    const HASH: [u8; HASH_SIZE] = [
        160, 3, 36, 228, 202, 18, 56, 37, 229, 28, 240, 65, 225, 238, 64, 55, 244, 83, 155, 232, 31, 255, 208, 9, 126,
        21, 82, 57, 180, 237, 182, 101, 242, 57, 202, 28, 118, 203, 67, 93, 74, 238, 57, 39, 51, 169, 193, 124, 254,
    ];

    #[test]
    fn id() {
        assert_eq!(TransactionRequest::ID, 5);
    }

    #[test]
    fn size_range() {
        assert_eq!(TransactionRequest::size_range().contains(&48), false);
        assert_eq!(TransactionRequest::size_range().contains(&49), true);
        assert_eq!(TransactionRequest::size_range().contains(&50), false);
    }

    #[test]
    fn size() {
        let message = TransactionRequest::new(&HASH);

        assert_eq!(message.size(), CONSTANT_SIZE);
    }

    fn to_from_eq(message: TransactionRequest) {
        assert_eq!(slice_eq(&message.hash, &HASH), true);
    }

    #[test]
    fn to_from() {
        let message_from = TransactionRequest::new(&HASH);
        let mut bytes = vec![0u8; message_from.size()];

        message_from.to_bytes(&mut bytes);
        to_from_eq(TransactionRequest::from_bytes(&bytes));
    }

    #[test]
    fn tlv_invalid_length() {
        match Tlv::from_bytes::<TransactionRequest>(
            &Header {
                message_type: TransactionRequest::ID,
                message_length: 48,
            },
            &[0; 48],
        ) {
            Err(MessageError::InvalidPayloadLength(length)) => assert_eq!(length, 48),
            _ => unreachable!(),
        }
        match Tlv::from_bytes::<TransactionRequest>(
            &Header {
                message_type: TransactionRequest::ID,
                message_length: 50,
            },
            &[0; 50],
        ) {
            Err(MessageError::InvalidPayloadLength(length)) => assert_eq!(length, 50),
            _ => unreachable!(),
        }
    }

    #[test]
    fn tlv() {
        let message_from = TransactionRequest::new(&HASH);
        let bytes = Tlv::into_bytes(message_from);

        to_from_eq(
            Tlv::from_bytes::<TransactionRequest>(&Header::from_bytes(&bytes[0..HEADER_SIZE]), &bytes[HEADER_SIZE..])
                .unwrap(),
        );
    }
}
