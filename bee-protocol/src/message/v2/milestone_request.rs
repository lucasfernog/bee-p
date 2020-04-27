use crate::message::{
    Message,
    MessageError,
};

use std::{
    convert::TryInto,
    ops::Range,
};

const INDEX_SIZE: usize = 4;
const CONSTANT_SIZE: usize = INDEX_SIZE;

#[derive(Clone, Default)]
pub(crate) struct MilestoneRequest {
    pub(crate) index: u32,
}

impl MilestoneRequest {
    pub(crate) fn new(index: u32) -> Self {
        Self { index: index }
    }
}

impl Message for MilestoneRequest {
    const ID: u8 = 0x03;

    fn size_range() -> Range<usize> {
        (CONSTANT_SIZE)..(CONSTANT_SIZE + 1)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, MessageError> {
        if !Self::size_range().contains(&bytes.len()) {
            Err(MessageError::InvalidPayloadLength(bytes.len()))?;
        }

        let mut message = Self::default();

        message.index = u32::from_be_bytes(
            bytes[0..INDEX_SIZE]
                .try_into()
                .map_err(|_| MessageError::InvalidPayloadField)?,
        );

        Ok(message)
    }

    fn size(&self) -> usize {
        CONSTANT_SIZE
    }

    fn to_bytes(self, bytes: &mut [u8]) {
        bytes.copy_from_slice(&self.index.to_be_bytes())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::message::{
        Header,
        HEADER_SIZE,
    };

    const INDEX: u32 = 0x81f7df7c;

    #[test]
    fn id_test() {
        assert_eq!(MilestoneRequest::ID, 3);
    }

    #[test]
    fn size_range_test() {
        assert_eq!(MilestoneRequest::size_range().contains(&3), false);
        assert_eq!(MilestoneRequest::size_range().contains(&4), true);
        assert_eq!(MilestoneRequest::size_range().contains(&5), false);
    }

    #[test]
    fn from_bytes_invalid_length_test() {
        match MilestoneRequest::from_bytes(&[0; 3]) {
            Err(MessageError::InvalidPayloadLength(length)) => assert_eq!(length, 3),
            _ => unreachable!(),
        }
        match MilestoneRequest::from_bytes(&[0; 5]) {
            Err(MessageError::InvalidPayloadLength(length)) => assert_eq!(length, 5),
            _ => unreachable!(),
        }
    }

    #[test]
    fn size_test() {
        let message = MilestoneRequest::new(INDEX);

        assert_eq!(message.size(), CONSTANT_SIZE);
    }

    fn to_from_eq(message: MilestoneRequest) {
        assert_eq!(message.index, INDEX);
    }

    #[test]
    fn to_from_test() {
        let message_from = MilestoneRequest::new(INDEX);
        let mut bytes = vec![0u8; message_from.size()];

        message_from.to_bytes(&mut bytes);
        to_from_eq(MilestoneRequest::from_bytes(&bytes).unwrap());
    }

    #[test]
    fn full_to_from_test() {
        let message_from = MilestoneRequest::new(INDEX);
        let bytes = message_from.into_full_bytes();

        to_from_eq(
            MilestoneRequest::from_full_bytes(&Header::from_bytes(&bytes[0..HEADER_SIZE]), &bytes[HEADER_SIZE..])
                .unwrap(),
        );
    }
}
