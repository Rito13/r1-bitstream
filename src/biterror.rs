#[derive(Debug, Clone, PartialEq, Eq)]
/** Errors risen while processing the bit buffers. */
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

impl std::fmt::Display for BitError {
	/**
	 * Formats the BitError using the given formatter.
	 *
	 * __Parameters:__
	 * * `f`: The formatter to use.
	 *
	 * __Returns__: Error or a success value.
	 */
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			BitError::SixOnes { at } => {
				write!(f, "Destuffing failed: six ones in a row encountered at {at}.")
			}
			BitError::TruncatedStuff { at } => {
				write!(f, "Destuffing failed: buffer ended with five consecutive ones at {at}.")
			}
			BitError::NotByteAligned { len } => {
				write!(f, "Decoding failed: destuffed buffer is of length {len} which is not divisible by 8.")
			}
		}
	}
}

impl std::error::Error for BitError {}
