use super::bitbuf::BitBuffer;
use super::bitbuf::BitBufferIterator;
use super::biterror::BitError;
use super::nrzi;
use super::scramble;
use super::stuffing;

/// Perform scrambling on given bits.
///
/// # Parameters
/// * `input`: The bits to scramble.
///
/// # Returns
/// Scrambled bits.
pub fn scramble(input: &BitBuffer) -> BitBuffer {
	scramble::Encoder::new().encode(input.into_iter())
}

/// Perform descrambling on given bits.
///
/// # Parameters
/// * `input`: The bits to descramble.
///
/// # Returns
/// Descrambled bits.
pub fn descramble(input: &BitBuffer) -> BitBuffer {
	scramble::Decoder::new().decode(input.into_iter())
}

/// Encode given bits with NRZI.
///
/// # Parameters
/// * `input`: The bits to encode.
///
/// __Returns:__ Encoded bits.
pub fn nrzi_encode(input: &BitBuffer) -> BitBuffer {
	nrzi::Encoder::new().encode(input.into_iter())
}

/// Decode given bits with NRZI.
///
/// # Parameters
/// * `input`: The bits to decode.
///
/// __Returns:__ Decoded bits.
pub fn nrzi_decode(input: &BitBuffer) -> BitBuffer {
	nrzi::Decoder::new().decode(input.into_iter())
}

/// Perform bit stuffing on given bits.
///
/// # Parameters
/// * `input`: The bits stuff.
///
/// __Returns:__ The stuffed bits.
pub fn stuff(input: &BitBuffer) -> BitBuffer {
	stuffing::Encoder::new().encode(input.into_iter())
}

/// Perform bit destuffing on given bits.
///
/// # Parameters
/// * `input`: The bits destuff.
///
/// __Returns:__ The destuffed bits or an appropriate error.
pub fn destuff(input: &BitBuffer) -> Result<BitBuffer, BitError> {
	stuffing::Decoder::new().decode(input.into_iter())
}

/// Encode data so it can be safely send over the radio.
///
/// # Parameters
/// * `payload`: The data to encode.
///
/// __Returns:__ Encoded data ready to send over the radio.
pub fn encode(payload: &[u8]) -> BitBuffer {
	scramble(&nrzi_encode(&stuff(&BitBuffer::from_bytes(payload))))
}

/// Decode message received by radio so the data contained in it can be retrieved.
///
/// # Parameters
/// * `air`: The received message.
///
/// __Returns:__ Decoded data for further examination or an error if the message could not be decoded.
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

/// Perform bit destuffing on given bits without terminating on errors.
///
/// # Parameters
/// * `input`: The bits destuff.
///
/// __Returns:__ The destuffed bits and a vector with appropriate errors.
fn destuff_lossy(input: BitBufferIterator) -> (Vec<u8>, Vec<BitError>) {
	let mut out = stuffing::Decoder::new().decode_lossy(input);
	if !out.0.len().is_multiple_of(8) {
		out.1.push(BitError::NotByteAligned { len: out.0.len() });
	}
	(out.0.to_bytes(), out.1)
}

/// Decode message received by radio so the data contained in it can be retrieved even if an error occured.
///
/// # Parameters
/// * `air`: The received message.
///
/// __Returns:__ Decoded data for further examination and a vector of errors if the message could not be decoded.
pub fn decode_lossy(air: &BitBuffer) -> (Vec<u8>, Vec<BitError>) {
	destuff_lossy(nrzi_decode(&descramble(air)).into_iter())
}

/// Decode message received by radio with unknown sate so the data contained in it can be retrieved.
///
/// # Parameters
/// * `air`: The received part of a message.
///
/// __Returns:__ As like decode_lossy. Decoded data for further examination and a vector of errors if the message could not be decoded.
pub fn decode_from_middle(air: &BitBuffer) -> (Vec<u8>, Vec<BitError>) {
	let descrambled = descramble(air);

	if descrambled.len() < 17 {
		return (vec![], vec![BitError::ToShortToSynchronise]);
	}

	let mut iter = descrambled.into_iter();
	iter.nth(16); // Synchronise.
	let nrzi_decoded = nrzi::Decoder::new().decode(iter);
	let mut iter = nrzi_decoded.into_iter();
	iter.next(); // First bit may be incorrect as we don't know the previous state.

	/* Skip first 0 as we don't know how many ones there where before it. */
	while match iter.next() {
		Some(v) => v,
		None => return (vec![], vec![BitError::ToShortToSynchronise]),
	} {}

	destuff_lossy(iter)
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
