use r1_bitstream::bitbuf::BitBuffer;
use r1_bitstream::biterror::BitError;
use r1_bitstream::pipeline;
use std::fs;

/// Parse string with hex representation of bytes.
///
/// # Parameters
/// * 'string': The string to parse.
///
/// __Returns:__ Vector with parsed bytes.
fn parse_hex_string(string: String) -> Vec<u8> {
	let contents = string.chars();

	let mut payload: Vec<u8> = vec![];
	let mut is_second = false;

	for c in contents {
		let value = if c.is_ascii_digit() {
			c as u8 - b'0'
		} else if ('a'..='f').contains(&c) {
			c as u8 - b'a' + 10
		} else if c == '\n' {
			break;
		} else {
			panic!("Could not convert 0x{c} to 4 bits.")
		};

		if is_second {
			let index = payload.len() - 1;
			payload[index] += value;
		} else {
			payload.push(value << 4);
		}

		is_second = !is_second;
	}

	payload
}

#[test]
fn encode_decode() {
	let read_string = fs::read_to_string("./fixtures//ax25_frame.hex").expect("Should have been able to read the file");
	let payload = parse_hex_string(read_string);
	assert_eq!(pipeline::decode(&pipeline::encode(&payload)), Ok(payload));
}

#[test]
fn from_middle() {
	let read_string = fs::read_to_string("./fixtures//ax25_frame.hex").expect("Should have been able to read the file");
	let payload = parse_hex_string(read_string);
	let p_buffer = BitBuffer::from_bytes(&payload);
	let air = pipeline::encode(&payload);
	let mut i = 2usize;
	while air.len() - i > 18 {
		/* Simulate reading the message from the middle. */
		let mut cropped_air = BitBuffer::from_bytes(&[]);
		let mut j = 0;
		for b in &air {
			if j == i {
				cropped_air.push_bit(b);
			} else {
				j += 1;
			}
		}

		/* Decode the received part of the message. */
		let mut received = pipeline::decode_from_middle(&cropped_air);

		/* Calculate how many bits from the original message we have lost. */
		let start = p_buffer.len() - received.0.len() * 8
			+ match received.1.pop() {
				None => 0,
				Some(error) => match error {
					BitError::NotByteAligned { len } => {
						print!("[");
						for v in &received.0 {
							print!("{v}, ")
						}
						println!("] - {len}");
						8 - len % 8
					}
					error => panic!("{error}"),
				},
			};

		/* Cut from the original message a slice that we should have received. */
		let mut cropped_p_buffer = BitBuffer::from_bytes(&[]);
		for j in start..p_buffer.len() {
			cropped_p_buffer.push_bit(p_buffer.bit(j).expect(""));
		}

		assert_eq!(received.0, cropped_p_buffer.to_bytes()); // Compare the decoded data with expected one.

		i += 1;
	}
}
