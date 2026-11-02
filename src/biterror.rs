#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitError {
	/** Destuffing process failed after encountering six ones in a row. */
	SixOnes {
		/** The index of sixth one in inputted buffer. */
		at: usize,
	},
	/** Destuffing process failed after encountering five ones in a row at the end of the buffer. */
	TruncatedStuff {
		/** The index of the last one in inputted buffer. One less than the length of the buffer. */
		at: usize,
	},
	/** Decoding process failed because length of destuffed buffer was not divisible by 8. */
	NotByteAligned {
		/** The encountered length of destuffed buffer. */
		len: usize,
	},
}

/*
impl std::fmt::Display for BitError {
	/* ... */
}
impl std::error::Error for BitError {}
*/
