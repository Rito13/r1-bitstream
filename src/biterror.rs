#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitError {
	SixOnes { at: usize },
	TruncatedStuff { at: usize },
	NotByteAligned { len: usize },
}

/*
impl std::fmt::Display for BitError {
	/* ... */
}
impl std::error::Error for BitError {}
*/
