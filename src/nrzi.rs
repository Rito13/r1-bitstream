use super::bitbuf::BitBuffer;

struct Encoder<'encoder> {
	output: &'encoder mut BitBuffer,
}

struct Decoder<'decoder> {
	output: &'decoder mut BitBuffer,
	current_level: bool,
}

impl Encoder<'_> {
	fn encode(&mut self, bits: &BitBuffer) {
		for b in bits {
			match self.output.last_bit() {
				None => {
					self.output.push_bit(b);
				}
				Some(value) => {
					self.output.push_bit(value ^ !b); // XOR will flip the value if !b == 1.
				}
			}
		}
	}

	fn new<'encoder>(output: &'encoder mut BitBuffer) -> Encoder<'encoder> {
		Encoder { output: output }
	}
}

impl Decoder<'_> {
	fn decode(&mut self, bits: &BitBuffer) {
		for b in bits {
			self.output.push_bit(b == self.current_level);
			self.current_level = b;
		}
	}

	fn new<'encoder>(output: &'encoder mut BitBuffer) -> Decoder<'encoder> {
		Decoder { output: output, current_level: true }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn table_compare() {
		for bytes in [[0x00u8, 0x55], [0xFF, 0xFF], [0x55, 0x33], [0xAA, 0x99]] {
			let input = BitBuffer::from_bytes(&[bytes[0]]);
			let mut output = BitBuffer::from_bytes(&[]);
			let mut encoder = Encoder::new(&mut output);
			encoder.encode(&input);
			assert_eq!(output.to_bytes()[0], bytes[1], "encode({})", bytes[0]);
		}

		for bytes in [[0x00u8, 0x55], [0xFF, 0xFF], [0x55, 0x33], [0xAA, 0x99]] {
			let input = BitBuffer::from_bytes(&[bytes[1]]);
			let mut output = BitBuffer::from_bytes(&[]);
			let mut decoder = Decoder::new(&mut output);
			decoder.decode(&input);
			assert_eq!(output.to_bytes()[0], bytes[0], "decode({})", bytes[1]);
		}
	}
}
