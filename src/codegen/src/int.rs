/// An empty struct to convert integer types into 64-bit word arrays.
pub struct To64LLVMWord;

impl To64LLVMWord {
    /// Convert a u8 to a 64-bit word array.
    pub fn from_u8(value: u8) -> [u64; 1] {
        [value as u64]
    }

    /// Convert a u16 to a 64-bit word array.
    pub fn from_u16(value: u16) -> [u64; 1] {
        [value as u64]
    }

    /// Convert a u32 to a 64-bit word array.
    pub fn from_u32(value: u32) -> [u64; 1] {
        [value as u64]
    }

    /// Convert a u64 to a 64-bit word array.
    pub fn from_u64(value: u64) -> [u64; 1] {
        [value as u64]
    }

    /// Convert a u128 to a 64-bit word array.
    pub fn from_u128(value: u128) -> [u64; 2] {
        [
            (value & u64::MAX as u128) as u64,
            ((value >> 64) & u64::MAX as u128) as u64
        ]
    }

    /// Convert a i8 to a 64-bit word array.
    pub fn from_i8(value: i8) -> [u64; 1] {
        [((value & 0x7F) as u64) | (if value < 0 { !(0x7F) } else { 0 })]
    }

    /// Convert a i16 to a 64-bit word array.
    pub fn from_i16(value: i16) -> [u64; 1] {
        [((value & 0x7FFF) as u64) | (if value < 0 { !(0x7FFF) } else { 0 })]
    }

    /// Convert a i32 to a 64-bit word array.
    pub fn from_i32(value: i32) -> [u64; 1] {
        [((value & 0x7FFF_FFFF) as u64) | (if value < 0 { !(0x7FFF_FFFF) } else { 0 })]
    }

    /// Convert a i64 to a 64-bit word array.
    pub fn from_i64(value: i64) -> [u64; 1] {
        Self::from_u64(unsafe { *(&value as *const i64 as *const u64) })
    }

    /// Convert a i128 to a 64-bit word array.
    pub fn from_i128(value: i128) -> [u64; 2] {
        let bytes = value.to_be_bytes();
        [
            u64::from_be_bytes([
                bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14],
                bytes[15]
            ]),
            u64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]
            ])
        ]
    }
}
