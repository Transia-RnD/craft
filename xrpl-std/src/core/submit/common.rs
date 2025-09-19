use crate::core::types::account_id::AccountID;
use crate::core::types::public_key::PublicKey;
use crate::core::types::amount::token_amount::TokenAmount;
use crate::core::type_codes::{
    STI_UINT16, STI_UINT32, STI_AMOUNT, STI_VL, STI_ACCOUNT,
};

// XRPL Field Codes (within each type)
pub const FIELD_TRANSACTION_TYPE: u8 = 2;  // UInt16 - TransactionType
pub const FIELD_FLAGS: u8 = 2;             // UInt32 - Flags  
pub const FIELD_SEQUENCE: u8 = 4;          // UInt32 - Sequence
pub const FIELD_DESTINATION_TAG: u8 = 14;  // UInt32 - DestinationTag
pub const FIELD_AMOUNT: u8 = 1;            // Amount - Amount
pub const FIELD_FEE: u8 = 8;               // Amount - Fee
pub const FIELD_SIGNING_PUB_KEY: u8 = 3;   // Blob - SigningPubKey
pub const FIELD_ACCOUNT: u8 = 1;           // AccountID - Account
pub const FIELD_DESTINATION: u8 = 3;       // AccountID - Destination

/// Custom buffer type for no_std environments
pub struct SerializationBuffer {
    data: [u8; 1024], // Fixed size buffer - adjust as needed
    len: usize,
}

impl SerializationBuffer {
    pub fn new() -> Self {
        Self {
            data: [0; 1024],
            len: 0,
        }
    }

    pub fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.len >= self.data.len() {
            return Err("Buffer overflow");
        }
        self.data[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    pub fn extend_from_slice(&mut self, slice: &[u8]) -> Result<(), &'static str> {
        if self.len + slice.len() > self.data.len() {
            return Err("Buffer overflow");
        }
        self.data[self.len..self.len + slice.len()].copy_from_slice(slice);
        self.len += slice.len();
        Ok(())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
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

/// Generate XRPL Field ID based on type code and field code
pub fn encode_field_id(type_code: u8, field_code: u8, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
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
    // 1. TransactionType (Type 1, Field 2) = 65538 - UInt16
    encode_field_id(STI_UINT16, FIELD_TRANSACTION_TYPE, buf)?;
    buf.extend_from_slice(&common.transaction_type.to_be_bytes())?;

    // 2. Flags (Type 2, Field 2) = 131074 - UInt32 (if present)
    encode_field_id(STI_UINT32, FIELD_FLAGS, buf)?;
    buf.extend_from_slice(&common.flags.to_be_bytes())?;

    // 3. Sequence (Type 2, Field 4) = 131076 - UInt32
    encode_field_id(STI_UINT32, FIELD_SEQUENCE, buf)?;
    buf.extend_from_slice(&common.sequence.to_be_bytes())?;

    Ok(())
}

/// Serialize the fee field (common to all transactions)
pub fn serialize_fee_field(fee: &TokenAmount, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
    // Fee (Type 6, Field 8) = 393224 - Amount
    encode_field_id(STI_AMOUNT, FIELD_FEE, buf)?;
    serialize_amount(fee, buf)?;
    Ok(())
}

/// Serialize the signing public key field (common to all transactions)
pub fn serialize_signing_pub_key_field(signing_pub_key: &PublicKey, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
    // SigningPubKey (Type 7, Field 3) = 458755 - Blob
    encode_field_id(STI_VL, FIELD_SIGNING_PUB_KEY, buf)?;
    if signing_pub_key.0[0] == 0x00 {
        // Empty key: just append 0x00 after the field ID
        buf.push(0x00)?;
    } else {
        // Normal key: serialize with length prefix
        serialize_length_prefix(signing_pub_key.0.len(), buf)?;
        buf.extend_from_slice(&signing_pub_key.0)?;
    }
    Ok(())
}

/// Serialize the account field (common to all transactions)
pub fn serialize_account_field(account: &AccountID, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
    // Account (Type 8, Field 1) = 524289 - AccountID
    encode_field_id(STI_ACCOUNT, FIELD_ACCOUNT, buf)?;
    serialize_length_prefix(account.0.len(), buf)?;
    buf.extend_from_slice(&account.0)?;
    Ok(())
}