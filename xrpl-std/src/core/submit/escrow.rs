// // ============================================================================
// // ESCROW TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::types::blob::Blob;
// use crate::core::types::credential::CredentialID;
// use crate::core::types::contract::ContractAccount;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix, serialize_uint32_field, serialize_uint64_field,
//     serialize_blob_field, serialize_array_field
//     // Field codes
//     FIELD_DESTINATION, FIELD_CONDITION, FIELD_FULFILLMENT,
//     FIELD_OWNER, FIELD_OFFER_SEQUENCE, FIELD_CANCEL_AFTER, FIELD_FINISH_AFTER,
//     FIELD_DESTINATION_TAG, FIELD_CREDENTIAL_IDS, FIELD_DATA, 
//     FIELD_FINISH_FUNCTION, FIELD_COMPUTATION_ALLOWANCE
// };

// pub struct EscrowCreateFields {
//     pub destination: AccountID,
//     pub amount: TokenAmount,
//     pub destination_tag: Option<u32>,
//     pub condition: Option<Blob>,
//     pub cancel_after: Option<u64>,
//     pub finish_after: Option<u64>,
//     pub finish_function: Option<Blob>,
//     pub data: Option<Blob>,
// }

// pub struct EscrowCreateTxn {
//     pub common: CommonFields,
//     pub escrow: EscrowCreateFields,
// }

// impl EscrowCreateTxn {
//     pub fn new(common: CommonFields, escrow: EscrowCreateFields) -> Self {
//         Self { common, escrow }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(tag) = self.escrow.destination_tag {
//             serialize_uint32_field(FIELD_DESTINATION_TAG, tag, buf)?;
//         }
        
//         if let Some(ref condition) = self.escrow.condition {
//             serialize_blob_field(FIELD_CONDITION, condition, buf)?;
//         }
        
//         if let Some(cancel_after) = self.escrow.cancel_after {
//             serialize_uint64_field(FIELD_CANCEL_AFTER, cancel_after, buf)?;
//         }
        
//         if let Some(finish_after) = self.escrow.finish_after {
//             serialize_uint64_field(FIELD_FINISH_AFTER, finish_after, buf)?;
//         }
        
//         serialize_amount(&self.escrow.amount, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         if let Some(ref data) = self.escrow.data {
//             serialize_blob_field(FIELD_DATA, data, buf)?;
//         }
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
//         serialize_length_prefix(self.escrow.destination.0.len(), buf)?;
//         buf.extend_from_slice(&self.escrow.destination.0)?;
        
//         if let Some(ref finish_fn) = self.escrow.finish_function {
//             serialize_blob_field(FIELD_FINISH_FUNCTION, finish_fn, buf)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct EscrowFinishFields {
//     pub owner: AccountID,
//     pub offer_sequence: u32,
//     pub fulfillment: Option<Blob>,
//     pub condition: Option<Blob>,
//     pub credential_ids: Option<Vec<CredentialID>>,
//     pub computation_allowance: Option<u64>,
// }

// pub struct EscrowFinishTxn {
//     pub common: CommonFields,
//     pub escrow: EscrowFinishFields,
// }

// impl EscrowFinishTxn {
//     pub fn new(common: CommonFields, escrow: EscrowFinishFields) -> Self {
//         Self { common, escrow }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         serialize_uint32_field(FIELD_OFFER_SEQUENCE, self.escrow.offer_sequence, buf)?;
        
//         if let Some(ref condition) = self.escrow.condition {
//             serialize_blob_field(FIELD_CONDITION, condition, buf)?;
//         }
        
//         if let Some(ref fulfillment) = self.escrow.fulfillment {
//             serialize_blob_field(FIELD_FULFILLMENT, fulfillment, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(computation) = self.escrow.computation_allowance {
//             serialize_uint64_field(FIELD_COMPUTATION_ALLOWANCE, computation, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_OWNER, buf)?;
//         serialize_length_prefix(self.escrow.owner.0.len(), buf)?;
//         buf.extend_from_slice(&self.escrow.owner.0)?;
        
//         if let Some(ref creds) = self.escrow.credential_ids {
//             serialize_array_field(FIELD_CREDENTIAL_IDS, creds, buf)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct EscrowCancelFields {
//     pub owner: AccountID,
//     pub offer_sequence: u32,
// }

// pub struct EscrowCancelTxn {
//     pub common: CommonFields,
//     pub escrow: EscrowCancelFields,
// }

// impl EscrowCancelTxn {
//     pub fn new(common: CommonFields, escrow: EscrowCancelFields) -> Self {
//         Self { common, escrow }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint32_field(FIELD_OFFER_SEQUENCE, self.escrow.offer_sequence, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_OWNER, buf)?;
//         serialize_length_prefix(self.escrow.owner.0.len(), buf)?;
//         buf.extend_from_slice(&self.escrow.owner.0)?;
        
//         Ok(())
//     }
// }