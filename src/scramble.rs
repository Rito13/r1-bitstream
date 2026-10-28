use super::bitbuf::BitBuffer;

struct Encoder {
	sr: u32,
}

impl Encoder {
	const SR_MASK: u32 = 0x1FFFF;
	const SR_BIT_11_INDEX: u32 = 11;
	const SR_BIT_16_INDEX: u32 = 16;
	const SR_BIT_11: u32 = 1 << Encoder::SR_BIT_11_INDEX;
	const SR_BIT_16: u32 = 1 << Encoder::SR_BIT_16_INDEX;

	fn encode(&mut self, bits: &BitBuffer) -> BitBuffer {
		let mut output = BitBuffer::from_bytes(&[]);
		for mut b in bits {
			b ^= (((self.sr & Encoder::SR_BIT_11) >> Encoder::SR_BIT_11_INDEX) ^ ((self.sr & Encoder::SR_BIT_16) >> Encoder::SR_BIT_16_INDEX)) == 1;
			output.push_bit(b);
			self.sr <<= 1;
			self.sr |= b as u32;
			self.sr &= Encoder::SR_MASK;
		}
		output
	}

	fn new() -> Self {
		Encoder { sr: 0 }
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
}
