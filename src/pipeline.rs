#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitError {
    SixOnes { at: usize },
    TruncatedStuff { at: usize },
    NotByteAligned { len: usize },
}

impl std::fmt::Display for BitError { /* ... */ }
impl std::error::Error for BitError {}

pub fn scramble(input: &BitBuffer) -> BitBuffer;
pub fn descramble(input: &BitBuffer) -> BitBuffer;
pub fn nrzi_encode(input: &BitBuffer) -> BitBuffer;
pub fn nrzi_decode(input: &BitBuffer) -> BitBuffer;
pub fn stuff(input: &BitBuffer) -> BitBuffer;
pub fn destuff(input: &BitBuffer) -> Result<BitBuffer, BitError>;

pub fn encode(payload: &[u8]) -> BitBuffer;
pub fn decode(air: &BitBuffer) -> Result<Vec<u8>, BitError>;
