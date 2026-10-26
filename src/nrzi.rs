use super::bitbuf::BitBuffer;

struct Encoder {
	current_level: bool,
}

struct Decoder {
	current_level: bool,
}

impl Encoder {
	fn encode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for b in bits {
			self.current_level ^= !b; // XOR will flip the value if !b == 1.
			output.push_bit(self.current_level);
		}
		output
	}

	fn new() -> Self {
		Encoder { current_level: true }
	}
}

impl Decoder {
	fn decode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for b in bits {
			output.push_bit(b == self.current_level);
			self.current_level = b;
		}
		output
	}

	fn new() -> Self {
		Decoder { current_level: true }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn table_compare() {
		for bytes in [[0x00u8, 0x55], [0xFF, 0xFF], [0x55, 0x33], [0xAA, 0x99]] {
			let input = BitBuffer::from_bytes(&[bytes[0]]);
			let mut encoder = Encoder::new();
			let output = encoder.encode(&input);
			assert_eq!(output.to_bytes()[0], bytes[1], "encode({})", bytes[0]);
		}

		for bytes in [[0x00u8, 0x55], [0xFF, 0xFF], [0x55, 0x33], [0xAA, 0x99]] {
			let input = BitBuffer::from_bytes(&[bytes[1]]);
			let mut decoder = Decoder::new();
			let output = decoder.decode(&input);
			assert_eq!(output.to_bytes()[0], bytes[0], "decode({})", bytes[1]);
		}
	}

	#[test]
	fn encode_decode() {
		let mut tests = [
			BitBuffer::from_bytes(&[0x00u8, 0x55, 0x13]),
			BitBuffer::from_bytes(&[0xFF, 0x88, 0xFF]),
			BitBuffer::from_bytes(&[0x55, 0x33, 0x17]),
			BitBuffer::from_bytes(&[0xAA, 0x99, 0x9A]),
			BitBuffer::from_bytes(&[]),
		];

		tests[2].push_bit(true);
		tests[3].push_bit(true);
		tests[3].push_bit(false);

		for input in tests {
			let mut encoder = Encoder::new();
			let mut decoder = Decoder::new();

			let in_between = encoder.encode(&input);
			let output = decoder.decode(&in_between);

			assert_eq!(input, output);
		}
	}
}
