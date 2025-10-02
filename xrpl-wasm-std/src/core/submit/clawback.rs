// // ============================================================================
// // CLAWBACK TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     FIELD_HOLDER, FIELD_ASSET, FIELD_ASSET2,
// };
// use crate::core::type_codes::{
//     STI_ACCOUNT, STI_ISSUE,
// };

// pub struct ClawbackFields {
//     pub amount: TokenAmount,
//     pub holder: Option<AccountID>,
// }

// pub struct ClawbackTxn {
//     pub common: CommonFields,
//     pub clawback: ClawbackFields,
// }

// impl ClawbackTxn {
//     pub fn new(common: CommonFields, clawback: ClawbackFields) -> Self {
//         Self { common, clawback }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_amount(&self.clawback.amount, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref holder) = self.clawback.holder {
//             encode_field_id(STI_ACCOUNT, FIELD_HOLDER, buf)?;
//             serialize_length_prefix(holder.0.len(), buf)?;
//             buf.extend_from_slice(&holder.0)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct AMMClawbackFields {
//     pub holder: AccountID,
//     pub asset: TokenAmount,
//     pub asset2: TokenAmount,
//     pub amount: Option<TokenAmount>,
// }

// pub struct AMMClawbackTxn {
//     pub common: CommonFields,
//     pub clawback: AMMClawbackFields,
// }

// impl AMMClawbackTxn {
//     pub fn new(common: CommonFields, clawback: AMMClawbackFields) -> Self {
//         Self { common, clawback }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(ref amount) = self.clawback.amount {
//             serialize_amount(amount, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET, buf)?;
//         serialize_amount(&self.clawback.asset, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET2, buf)?;
//         serialize_amount(&self.clawback.asset2, buf)?;
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_HOLDER, buf)?;
//         serialize_length_prefix(self.clawback.holder.0.len(), buf)?;
//         buf.extend_from_slice(&self.clawback.holder.0)?;
        
//         Ok(())
//     }
// }