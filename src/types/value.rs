#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub enum ValueType {
    /// A signed 8-bit integer
    Int8,
    /// A signed 16-bit integer
    Int16,
    /// A signed 32-bit integer
    Int32,
    /// A signed 64-bit integer
    Int64,
    /// An unsigned 8-bit integer
    UInt8,
    /// An unsigned 16-bit integer
    UInt16,
    /// An unsigned 32-bit integer
    UInt32,
    /// An unsigned 64-bit integer
    UInt64,
    /// A 32-bit floating point value
    Float32,
    /// A 64-bit floating point value
    Float64,
    /// A sequence of UTF-8 graphemes
    String,
    /// A list of values.
    Array,
    // A key-value assignment of strings and values.
    Map,
}