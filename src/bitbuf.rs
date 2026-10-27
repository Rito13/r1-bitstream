#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BitBuffer {
	data: Vec<u8>,
	bits_in_last_byte: u8,
}

impl BitBuffer {
	const IN_BYTE_INDEX_BIT_SIZE: u8 = 3;
	const IN_BYTE_INDEX_MASK: usize = (1 << BitBuffer::IN_BYTE_INDEX_BIT_SIZE) - 1;
	const FIRS_BIT_INDEX: u8 = 7;

	pub fn from_bytes(bytes: &[u8]) -> Self {
		BitBuffer {
			data: bytes.to_vec(),
			bits_in_last_byte: 8,
		}
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		let out = self.data.clone();
		out // Zero-pads the last byte.
	}

	pub fn len(&self) -> usize {
		self.data.len() * 8 + (self.bits_in_last_byte as usize) - 8
	}

	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	pub fn bit(&self, index: usize) -> Option<bool> {
		if self.len() <= index {
			None
		} else {
			let byte = index >> BitBuffer::IN_BYTE_INDEX_BIT_SIZE;
			Some(((self.data[byte] >> (BitBuffer::FIRS_BIT_INDEX - (index & BitBuffer::IN_BYTE_INDEX_MASK) as u8)) & 0b1) == 1) // MSB-first
		}
	}

	pub fn push_bit(&mut self, bit: bool) {
		/* MSB-first */
		if self.bits_in_last_byte == 8 {
			self.data.push((bit as u8) << BitBuffer::FIRS_BIT_INDEX);
			self.bits_in_last_byte = 1;
		} else {
			let byte = self.data.len() - 1;
			self.data[byte] |= (bit as u8) << (BitBuffer::FIRS_BIT_INDEX - self.bits_in_last_byte);
			self.bits_in_last_byte += 1;
		}
	}

	pub fn last_bit(&self) -> Option<bool> {
		if self.len() == 0 {
			None
		} else {
			let byte = self.data.len() - 1;
			Some(((self.data[byte] >> (BitBuffer::FIRS_BIT_INDEX - (self.bits_in_last_byte - 1))) & 0b1) == 1) // MSB-first
		}
	}
}

pub struct BitBufferIterator<'bit_buffer> {
	bit_buffer: &'bit_buffer BitBuffer,
	index: usize,
}

impl BitBufferIterator<'_> {
	pub fn get_index_of_last_bit(&self) -> usize {
		self.index - 1
	}
}

impl Iterator for BitBufferIterator<'_> {
	type Item = bool;

	fn next(&mut self) -> Option<Self::Item> {
		let out = self.bit_buffer.bit(self.index);
		self.index += 1;
		out
	}
}

impl<'bit_buffer> IntoIterator for &'bit_buffer BitBuffer {
	type Item = bool;
	type IntoIter = BitBufferIterator<'bit_buffer>;

	fn into_iter(self) -> Self::IntoIter {
		BitBufferIterator { bit_buffer: self, index: 0 }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn bit_buffer() {
		/* Create empty BitBuffer. */
		let bytes: [u8; 0] = [];
		let mut bit_buffer = BitBuffer::from_bytes(&bytes);

		assert!(bit_buffer.is_empty());
		assert_eq!(bit_buffer.len(), 0); // Check if len correctly works for empty buffer.

		let bits_to_test = vec![true, true, true, false, false, false, true, true, false, false, true];

		/* Push some bits to it. */
		for bit in &bits_to_test {
			bit_buffer.push_bit(*bit)
		}

		assert!(!bit_buffer.is_empty());
		assert_eq!(bit_buffer.len(), bits_to_test.len());

		let returned_bytes = bit_buffer.to_bytes();
		assert_eq!(returned_bytes[0], 0b11100011); // Sould match first 8 bits in bits_to_test.
		assert_eq!(returned_bytes[1], 0b00100000); // Sould match last 3 bits in bits_to_test and be zero-panned to 8 bits.

		for i in 0..bits_to_test.len() {
			assert_eq!(bit_buffer.bit(i), Some(bits_to_test[i]), "i = {}", i);
		}
		assert_eq!(bit_buffer.bit(bit_buffer.len()), None); // Out of bound check should fail.

		let bit_buffer2 = BitBuffer::from_bytes(&returned_bytes);
		assert_ne!(bit_buffer, bit_buffer2); // Converting to bytes and then to bits should not be equal.
		assert_eq!(bit_buffer.data, bit_buffer2.data); // Though their data should.
	}
}
