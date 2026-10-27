use super::bitbuf::BitBuffer;
use super::bitbuf::BitBufferIterator;
use super::biterror::BitError;

struct Encoder {
	ones_count: u8,
}

struct Decoder {
	ones_count: u8,
}

impl Encoder {
	fn encode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for b in bits {
			output.push_bit(b);
			if b {
				self.ones_count += 1;
				if self.ones_count < 5 {
					continue;
				}
				output.push_bit(false);
			}
			self.ones_count = 0;
		}
		output
	}

	fn new() -> Self {
		Encoder { ones_count: 0 }
	}
}

impl Decoder {
	fn decode(&mut self, bits: &BitBuffer) -> Result<BitBuffer, BitError> {
		let mut output = BitBuffer::from_bytes(&[]);
		let mut iter: BitBufferIterator = bits.into_iter();
		loop {
			let value = iter.next();
			match value {
				None => {
					break;
				}
				Some(b) => {
					if self.ones_count == 5 {
						if b {
							return Err(BitError::SixOnes { at: iter.get_index_of_last_bit() });
						}
						self.ones_count = 0;
						continue;
					}

					/* Multiplying by 1 or adding 0 do not change the value.
					 * By observing that fact we can omit branching and therefore boost performance. */
					self.ones_count = self.ones_count * (b as u8) + (b as u8);

					output.push_bit(b);
				}
			}
		}
		if self.ones_count == 5 {
			return Err(BitError::TruncatedStuff { at: bits.len() - 1 });
		}
		Ok(output)
	}

	fn new() -> Self {
		Decoder { ones_count: 0 }
	}
}
