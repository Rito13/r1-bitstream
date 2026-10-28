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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn table_compare() {
		let mut expected_output_ff = BitBuffer::from_bytes(&[0xFB]);
		expected_output_ff.push_bit(true);

		let mut expected_output_7e = BitBuffer::from_bytes(&[0x7D]);
		expected_output_7e.push_bit(false);

		for [input, expected_output] in [
			[BitBuffer::from_bytes(&[0xFF]), expected_output_ff],
			[BitBuffer::from_bytes(&[0x7E]), expected_output_7e],
			[BitBuffer::from_bytes(&[0x55]), BitBuffer::from_bytes(&[0x55])],
			[BitBuffer::from_bytes(&[0x00]), BitBuffer::from_bytes(&[0x00])],
		] {
			let mut encoder = Encoder::new();
			let output = encoder.encode(&input);
			assert_eq!(output, expected_output, "encode() with input[0..7] == 0x{:X}", input.to_bytes()[0]);

			let mut decoder = Decoder::new();
			let output = decoder.decode(&expected_output).expect("Test value is invalid.");
			assert_eq!(output, input, "decode() with expected_output[0..7] == 0x{:X}", expected_output.to_bytes()[0]);
		}
	}

	#[test]
	fn encode_decode() {
		let mut tests = [
			BitBuffer::from_bytes(&[0x00u8, 0x55, 0x13]),
			BitBuffer::from_bytes(&[0xFF, 0x88, 0xFF]),
			BitBuffer::from_bytes(&[0x55, 0x33, 0x17]),
			BitBuffer::from_bytes(&[0xAA, 0x99, 0x9A]),
			BitBuffer::from_bytes(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]), // Test all ones.
			BitBuffer::from_bytes(&[0xFB, 0xEF, 0xBE, 0xFB, 0xEF, 0xBE]), // Test 5 ones and one zero.
			BitBuffer::from_bytes(&[]),
		];

		tests[2].push_bit(true);
		tests[3].push_bit(true);
		tests[3].push_bit(false);

		for input in tests {
			let mut encoder = Encoder::new();
			let mut decoder = Decoder::new();

			let in_between = encoder.encode(&input);
			let output = decoder.decode(&in_between).expect("Something went wrong.");

			assert_eq!(input, output);
		}
	}

	#[test]
	fn six_ones() {
		let bit_buffer = BitBuffer::from_bytes(&[0x7E]);
		let mut decoder = Decoder::new();
		assert_eq!(decoder.decode(&bit_buffer), Err(BitError::SixOnes { at: 6 }));
	}

	#[test]
	fn truncated() {
		let bit_buffer = BitBuffer::from_bytes(&[0x9F]);
		let mut decoder = Decoder::new();
		assert_eq!(decoder.decode(&bit_buffer), Err(BitError::TruncatedStuff { at: 7 }));
	}
}
