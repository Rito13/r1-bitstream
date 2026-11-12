#[derive(Clone, Debug, Default, PartialEq, Eq)]
/** A buffer that allows to store variable number of bits. */
pub struct BitBuffer {
	/** The raw data storage. Bits are grouped by eight inside a u8 and stored in MSB-first order. */
	data: Vec<u8>,
	/** The number of bits it the last byte in the data. */
	bits_in_last_byte: u8,
}

impl BitBuffer {
	/** The number of bits needed to index all bits that can fit one byte. */
	const IN_BYTE_INDEX_BIT_SIZE: u8 = 3; // Byte has 8 bits and 8 is 2 to the power of 3.
	/** The mask that can be used to extract the bit index inside the byte from the bit index inside the whole buffer. */
	const IN_BYTE_INDEX_MASK: usize = (1 << BitBuffer::IN_BYTE_INDEX_BIT_SIZE) - 1;
	/** The index of the bit 0 inside the u8. */
	const FIRS_BIT_INDEX: u8 = 7;

	///Construct a new bit buffer from a table of bytes.
	///
	///__Parameters:__
	///* `bytes`: The table of bytes to initialy store inside the buffer.
	pub fn from_bytes(bytes: &[u8]) -> Self {
		BitBuffer {
			data: bytes.to_vec(),
			bits_in_last_byte: 8,
		}
	}

	///Convert this buffer into a vector of bytes.
	///The last byte is zero-panned during the conversion.
	///
	///__Returns:__ Raw copy of the data stored inside this buffer.
	pub fn to_bytes(&self) -> Vec<u8> {
		self.data.clone() // Zero-pads the last byte.
	}

	///Get the length of this buffer.
	///
	///__Returns:__ The number of bits currently stored inside this buffer.
	pub fn len(&self) -> usize {
		self.data.len() * 8 + (self.bits_in_last_byte as usize) - 8
	}

	///Check if this buffer is empty.
	///
	///__Returns:__ `true` if and only if the buffer does not contain any bits.
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}

	///Get the value of a bit under the given index.
	///
	///__Parameters:__
	///* `index`: The index of the bit to get the value of.
	///
	///__Returns:__ The value of the bit or `None` if the index is out of bounds.
	pub fn bit(&self, index: usize) -> Option<bool> {
		if self.len() <= index {
			None
		} else {
			let byte = index >> BitBuffer::IN_BYTE_INDEX_BIT_SIZE;
			Some(((self.data[byte] >> (BitBuffer::FIRS_BIT_INDEX - (index & BitBuffer::IN_BYTE_INDEX_MASK) as u8)) & 0b1) == 1) // MSB-first
		}
	}

	///Insert a new bit at the end of the buffer.
	///
	///__Parameters:__
	///* `bit`: The value for the inserted bit.
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

	///Get the value of the last bit inside the buffer.
	///
	///__Returns:__ The value of the last bit or `None` if the buffer is empty.
	pub fn last_bit(&self) -> Option<bool> {
		if self.is_empty() {
			None
		} else {
			let byte = self.data.len() - 1;
			Some(((self.data[byte] >> (BitBuffer::FIRS_BIT_INDEX - (self.bits_in_last_byte - 1))) & 0b1) == 1) // MSB-first
		}
	}
}

/** An iterator for iterating over the BitBuffer. */
pub struct BitBufferIterator<'bit_buffer> {
	bit_buffer: &'bit_buffer BitBuffer,
	index: usize,
}

impl BitBufferIterator<'_> {
	///Get the index of the previously returned bit.
	///
	///__Returns:__ The index of that bit inside the iterated buffer.
	pub fn get_index_of_last_bit(&self) -> usize {
		self.index - 1
	}
}

impl Iterator for BitBufferIterator<'_> {
	/** The type that allows to represent one bit stored inside the iterated buffer. */
	type Item = bool;

	///Increment the iterator and get the value of the new bit pointed by the iterator.
	///
	///__Returns:__ The value of currently the next bit.
	fn next(&mut self) -> Option<Self::Item> {
		let out = self.bit_buffer.bit(self.index);
		self.index += 1;
		out
	}
}

impl<'bit_buffer> IntoIterator for &'bit_buffer BitBuffer {
	/** The type returned by the getter of the buffer element. */
	type Item = bool;
	/** The forward iterator for the bit buffer. */
	type IntoIter = BitBufferIterator<'bit_buffer>;

	///Convert this buffer into a forward iterator.
	///
	///__Returns:__ The forward iterator iterating over this buffer.
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

		for (i, v) in bits_to_test.iter().enumerate() {
			assert_eq!(bit_buffer.bit(i), Some(*v), "i = {}", i);
		}
		assert_eq!(bit_buffer.bit(bit_buffer.len()), None); // Out of bound check should fail.

		let bit_buffer2 = BitBuffer::from_bytes(&returned_bytes);
		assert_ne!(bit_buffer, bit_buffer2); // Converting to bytes and then to bits should not be equal.
		assert_eq!(bit_buffer.data, bit_buffer2.data); // Though their data should.
	}
}
