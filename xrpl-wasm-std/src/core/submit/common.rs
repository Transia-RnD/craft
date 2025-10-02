use crate::core::types::account_id::AccountID;
use crate::core::types::public_key::PublicKey;
use crate::core::types::amount::token_amount::TokenAmount;
use crate::core::type_codes::{
    STI_UINT16, STI_UINT32, STI_AMOUNT, STI_VL, STI_ACCOUNT,
};
use crate::sfield;
use core::{mem::MaybeUninit, ptr, slice};

/// Custom buffer type for no_std environments
pub struct SerializationBuffer {
    data: MaybeUninit<[u8; 1024]>,
    len: usize,
}

impl SerializationBuffer {
    pub fn new() -> Self {
        Self {
            data: MaybeUninit::uninit(),
            len: 0,
        }
    }

    #[inline]
    fn capacity(&self) -> usize {
        1024
    }

    pub fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.len >= self.capacity() {
            return Err("Buffer overflow");
        }
        unsafe {
            let base = self.data.as_mut_ptr() as *mut u8;
            ptr::write(base.add(self.len), byte);
        }
        self.len += 1;
        Ok(())
    }

    pub fn extend_from_slice(&mut self, slice_in: &[u8]) -> Result<(), &'static str> {
        if self.len + slice_in.len() > self.capacity() {
            return Err("Buffer overflow");
        }
        unsafe {
            let mut dst = (self.data.as_mut_ptr() as *mut u8).add(self.len);
            for &b in slice_in {
                ptr::write(dst, b);
                dst = dst.add(1);
            }
        }
        self.len += slice_in.len();
        Ok(())
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self.data.as_ptr() as *const u8, self.len)
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

/// Common fields shared by all transaction types
pub struct CommonFields {
    pub transaction_type: u16,
    pub sequence: u32,
    pub account: AccountID,
    pub fee: TokenAmount,
    pub signing_pub_key: PublicKey,
    pub flags: u32,
}

/// Generate XRPL Field ID from sfield value
pub fn encode_field_id(sfield_value: i32, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
    // Extract type code and field code from the sfield value
    // sfield values are encoded as: (type_code << 16) | field_code
    let type_code = ((sfield_value >> 16) & 0xFF) as u8;
    let field_code = (sfield_value & 0xFF) as u8;
    
    if type_code < 16 && field_code < 16 {
        // 1 byte: high 4 bits = type, low 4 bits = field
        buf.push((type_code << 4) | field_code)?;
    } else if type_code < 16 {
        // 2 bytes: first byte has type in high 4 bits, 0 in low 4 bits
        //          second byte has field code
        buf.push(type_code << 4)?;
        buf.push(field_code)?;
    } else if field_code < 16 {
        // 2 bytes: first byte has 0 in high 4 bits, field in low 4 bits
        //          second byte has type code
        buf.push(field_code)?;
        buf.push(type_code)?;
    } else {
        // 3 bytes: first byte is 0, second is type, third is field
        buf.push(0)?;
        buf.push(type_code)?;
        buf.push(field_code)?;
    }
    Ok(())
}

/// Serialize length prefix according to XRPL rules
pub fn serialize_length_prefix(length: usize, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
    if length <= 192 {
        buf.push(length as u8)?;
    } else if length <= 12480 {
        let encoded = length - 193;
        buf.push(193 + (encoded >> 8) as u8)?;
        buf.push((encoded & 0xff) as u8)?;
    } else if length <= 918744 {
        let encoded = length - 12481;
        buf.push(241 + ((encoded >> 16) & 0x0f) as u8)?;
        buf.push((encoded >> 8) as u8)?;
        buf.push((encoded & 0xff) as u8)?;
    } else {
        return Err("Length too large");
    }
    Ok(())
}

/// Serialize an amount (common to all transaction types that use amounts)
pub fn serialize_amount(amount: &TokenAmount, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
    match amount {
        TokenAmount::XRP { num_drops } => {
            // XRP amounts are serialized as 8 bytes
            // For positive amounts, set the positive bit (0x40) and clear the currency bit (0x80)
            // For negative amounts, clear the positive bit
            if *num_drops >= 0 {
                let positive_drops = (*num_drops as u64) | 0x4000000000000000u64; // Set positive bit
                buf.extend_from_slice(&positive_drops.to_be_bytes())?;
            } else {
                let negative_drops = ((-*num_drops) as u64) & 0x3FFFFFFFFFFFFFFFu64; // Clear positive bit
                buf.extend_from_slice(&negative_drops.to_be_bytes())?;
            }
        },
        TokenAmount::IOU { amount, currency_code, issuer } => {
            // IOU amounts are serialized as 48 bytes: 8 bytes amount + 20 bytes currency + 20 bytes issuer
            buf.extend_from_slice(&amount.0)?; // 8 bytes - includes type/sign/exponent/mantissa
            buf.extend_from_slice(&currency_code.0)?; // 20 bytes
            buf.extend_from_slice(&issuer.0)?; // 20 bytes
        },
        TokenAmount::MPT { num_units, is_positive, mpt_id } => {
            // MPT amounts are serialized as 33 bytes: 1 byte flags + 8 bytes amount + 24 bytes mpt_id
            let flags = if *is_positive { 0x60u8 } else { 0x20u8 }; // MPT bit + sign bit
            buf.push(flags)?;
            buf.extend_from_slice(&num_units.to_be_bytes())?; // 8 bytes
            
            // Serialize MptId (24 bytes total: 4 bytes sequence + 20 bytes issuer)
            buf.extend_from_slice(&mpt_id.get_sequence_num().to_be_bytes())?; // 4 bytes
            buf.extend_from_slice(&mpt_id.get_issuer().0)?; // 20 bytes
        }
    }
    Ok(())
}

/// Serialize common fields in canonical order
pub fn serialize_common_fields(common: &CommonFields, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
    encode_field_id(sfield::TransactionType, buf)?;
    buf.extend_from_slice(&common.transaction_type.to_be_bytes())?;

    encode_field_id(sfield::Flags, buf)?;
    buf.extend_from_slice(&common.flags.to_be_bytes())?;

    encode_field_id(sfield::Sequence, buf)?;
    buf.extend_from_slice(&common.sequence.to_be_bytes())?;

    encode_field_id(sfield::SigningPubKey, buf)?;
    buf.push(0x00)?;

    Ok(())
}

/// Serialize the fee field (common to all transactions)
pub fn serialize_fee_field(fee: &TokenAmount, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
    encode_field_id(sfield::Fee, buf)?;
    serialize_amount(fee, buf)?;
    Ok(())
}

/// Serialize the account field (common to all transactions)
pub fn serialize_account_field(account: &AccountID, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
    encode_field_id(sfield::Account, buf)?;
    serialize_length_prefix(account.0.len(), buf)?;
    buf.extend_from_slice(&account.0)?;
    Ok(())
}