use super::bitbuf::BitBuffer;

/** Common base for Encoder and Decoder of scrambling process. */
struct Coder {
	/** The current state of the encoder/decoder. */
	sr: u32,
}

impl Coder {
	/** Mask containing all used bits from `sr`. */
	const SR_MASK: u32 = 0x1FFFF;
	/** Index of bit 11 in `sr`. */
	const SR_BIT_11_INDEX: u32 = 11;
	/** Index of bit 16 in `sr`. */
	const SR_BIT_16_INDEX: u32 = 16;
	/** Mask that allows to extract bit 11 from `sr`. */
	const SR_BIT_11: u32 = 1 << Coder::SR_BIT_11_INDEX;
	/** Mask that allows to extract bit 16 from `sr`. */
	const SR_BIT_16: u32 = 1 << Coder::SR_BIT_16_INDEX;

	///Calculate the value of feedback from current state of `sr`.
	///
	///__Returns:__ The calculated single bit feedback.
	#[inline]
	fn feedback(&mut self) -> bool {
		(((self.sr & Coder::SR_BIT_11) >> Coder::SR_BIT_11_INDEX) ^ ((self.sr & Coder::SR_BIT_16) >> Coder::SR_BIT_16_INDEX)) == 1
	}

	///Update the value of `sr` after an bit was encoded/decoded.
	///
	///__Parameters:__
	///* `b`: The encoded bit that was encoded or the encoded bit that was decoded.
	#[inline]
	fn update_sr(&mut self, b: bool) {
		self.sr <<= 1;
		self.sr |= b as u32;
		self.sr &= Coder::SR_MASK;
	}
}

impl Default for Coder {
	/** Construct new Coder instance with a default state. */
	fn default() -> Self {
		Coder { sr: 0 }
	}
}

/** Encoder of scrambling process. */
#[derive(Default)]
pub struct Encoder(Coder);

/** Decoder of scrambling process. */
#[derive(Default)]
pub struct Decoder(Coder);

impl Encoder {
	///Perform the scrambling process on given bits.
	///
	///__Parameters:__
	///* `bits`: The bits to scramble.
	///
	///__Returns__: Scrambled bits.
	pub fn encode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for mut b in bits {
			b ^= self.0.feedback();
			output.push_bit(b);
			self.0.update_sr(b);
		}
		output
	}

	/** Construct a new Encoder instance. */
	pub fn new() -> Self {
		Default::default()
	}
}

impl Decoder {
	///Perform the descrambling process on given bits.
	///
	///__Parameters:__
	///* `bits`: The bits to descramble.
	///
	///__Returns__: Descrambled bits.
	pub fn decode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for b in bits {
			output.push_bit(b ^ self.0.feedback());
			self.0.update_sr(b);
		}
		output
	}

	/** Construct a new Decoder instance. */
	pub fn new() -> Self {
		Default::default()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn encode_123456789() {
		let bit_buffer = BitBuffer::from_bytes(b"123456789");
		for b in bit_buffer.to_bytes() {
			println!("0x{:X}", b);
		}

		let mut encoder = Encoder::new();
		assert_eq!(encoder.encode(&bit_buffer), BitBuffer::from_bytes(&[0x31, 0x31, 0x38, 0xbf, 0x22, 0x9b, 0x8f, 0xcd, 0x02]));
	}

	#[test]
	fn encode_decode() {
		let tests = [
			BitBuffer::from_bytes(&[0x00]),
			BitBuffer::from_bytes(&[0xFF]),
			BitBuffer::from_bytes(b"ping"),
			BitBuffer::from_bytes(b"123456789"),
			BitBuffer::from_bytes(&[]),
		];

		for input in tests {
			let mut encoder = Encoder::new();
			let mut decoder = Decoder::new();

			let in_between = encoder.encode(&input);
			let output = decoder.decode(&in_between);

			assert_eq!(input, output);
		}
	}
}
