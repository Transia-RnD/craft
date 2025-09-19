// ============================================================================
// PAYMENT TRANSACTIONS
// ============================================================================
use crate::core::types::account_id::AccountID;
use crate::core::types::amount::token_amount::TokenAmount;
use crate::core::submit::common::{
    CommonFields, SerializationBuffer, 
    serialize_common_fields, serialize_amount, serialize_fee_field, 
    serialize_signing_pub_key_field, serialize_account_field,
    encode_field_id, serialize_length_prefix,
    FIELD_AMOUNT, FIELD_DESTINATION
};
use crate::core::type_codes::{
    STI_ACCOUNT, STI_AMOUNT
};

/// Payment-specific transaction fields
pub struct PaymentFields {
    pub destination: AccountID,
    pub amount: TokenAmount,
}

/// Complete Payment transaction
pub struct PaymentTxn {
    pub common: CommonFields,
    pub payment: PaymentFields,
}

impl PaymentTxn {
    /// Create a new Payment transaction
    pub fn new(common: CommonFields, payment: PaymentFields) -> Self {
        Self { common, payment }
    }

    /// Serialize the Payment transaction according to XRPL canonical field ordering
    pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
        // Serialize fields in canonical order (by field ID: (type_code << 16) | field_code)
        
        // 1-3. Common fields: TransactionType, Flags, Sequence
        serialize_common_fields(&self.common, buf)?;

        // 4. DestinationTag (Type 2, Field 14) = 131086 - UInt32 (if present)
        // Skip for now - can be added later if needed

        // 5. Amount (Type 6, Field 1) = 393217 - Amount
        encode_field_id(STI_AMOUNT, FIELD_AMOUNT, buf)?;
        serialize_amount(&self.payment.amount, buf)?;

        // 6. Fee (Type 6, Field 8) = 393224 - Amount
        serialize_fee_field(&self.common.fee, buf)?;

        // 7. SigningPubKey (Type 7, Field 3) = 458755 - Blob
        serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;

        // 8. Account (Type 8, Field 1) = 524289 - AccountID
        serialize_account_field(&self.common.account, buf)?;

        // 9. Destination (Type 8, Field 3) = 524291 - AccountID
        encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
        serialize_length_prefix(self.payment.destination.0.len(), buf)?;
        buf.extend_from_slice(&self.payment.destination.0)?;

        Ok(())
    }
}