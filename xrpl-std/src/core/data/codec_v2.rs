use crate::host::{set_contract_data_from_key, get_contract_data_from_key, set_nested_contract_data_from_key, get_nested_contract_data_from_key};
use crate::host::trace::{trace, trace_num, trace_data, DataRepr};
use crate::core::types::account_id::AccountID;
use crate::core::type_codes::{
    STI_UINT8, STI_UINT16, STI_UINT32, STI_UINT64, STI_UINT128, 
    STI_UINT160, STI_UINT256, STI_AMOUNT, STI_VL, STI_ACCOUNT, 
    STI_OBJECT, STI_ARRAY, STI_CURRENCY
};

// Direct getter functions with efficient casting
#[inline]
pub fn get_uint8(account: &AccountID, key: &str) -> Option<u8> {
    let mut buffer = [0u8; 1];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 1 {
        Some(buffer[0])
    } else {
        None
    }
}

#[inline]
pub fn get_uint16(account: &AccountID, key: &str) -> Option<u16> {
    let mut buffer = [0u8; 2];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };
    
    if output_len == 2 {
        Some(u16::from_be_bytes([buffer[0], buffer[1]]))
    } else {
        None
    }
}

#[inline]
pub fn get_uint32(account: &AccountID, key: &str) -> Option<u32> {
    let mut buffer = [0u8; 4];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 4 {
        Some(u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]))
    } else {
        None
    }
}

#[inline]
pub fn get_uint64(account: &AccountID, key: &str) -> Option<u64> {
    let mut buffer = [0u8; 8];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 8 {
        Some(u64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    } else {
        None
    }
}

#[inline]
pub fn get_uint128(account: &AccountID, key: &str) -> Option<[u8; 16]> {
    let mut buffer = [0u8; 16];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 16 {
        Some(buffer)
    } else {
        None
    }
}

#[inline]
pub fn get_uint160(account: &AccountID, key: &str) -> Option<[u8; 20]> {
    let mut buffer = [0u8; 20];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 20 {
        Some(buffer)
    } else {
        None
    }
}

#[inline]
pub fn get_uint256(account: &AccountID, key: &str) -> Option<[u8; 32]> {
    let mut buffer = [0u8; 32];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 32 {
        Some(buffer)
    } else {
        None
    }
}

#[inline]
pub fn get_amount(account: &AccountID, key: &str) -> Option<[u8; 8]> {
    let mut buffer = [0u8; 8];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };
    if output_len == 8 {
        Some(buffer)
    } else {
        None
    }
}

#[inline]
pub fn get_account(account: &AccountID, key: &str) -> Option<[u8; 20]> {
    let mut buffer = [0u8; 21];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };
    if output_len == 21 {
        Some(buffer[1..21].try_into().unwrap())
    } else {
        None
    }
}

#[inline]
pub fn get_currency(account: &AccountID, key: &str) -> Option<[u8; 20]> {
    let mut buffer = [0u8; 20];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 20 {
        Some(buffer)
    } else {
        None
    }
}

// Variable length blob getter
#[inline]
pub fn get_blob(account: &AccountID, key: &str, output: &mut [u8]) -> Option<usize> {
    let mut buffer = [0u8; 1024];
    let output_len = unsafe {
        get_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };
    
    if output_len <= 2 { return None; }
    
    let len = output_len as usize;
    let mut pos = 1; // Skip type byte
    
    // Parse VL length efficiently
    let data_len = if buffer[pos] <= 192 {
        let blob_len = buffer[pos] as usize;
        pos += 1;
        blob_len
    } else if buffer[pos] <= 240 && pos + 1 < len {
        let blob_len = 193 + ((buffer[pos] - 193) as usize) * 256 + buffer[pos + 1] as usize;
        pos += 2;
        blob_len
    } else if buffer[pos] <= 254 && pos + 2 < len {
        let blob_len = 12481 + ((buffer[pos] - 241) as usize) * 65536 + 
                      (buffer[pos + 1] as usize) * 256 + 
                      buffer[pos + 2] as usize;
        pos += 3;
        blob_len
    } else {
        return None;
    };
    
    if pos + data_len > len || data_len > output.len() {
        return None;
    }
    
    output[..data_len].copy_from_slice(&buffer[pos..pos + data_len]);
    Some(data_len)
}

#[inline]
pub fn get_string(account: &AccountID, key: &str, output: &mut [u8]) -> Option<usize> {
    get_blob(account, key, output)
}

// Nested getter functions
#[inline]
pub fn get_nested_uint8(account: &AccountID, nested: &str, key: &str) -> Option<u8> {
    let mut buffer = [0u8; 1];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 1 {
        Some(buffer[0])
    } else {
        None
    }
}

#[inline]
pub fn get_nested_uint16(account: &AccountID, nested: &str, key: &str) -> Option<u16> {
    let mut buffer = [0u8; 2];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };
    
    if output_len == 2 {
        Some(u16::from_be_bytes([buffer[0], buffer[1]]))
    } else {
        None
    }
}

#[inline]
pub fn get_nested_uint32(account: &AccountID, nested: &str, key: &str) -> Option<u32> {
    let mut buffer = [0u8; 4];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 4 {
        Some(u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]))
    } else {
        None
    }
}

#[inline]
pub fn get_nested_uint64(account: &AccountID, nested: &str, key: &str) -> Option<u64> {
    let mut buffer = [0u8; 8];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 8 {
        Some(u64::from_be_bytes([
            buffer[0], buffer[1], buffer[2], buffer[3],
            buffer[4], buffer[5], buffer[6], buffer[7]
        ]))
    } else {
        None
    }
}

#[inline]
pub fn get_nested_uint128(account: &AccountID, nested: &str, key: &str) -> Option<[u8; 16]> {
    let mut buffer = [0u8; 16];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 16 {
        Some(buffer)
    } else {
        None
    }
}

#[inline]
pub fn get_nested_uint160(account: &AccountID, nested: &str, key: &str) -> Option<[u8; 20]> {
    let mut buffer = [0u8; 20];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 20 {
        Some(buffer)
    } else {
        None
    }
}

#[inline]
pub fn get_nested_uint256(account: &AccountID, nested: &str, key: &str) -> Option<[u8; 32]> {
    let mut buffer = [0u8; 32];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 32 {
        Some(buffer)
    } else {
        None
    }
}

#[inline]
pub fn get_nested_amount(account: &AccountID, nested: &str, key: &str) -> Option<[u8; 8]> {
    let mut buffer = [0u8; 8];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };
    if output_len == 8 {
        Some(buffer)
    } else {
        None
    }
}

#[inline]
pub fn get_nested_account(account: &AccountID, nested: &str, key: &str) -> Option<[u8; 20]> {
    let mut buffer = [0u8; 21];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 21 {
        Some(buffer[1..20].try_into().unwrap())
    } else {
        None
    }
}

#[inline]
pub fn get_nested_currency(account: &AccountID, nested: &str, key: &str) -> Option<[u8; 20]> {
    let mut buffer = [0u8; 20];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };

    if output_len == 20 {
        Some(buffer)
    } else {
        None
    }
}

#[inline]
pub fn get_nested_blob(account: &AccountID, nested: &str, key: &str, output: &mut [u8]) -> Option<usize> {
    let mut buffer = [0u8; 1024];
    let output_len = unsafe {
        get_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_mut_ptr(),
            buffer.len()
        )
    };
    
    if output_len <= 2 { return None; }
    
    let len = output_len as usize;
    let mut pos = 1; // Skip type byte
    
    // Parse VL length efficiently
    let data_len = if buffer[pos] <= 192 {
        let blob_len = buffer[pos] as usize;
        pos += 1;
        blob_len
    } else if buffer[pos] <= 240 && pos + 1 < len {
        let blob_len = 193 + ((buffer[pos] - 193) as usize) * 256 + buffer[pos + 1] as usize;
        pos += 2;
        blob_len
    } else if buffer[pos] <= 254 && pos + 2 < len {
        let blob_len = 12481 + ((buffer[pos] - 241) as usize) * 65536 + 
                      (buffer[pos + 1] as usize) * 256 + 
                      buffer[pos + 2] as usize;
        pos += 3;
        blob_len
    } else {
        return None;
    };
    
    if pos + data_len > len || data_len > output.len() {
        return None;
    }
    
    output[..data_len].copy_from_slice(&buffer[pos..pos + data_len]);
    Some(data_len)
}

#[inline]
pub fn get_nested_string(account: &AccountID, nested: &str, key: &str, output: &mut [u8]) -> Option<usize> {
    get_nested_blob(account, nested, key, output)
}

// Direct setter functions with efficient serialization
#[inline]
pub fn set_uint8(account: &AccountID, key: &str, value: u8) -> Result<(), i32> {
    let buffer = [STI_UINT8.into(), value];

    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            2
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_uint8(account: &AccountID, nested: &str, key: &str, value: u8) -> Result<(), i32> {
    let buffer = [STI_UINT8.into(), value];

    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            2
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_uint16(account: &AccountID, nested: &str, key: &str, value: u16) -> Result<(), i32> {
    let bytes = value.to_be_bytes();
    let buffer = [STI_UINT16.into(), bytes[0], bytes[1]];
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            3
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_uint32(account: &AccountID, nested: &str, key: &str, value: u32) -> Result<(), i32> {
    let bytes = value.to_be_bytes();
    let buffer = [STI_UINT32.into(), bytes[0], bytes[1], bytes[2], bytes[3]];
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            5
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_uint64(account: &AccountID, nested: &str, key: &str, value: u64) -> Result<(), i32> {
    let bytes = value.to_be_bytes();
    let mut buffer = [0u8; 9];
    buffer[0] = STI_UINT64.into();
    buffer[1..9].copy_from_slice(&bytes);
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            9
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_uint128(account: &AccountID, nested: &str, key: &str, value: [u8; 16]) -> Result<(), i32> {
    let mut buffer = [0u8; 17];
    buffer[0] = STI_UINT128.into();
    buffer[1..17].copy_from_slice(&value);
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            17
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_uint160(account: &AccountID, nested: &str, key: &str, value: [u8; 20]) -> Result<(), i32> {
    let mut buffer = [0u8; 21];
    buffer[0] = STI_UINT160.into();
    buffer[1..21].copy_from_slice(&value);
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            21
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_uint256(account: &AccountID, nested: &str, key: &str, value: [u8; 32]) -> Result<(), i32> {
    let mut buffer = [0u8; 33];
    buffer[0] = STI_UINT256.into();
    buffer[1..33].copy_from_slice(&value);
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            33
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_amount(account: &AccountID, nested: &str, key: &str, value: [u8; 8]) -> Result<(), i32> {
    let mut buffer = [0u8; 9];
    buffer[0] = STI_AMOUNT.into();
    buffer[1..9].copy_from_slice(&value);
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            9
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_account(account: &AccountID, nested: &str, key: &str, value: [u8; 20]) -> Result<(), i32> {
    let mut buffer = [0u8; 22];
    buffer[0] = STI_ACCOUNT.into();
    buffer[1] = 0x14; // Account length prefix
    buffer[2..22].copy_from_slice(&value);
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            22
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_currency(account: &AccountID, nested: &str, key: &str, value: [u8; 20]) -> Result<(), i32> {
    let mut buffer = [0u8; 21];
    buffer[0] = STI_CURRENCY.into();
    buffer[1..21].copy_from_slice(&value);
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            21
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_blob(account: &AccountID, nested: &str, key: &str, data: &[u8]) -> Result<(), i32> {
    if data.len() > 512 {
        return Err(-1); // DATA_FIELD_TOO_LARGE
    }
    
    let mut buffer = [0u8; 1024];
    buffer[0] = STI_VL.into();
    
    let len_bytes = encode_vl_length(&mut buffer, 1, data.len());
    if len_bytes == 0 {
        return Err(-1);
    }
    
    let data_start = 1 + len_bytes;
    buffer[data_start..data_start + data.len()].copy_from_slice(data);
    
    unsafe {
        set_nested_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            nested.as_ptr(),
            nested.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            data_start + data.len()
        );
    }
    Ok(())
}

#[inline]
pub fn set_nested_string(account: &AccountID, nested: &str, key: &str, value: &str) -> Result<(), i32> {
    set_nested_blob(account, nested, key, value.as_bytes())
}

#[inline]
pub fn set_uint16(account: &AccountID, key: &str, value: u16) -> Result<(), i32> {
    let bytes = value.to_be_bytes();
    let buffer = [STI_UINT16.into(), bytes[0], bytes[1]];
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            3
        );
    }
    Ok(())
}

#[inline]
pub fn set_uint32(account: &AccountID, key: &str, value: u32) -> Result<(), i32> {
    let bytes = value.to_be_bytes();
    let buffer = [STI_UINT32.into(), bytes[0], bytes[1], bytes[2], bytes[3]];
    
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            5
        );
    }
    Ok(())
}

#[inline]
pub fn set_uint64(account: &AccountID, key: &str, value: u64) -> Result<(), i32> {
    let bytes = value.to_be_bytes();
    let mut buffer = [0u8; 9];
    buffer[0] = STI_UINT64.into();
    buffer[1..9].copy_from_slice(&bytes);
    
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            9
        );
    }
    Ok(())
}

#[inline]
pub fn set_uint128(account: &AccountID, key: &str, value: [u8; 16]) -> Result<(), i32> {
    let mut buffer = [0u8; 17];
    buffer[0] = STI_UINT128.into();
    buffer[1..17].copy_from_slice(&value);
    
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            17
        );
    }
    Ok(())
}

#[inline]
pub fn set_uint160(account: &AccountID, key: &str, value: [u8; 20]) -> Result<(), i32> {
    let mut buffer = [0u8; 21];
    buffer[0] = STI_UINT160.into();
    buffer[1..21].copy_from_slice(&value);
    
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            21
        );
    }
    Ok(())
}

#[inline]
pub fn set_uint256(account: &AccountID, key: &str, value: [u8; 32]) -> Result<(), i32> {
    let mut buffer = [0u8; 33];
    buffer[0] = STI_UINT256.into();
    buffer[1..33].copy_from_slice(&value);
    
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            33
        );
    }
    Ok(())
}

#[inline]
pub fn set_amount(account: &AccountID, key: &str, value: [u8; 8]) -> Result<(), i32> {
    let mut buffer = [0u8; 9];
    buffer[0] = STI_AMOUNT.into();
    buffer[1..9].copy_from_slice(&value);
    
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            9
        );
    }
    Ok(())
}

#[inline]
pub fn set_account(account: &AccountID, key: &str, value: [u8; 20]) -> Result<(), i32> {
    let mut buffer = [0u8; 22];
    buffer[0] = STI_ACCOUNT.into();
    buffer[1] = 0x14; // Account length prefix
    buffer[2..22].copy_from_slice(&value);
    
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            22
        );
    }
    Ok(())
}

#[inline]
pub fn set_currency(account: &AccountID, key: &str, value: [u8; 20]) -> Result<(), i32> {
    let mut buffer = [0u8; 21];
    buffer[0] = STI_CURRENCY.into();
    buffer[1..21].copy_from_slice(&value);
    
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            21
        );
    }
    Ok(())
}

// VL encoding helper
#[inline]
fn encode_vl_length(buffer: &mut [u8], pos: usize, len: usize) -> usize {
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

#[inline]
pub fn set_blob(account: &AccountID, key: &str, data: &[u8]) -> Result<(), i32> {
    if data.len() > 512 {
        return Err(-1); // DATA_FIELD_TOO_LARGE
    }
    
    let mut buffer = [0u8; 1024];
    buffer[0] = STI_VL.into();
    
    let len_bytes = encode_vl_length(&mut buffer, 1, data.len());
    if len_bytes == 0 {
        return Err(-1);
    }
    
    let data_start = 1 + len_bytes;
    buffer[data_start..data_start + data.len()].copy_from_slice(data);
    
    unsafe {
        set_contract_data_from_key(
            account.0.as_ptr(),
            account.0.len(),
            key.as_ptr(),
            key.len(),
            buffer.as_ptr(),
            data_start + data.len()
        );
    }
    Ok(())
}

#[inline]
pub fn set_string(account: &AccountID, key: &str, value: &str) -> Result<(), i32> {
    set_blob(account, key, value.as_bytes())
}