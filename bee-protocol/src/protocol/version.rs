const VERSION_LEGACY: u8 = 1 << 0;
const VERSION_STING: u8 = 1 << 1;
pub(crate) const SUPPORTED_VERSIONS: [u8; 1] = [VERSION_LEGACY | VERSION_STING];

