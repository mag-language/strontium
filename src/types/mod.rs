/// A reference to a location in memory
pub type Pointer = u64;

/// Defines a numeric value on which arithmetic may be performed
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Numeric {
	Int {
		value: i64
	},

	UInt {
		value: u64
	},

	Float {
		value: f64,
	}
}