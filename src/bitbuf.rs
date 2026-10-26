#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BitBuffer {/* Vec<bool> or Vec<u8> + bit_len */}

impl BitBuffer {
	pub fn from_bytes(bytes: &[u8]) -> Self; // MSB-first
	pub fn to_bytes(&self) -> Vec<u8>; // zero-pad at the end
	pub fn len(&self) -> usize;
	pub fn is_empty(&self) -> bool;
	pub fn bit(&self, index: usize) -> Option<bool>;
	pub fn push_bit(&mut self, bit: bool);
}
