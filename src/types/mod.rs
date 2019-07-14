#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Location {
	Memory(MemoryAddress),
	Register(RegisterAddress),
}

/// A reference to a location in memory
pub type MemoryAddress = u64;

/// A reference to a floating-point register
pub type RegisterAddress = u8;
