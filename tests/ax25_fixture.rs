use r1_bitstream::pipeline;
use r1_bitstream::bitbuf::BitBuffer;
use std::fs;

/// Parse string with hex representation of bytes.
///
/// __Parameters:__
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
	let mut i = 1usize;
	while air.len() - i > 17 {
		let mut cropped_air = BitBuffer::from_bytes(&[]);
		let mut j = 0;
		for b in &air {
			if j == i {
				cropped_air.push_bit(b);
			} else {
				j += i;
			}
		}
		let received = BitBuffer::from_bytes(&pipeline::decode_lossy(&cropped_air).0);
		for j in 16..received.len() {
			assert_eq!(received.bit(j), p_buffer.bit(j + i));
		}
		i += 1;
	}
}
