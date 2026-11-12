use r1_bitstream::pipeline;
use std::fs;

#[test]
fn test() {
	let read_sting = fs::read_to_string("./fixtures//ax25_frame.hex").expect("Should have been able to read the file");
	let contents = read_sting.chars();

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

	assert_eq!(pipeline::decode(&pipeline::encode(&payload)), Ok(payload));
}
