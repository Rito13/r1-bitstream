use super::bitbuf::BitBuffer;

struct Coder {
	sr: u32,
}

impl Coder {
	const SR_MASK: u32 = 0x1FFFF;
	const SR_BIT_11_INDEX: u32 = 11;
	const SR_BIT_16_INDEX: u32 = 16;
	const SR_BIT_11: u32 = 1 << Coder::SR_BIT_11_INDEX;
	const SR_BIT_16: u32 = 1 << Coder::SR_BIT_16_INDEX;

	#[inline]
	fn feedback(&mut self) -> bool {
		(((self.sr & Coder::SR_BIT_11) >> Coder::SR_BIT_11_INDEX) ^ ((self.sr & Coder::SR_BIT_16) >> Coder::SR_BIT_16_INDEX)) == 1
	}

	#[inline]
	fn update_sr(&mut self, b: bool) {
		self.sr <<= 1;
		self.sr |= b as u32;
		self.sr &= Coder::SR_MASK;
	}
}

struct Encoder(Coder);
struct Decoder(Coder);

impl Encoder {
	fn encode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for mut b in bits {
			b ^= self.0.feedback();
			output.push_bit(b);
			self.0.update_sr(b);
		}
		output
	}

	fn new() -> Self {
		Encoder(Coder { sr: 0 })
	}
}

impl Decoder {
	fn decode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for b in bits {
			output.push_bit(b ^ self.0.feedback());
			self.0.update_sr(b);
		}
		output
	}

	fn new() -> Self {
		Decoder(Coder { sr: 0 })
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
