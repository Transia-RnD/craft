use crate::host::add_txn_field;
use crate::core::types::account_id::AccountID;

// XRPL serialization markers
const ARRAY_END: u8 = 0xF1;
const OBJECT_END: u8 = 0xE1;

// Maximum sizes for buffers
const MAX_BUFFER_SIZE: usize = 1024;
const MAX_VL_PREFIX_SIZE: usize = 3;

// ============= Simple field setters (no type byte needed) =============

#[inline]
pub fn set_txn_uint8(index: i32, field: i32, value: u8) -> i32 {
    unsafe {
        add_txn_field(index, field, &value as *const u8, 1)
    }
}

#[inline]
pub fn set_txn_uint16(index: i32, field: i32, value: u16) -> i32 {
    let bytes = value.to_be_bytes();
    unsafe {
        add_txn_field(index, field, bytes.as_ptr(), 2)
    }
}

#[inline]
pub fn set_txn_uint32(index: i32, field: i32, value: u32) -> i32 {
    let bytes = value.to_be_bytes();
    unsafe {
        add_txn_field(index, field, bytes.as_ptr(), 4)
    }
}

#[inline]
pub fn set_txn_uint64(index: i32, field: i32, value: u64) -> i32 {
    let bytes = value.to_be_bytes();
    unsafe {
        add_txn_field(index, field, bytes.as_ptr(), 8)
    }
}

#[inline]
pub fn set_txn_uint128(index: i32, field: i32, value: &[u8; 16]) -> i32 {
    unsafe {
        add_txn_field(index, field, value.as_ptr(), 16)
    }
}

#[inline]
pub fn set_txn_uint160(index: i32, field: i32, value: &[u8; 20]) -> i32 {
    unsafe {
        add_txn_field(index, field, value.as_ptr(), 20)
    }
}

#[inline]
pub fn set_txn_uint192(index: i32, field: i32, value: &[u8; 24]) -> i32 {
    unsafe {
        add_txn_field(index, field, value.as_ptr(), 24)
    }
}

#[inline]
pub fn set_txn_uint256(index: i32, field: i32, value: &[u8; 32]) -> i32 {
    unsafe {
        add_txn_field(index, field, value.as_ptr(), 32)
    }
}

// Amount field - already serialized correctly (without type byte)
#[inline]
pub fn set_txn_amount(index: i32, field: i32, amount_bytes: &[u8]) -> i32 {
    unsafe {
        add_txn_field(index, field, amount_bytes.as_ptr(), amount_bytes.len())
    }
}

// Account field - needs length prefix but no type byte
#[inline]
pub fn set_txn_account(index: i32, field: i32, account: &AccountID) -> i32 {
    let mut buffer = [0u8; 21];
    buffer[0] = 0x14; // Length prefix for 20-byte account
    buffer[1..21].copy_from_slice(&account.0);
    
    unsafe {
        add_txn_field(index, field, buffer.as_ptr(), 21)
    }
}

// Currency field - direct 20 bytes, no type byte
#[inline]
pub fn set_txn_currency(index: i32, field: i32, currency: &[u8; 20]) -> i32 {
    unsafe {
        add_txn_field(index, field, currency.as_ptr(), 20)
    }
}

// VL encoding helper
#[inline]
pub fn encode_vl_length(buffer: &mut [u8], pos: usize, len: usize) -> usize {
    if len <= 192 {
        buffer[pos] = len as u8;
        1
    } else if len <= 12480 {
        let encoded = len - 193;
        buffer[pos] = 193 + (encoded >> 8) as u8;
        buffer[pos + 1] = (encoded & 0xff) as u8;
        2
    } else if len <= 918744 {
        let encoded = len - 12481;
        buffer[pos] = 241 + (encoded >> 16) as u8;
        buffer[pos + 1] = ((encoded >> 8) & 0xff) as u8;
        buffer[pos + 2] = (encoded & 0xff) as u8;
        3
    } else {
        0 // Error case
    }
}

// Blob/VL field - needs length prefix but no type byte
#[inline]
pub fn set_txn_blob(index: i32, field: i32, data: &[u8]) -> i32 {
    if data.len() > MAX_BUFFER_SIZE - MAX_VL_PREFIX_SIZE {
        return -1;
    }
    
    let mut buffer = [0u8; MAX_BUFFER_SIZE];
    let len_bytes = encode_vl_length(&mut buffer, 0, data.len());
    if len_bytes == 0 {
        return -1;
    }
    
    buffer[len_bytes..len_bytes + data.len()].copy_from_slice(data);
    
    unsafe {
        add_txn_field(index, field, buffer.as_ptr(), len_bytes + data.len())
    }
}

// ============= Fixed-size buffer for building complex fields =============

pub struct FixedBuffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> FixedBuffer<N> {
    pub fn new() -> Self {
        Self {
            data: [0u8; N],
            len: 0,
        }
    }

    pub fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.len >= N {
            return Err("Buffer overflow");
        }
        self.data[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    pub fn extend(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        if self.len + bytes.len() > N {
            return Err("Buffer overflow");
        }
        self.data[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

// ============= Helper for encoding field headers in objects =============

/// Encode field header byte for objects/arrays
pub fn encode_field_header(field_id: i32) -> (u8, Option<[u8; 2]>) {
    let type_code = ((field_id >> 16) & 0xFF) as u8;
    let field_code = (field_id & 0xFF) as u8;
    
    if type_code < 16 && field_code < 16 {
        // Both fit in 4 bits each - single byte encoding
        ((type_code << 4) | field_code, None)
    } else if type_code < 16 {
        // Type fits in 4 bits, field needs extended encoding
        let header = type_code << 4; // Upper nibble = type, lower nibble = 0
        let extended = [field_code, 0];
        (header, Some(extended))
    } else if field_code < 16 {
        // Field fits in 4 bits, type needs extended encoding
        let header = field_code; // Upper nibble = 0, lower nibble = field
        let extended = [type_code, 0];
        (header, Some(extended))
    } else {
        // Both need extended encoding
        let header = 0; // Both nibbles = 0
        let extended = [type_code, field_code];
        (header, Some(extended))
    }
}

// ============= Example: Building a simple Parameter object =============

// /// Build a Parameter object with name and flag
// /// Returns the number of bytes written, or negative error
// pub fn build_parameter_object(
//     buffer: &mut [u8; 256],
//     param_name: &[u8],
//     param_flag: u32
// ) -> i32 {
//     let mut pos = 0;
    
//     // Add ParameterName field (SF_PARAMETER_NAME = 786467)
//     let (header, extended) = encode_field_header(786467);
//     buffer[pos] = header;
//     pos += 1;
    
//     if let Some(ext) = extended {
//         buffer[pos..pos + 2].copy_from_slice(&ext);
//         pos += 2;
//     }
    
//     // Add VL length prefix for name
//     let len_bytes = encode_vl_length(buffer, pos, param_name.len());
//     if len_bytes == 0 {
//         return -1;
//     }
//     pos += len_bytes;
    
//     // Add name data
//     if pos + param_name.len() > buffer.len() {
//         return -1;
//     }
//     buffer[pos..pos + param_name.len()].copy_from_slice(param_name);
//     pos += param_name.len();
    
//     // Add ParameterFlag field (SF_PARAMETER_FLAG = 196666)
//     let (header, extended) = encode_field_header(196666);
//     buffer[pos] = header;
//     pos += 1;
    
//     if let Some(ext) = extended {
//         buffer[pos..pos + 2].copy_from_slice(&ext);
//         pos += 2;
//     }
    
//     // Add flag value (4 bytes)
//     let flag_bytes = param_flag.to_be_bytes();
//     buffer[pos..pos + 4].copy_from_slice(&flag_bytes);
//     pos += 4;
    
//     // Add object end marker
//     buffer[pos] = OBJECT_END;
//     pos += 1;
    
//     pos as i32
// }

// // ============= Convenience functions for common fields =============

// #[inline]
// pub fn set_txn_transaction_type(index: i32, tt: u16) -> i32 {
//     set_txn_uint16(index, 131074, tt) // SF_TRANSACTION_TYPE
// }

// #[inline]
// pub fn set_txn_flags(index: i32, flags: u32) -> i32 {
//     set_txn_uint32(index, 196610, flags) // SF_FLAGS
// }

// #[inline]
// pub fn set_txn_sequence(index: i32, sequence: u32) -> i32 {
//     set_txn_uint32(index, 196612, sequence) // SF_SEQUENCE
// }

// #[inline]
// pub fn set_txn_fee_amount(index: i32, fee_bytes: &[u8]) -> i32 {
//     set_txn_amount(index, 720904, fee_bytes) // SF_FEE
// }

// #[inline]
// pub fn set_txn_signing_pub_key(index: i32, pubkey: &[u8]) -> i32 {
//     set_txn_blob(index, 786435, pubkey) // SF_SIGNING_PUB_KEY
// }