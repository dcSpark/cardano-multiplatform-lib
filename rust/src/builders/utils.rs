/// this value is large enough to trigger cbor_event to force writing a u64 cbor type
/// when used inside `write_type`.
/// This is useful to calculate payloads assuming the max possible size that can be used
/// since u64 in CBOR are fixed size (will prefix with 0s to stay a constant width regardless of value)
pub const force_u64: u64 = 0x01_00_00_00_00; 