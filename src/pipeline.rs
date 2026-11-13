use super::bitbuf::BitBuffer;
use super::biterror::BitError;
use super::nrzi;
use super::scramble;
use super::stuffing;

///Perform scrambling on given bits.
///
///__Parameters:__
///* `input`: The bits to scramble.
///
///__Returns__: Scrambled bits.
pub fn scramble(input: &BitBuffer) -> BitBuffer {
	scramble::Encoder::new().encode(input)
}

///Perform descrambling on given bits.
///
///__Parameters:__
///* `input`: The bits to descramble.
///
///__Returns__: Descrambled bits.
pub fn descramble(input: &BitBuffer) -> BitBuffer {
	scramble::Decoder::new().decode(input)
}

///Encode given bits with NRZI.
///
///__Parameters:__
///* `input`: The bits to encode.
///
///__Returns:__ Encoded bits.
pub fn nrzi_encode(input: &BitBuffer) -> BitBuffer {
	nrzi::Encoder::new().encode(input)
}

///Decode given bits with NRZI.
///
///__Parameters:__
///* `input`: The bits to decode.
///
///__Returns:__ Decoded bits.
pub fn nrzi_decode(input: &BitBuffer) -> BitBuffer {
	nrzi::Decoder::new().decode(input)
}

///Perform bit stuffing on given bits.
///
///__Parameters:__
///* `input`: The bits stuff.
///
///__Returns:__ The stuffed bits.
pub fn stuff(input: &BitBuffer) -> BitBuffer {
	stuffing::Encoder::new().encode(input)
}

///Perform bit destuffing on given bits.
///
///__Parameters:__
///* `input`: The bits destuff.
///
///__Returns:__ The destuffed bits or an appropriate error.
pub fn destuff(input: &BitBuffer) -> Result<BitBuffer, BitError> {
	stuffing::Decoder::new().decode(input)
}

///Encode data so it can be safely send over the radio.
///
///__Parameters:__
///* `payload`: The data to encode.
///
///__Returns:__ Encoded data ready to send over the radio.
pub fn encode(payload: &[u8]) -> BitBuffer {
	scramble(&nrzi_encode(&stuff(&BitBuffer::from_bytes(payload))))
}

///Decode message received by radio so the data contained in it can be retrieved.
///
///__Parameters:__
///* `air`: The received message.
///
///__Returns:__ Decoded data for further examination or an error if the message could not be decoded.
pub fn decode(air: &BitBuffer) -> Result<Vec<u8>, BitError> {
	match destuff(&nrzi_decode(&descramble(air))) {
		Ok(bits) => {
			if bits.len() % 8 != 0 {
				return Err(BitError::NotByteAligned { len: bits.len() });
			}
			Ok(bits.to_bytes())
		}
		Err(error) => Err(error),
	}
}

///Decode message received by radio so the data contained in it can be retrieved even if an error occured.
///
///__Parameters:__
///* `air`: The received message.
///
///__Returns:__ Decoded data for further examination and an error if the message could not be decoded.
pub fn decode_lossy(air: &BitBuffer) -> (Vec<u8>, Option<BitError>) {
	match destuff(&nrzi_decode(&descramble(air))) {
		Ok(bits) => {
			if bits.len() % 8 != 0 {
				return (bits.to_bytes(), Some(BitError::NotByteAligned { len: bits.len() }));
			}
			(bits.to_bytes(), None)
		}
		Err(error) => (vec![], Some(error)),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn empty_input() {
		let empty_buffer = BitBuffer::from_bytes(&[]);
		assert_eq!(encode(&[]), empty_buffer);
		assert_eq!(decode(&empty_buffer), Ok(vec![]));
	}

	#[test]
	fn ping() {
		assert_eq!(decode(&encode(b"ping")), Ok(vec![b'p', b'i', b'n', b'g']));
	}

	#[test]
	fn not_byte_aligned() {
		let bytes = [0xFBu8, 0xEF]; // 16 bits: 5 ones, 1 zero, 5 ones, 1 zero, 4 ones
		let buffer = scramble(&nrzi_encode(&BitBuffer::from_bytes(&bytes)));
		assert_eq!(decode(&buffer), Err(BitError::NotByteAligned { len: 14 }));
	}
}
