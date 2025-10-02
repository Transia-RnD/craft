// ============================================================================
// PAYMENT TRANSACTIONS
// ============================================================================
use crate::core::types::account_id::AccountID;
use crate::core::types::amount::token_amount::TokenAmount;
use crate::core::submit::common::{
    CommonFields, SerializationBuffer, 
    serialize_common_fields, serialize_amount, serialize_fee_field, 
    serialize_account_field,
    encode_field_id, serialize_length_prefix
};
use crate::core::type_codes::{
    STI_ACCOUNT, STI_AMOUNT
};
use crate::sfield;

pub struct PaymentFields {
    pub destination: AccountID,
    pub amount: TokenAmount,
}

pub struct PaymentTxn {
    pub common: CommonFields,
    pub payment: PaymentFields,
}

impl PaymentTxn {
    pub fn new(common: CommonFields, payment: PaymentFields) -> Self {
        Self { common, payment }
    }

    pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
        
        serialize_common_fields(&self.common, buf)?;
        // 4. DestinationTag (Type 2, Field 14) = 131086 - UInt32 (if present)
        // Skip for now - can be added later if needed
        encode_field_id(sfield::Amount, buf)?;
        serialize_amount(&self.payment.amount, buf)?;
        serialize_fee_field(&self.common.fee, buf)?;
        serialize_account_field(&self.common.account, buf)?;
        encode_field_id(sfield::Destination, buf)?;
        serialize_length_prefix(self.payment.destination.0.len(), buf)?;
        buf.extend_from_slice(&self.payment.destination.0)?;

        Ok(())
    }
}