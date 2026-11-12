use r1_bitstream::pipeline;
use std::fs;

#[test]
fn test() {
	let read_sting = fs::read_to_string("./fixtures//ax25_frame.hex").expect("Should have been able to read the file");
	let contents = read_sting.chars();

	let mut payload: Vec<u8> = vec![];
	let mut is_second = false;

	for c in contents {
		let value = if c <= '9' && c >= '0' {
			c as u8 - '0' as u8
		} else if c <= 'f' && c >= 'a' {
			c as u8 - 'a' as u8 + 10
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
