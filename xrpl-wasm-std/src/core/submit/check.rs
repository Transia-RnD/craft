// // ============================================================================
// // CHECK TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::types::hash::Hash256;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_uint32_field, serialize_hash256_field,
//     // Field codes
//     FIELD_DESTINATION_TAG, FIELD_SEND_MAX, FIELD_DELIVER_MIN,
//     FIELD_INVOICE_ID, FIELD_EXPIRATION, FIELD_CHECK_ID, FIELD_DESTINATION,
// };
// use crate::core::type_codes::{
//     STI_AMOUNT, STI_ACCOUNT,
// };

// pub struct CheckCreateFields {
//     pub destination: AccountID,
//     pub send_max: TokenAmount,
//     pub expiration: Option<u32>,
//     pub destination_tag: Option<u32>,
//     pub invoice_id: Option<Hash256>,
// }

// pub struct CheckCreateTxn {
//     pub common: CommonFields,
//     pub check: CheckCreateFields,
// }

// impl CheckCreateTxn {
//     pub fn new(common: CommonFields, check: CheckCreateFields) -> Self {
//         Self { common, check }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(tag) = self.check.destination_tag {
//             serialize_uint32_field(FIELD_DESTINATION_TAG, tag, buf)?;
//         }
        
//         if let Some(expiration) = self.check.expiration {
//             serialize_uint32_field(FIELD_EXPIRATION, expiration, buf)?;
//         }
        
//         if let Some(ref invoice_id) = self.check.invoice_id {
//             serialize_hash256_field(FIELD_INVOICE_ID, invoice_id, buf)?;
//         }
        
//         encode_field_id(STI_AMOUNT, FIELD_SEND_MAX, buf)?;
//         serialize_amount(&self.check.send_max, buf)?;
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
//         serialize_length_prefix(self.check.destination.0.len(), buf)?;
//         buf.extend_from_slice(&self.check.destination.0)?;
        
//         Ok(())
//     }
// }

// pub struct CheckCashFields {
//     pub check_id: Hash256,
//     pub amount: Option<TokenAmount>,
//     pub deliver_min: Option<TokenAmount>,
// }

// pub struct CheckCashTxn {
//     pub common: CommonFields,
//     pub check: CheckCashFields,
// }

// impl CheckCashTxn {
//     pub fn new(common: CommonFields, check: CheckCashFields) -> Self {
//         Self { common, check }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_hash256_field(FIELD_CHECK_ID, &self.check.check_id, buf)?;
        
//         if let Some(ref amount) = self.check.amount {
//             serialize_amount(amount, buf)?;
//         }
        
//         if let Some(ref deliver_min) = self.check.deliver_min {
//             encode_field_id(STI_AMOUNT, FIELD_DELIVER_MIN, buf)?;
//             serialize_amount(deliver_min, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct CheckCancelFields {
//     pub check_id: Hash256,
// }

// pub struct CheckCancelTxn {
//     pub common: CommonFields,
//     pub check: CheckCancelFields,
// }

// impl CheckCancelTxn {
//     pub fn new(common: CommonFields, check: CheckCancelFields) -> Self {
//         Self { common, check }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_hash256_field(FIELD_CHECK_ID, &self.check.check_id, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         Ok(())
//     }
// }