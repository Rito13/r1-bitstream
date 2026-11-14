use super::bitbuf::BitBuffer;

/** Common base of en/decoder of NRZI process. */
struct Coder {
	/** The latest encoded or decoded level. Can be either high or low. */
	current_level: bool,
}

impl Default for Coder {
	/** Construct a new Coder instance with default state. */
	fn default() -> Self {
		Coder { current_level: true }
	}
}

/** Encoder of NRZI process. */
#[derive(Default)]
pub struct Encoder(Coder);

/** Decoder of NRZI process. */
#[derive(Default)]
pub struct Decoder(Coder);

impl Encoder {
	/// Serialize bits from buffer into Non-return-to-zero inverted code.
	/// Zero is coded by difference in levels and one by constant state.
	///
	/// __Parameters:__
	/// * `bits`: The bit buffer to serialize.
	///
	/// __Returns:__ The buffer with bits coded by NRZI.
	pub fn encode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for b in bits {
			self.0.current_level ^= !b; // XOR will flip the value if !b == 1.
			output.push_bit(self.0.current_level);
		}
		output
	}

	/** Construct a new Encoder instance with default state. */
	pub fn new() -> Self {
		Default::default()
	}
}

impl Decoder {
	/// Deserialize bits from buffer coded with Non-return-to-zero inverted.
	/// Difference in levels is decoded to zero and constant state into one.
	///
	/// __Parameters:__
	/// * `bits`: The bit buffer to deserialize.
	///
	/// __Returns:__ The buffer containing decoded data.
	pub fn decode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for b in bits {
			output.push_bit(b == self.0.current_level);
			self.0.current_level = b;
		}
		output
	}

	/** Construct a new Decoder instance with default state. */
	pub fn new() -> Self {
		Default::default()
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
