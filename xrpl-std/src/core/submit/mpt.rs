// // ============================================================================
// // MPTOKEN TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::hash::Hash256;
// use crate::core::types::blob::Blob;
// use crate::core::types::mpt::MPTokenIssuanceID;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_uint8_field, serialize_uint16_field, serialize_uint64_field,
//     serialize_blob_field, serialize_hash256_field,
//     serialize_mptokenissuanceid_field,
//     // Field codes
//     FIELD_ASSET_SCALE, FIELD_TRANSFER_FEE, FIELD_MAXIMUM_AMOUNT, FIELD_MPTOKEN_METADATA,
//     FIELD_MPTOKEN_ISSUANCE_ID, FIELD_DOMAIN_ID, FIELD_HOLDER
// };
// use crate::core::type_codes::{
//     STI_UINT8, STI_UINT16, STI_UINT64,
//     STI_HASH256, STI_BLOB,
//     STI_ACCOUNT, STI_MPTOKENISSUANCEID
// };

// pub struct MPTokenIssuanceCreateFields {
//     pub asset_scale: Option<u8>,
//     pub transfer_fee: Option<u16>,
//     pub maximum_amount: Option<u64>,
//     pub mptoken_metadata: Option<Blob>,
//     pub domain_id: Option<Hash256>,
// }

// pub struct MPTokenIssuanceCreateTxn {
//     pub common: CommonFields,
//     pub mptoken: MPTokenIssuanceCreateFields,
// }

// impl MPTokenIssuanceCreateTxn {
//     pub fn new(common: CommonFields, mptoken: MPTokenIssuanceCreateFields) -> Self {
//         Self { common, mptoken }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(asset_scale) = self.mptoken.asset_scale {
//             serialize_uint8_field(FIELD_ASSET_SCALE, asset_scale, buf)?;
//         }
        
//         if let Some(transfer_fee) = self.mptoken.transfer_fee {
//             serialize_uint16_field(FIELD_TRANSFER_FEE, transfer_fee, buf)?;
//         }
        
//         if let Some(max_amount) = self.mptoken.maximum_amount {
//             serialize_uint64_field(FIELD_MAXIMUM_AMOUNT, max_amount, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref metadata) = self.mptoken.mptoken_metadata {
//             serialize_blob_field(FIELD_MPTOKEN_METADATA, metadata, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         if let Some(ref domain_id) = self.mptoken.domain_id {
//             serialize_hash256_field(FIELD_DOMAIN_ID, domain_id, buf)?;
//         }
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct MPTokenIssuanceDestroyFields {
//     pub mptoken_issuance_id: MPTokenIssuanceID,
// }

// pub struct MPTokenIssuanceDestroyTxn {
//     pub common: CommonFields,
//     pub mptoken: MPTokenIssuanceDestroyFields,
// }

// impl MPTokenIssuanceDestroyTxn {
//     pub fn new(common: CommonFields, mptoken: MPTokenIssuanceDestroyFields) -> Self {
//         Self { common, mptoken }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_mptokenissuanceid_field(FIELD_MPTOKEN_ISSUANCE_ID, &self.mptoken.mptoken_issuance_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         Ok(())
//     }
// }

// pub struct MPTokenIssuanceSetFields {
//     pub mptoken_issuance_id: MPTokenIssuanceID,
//     pub holder: Option<AccountID>,
//     pub domain_id: Option<Hash256>,
// }

// pub struct MPTokenIssuanceSetTxn {
//     pub common: CommonFields,
//     pub mptoken: MPTokenIssuanceSetFields,
// }

// impl MPTokenIssuanceSetTxn {
//     pub fn new(common: CommonFields, mptoken: MPTokenIssuanceSetFields) -> Self {
//         Self { common, mptoken }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         if let Some(ref domain_id) = self.mptoken.domain_id {
//             serialize_hash256_field(FIELD_DOMAIN_ID, domain_id, buf)?;
//         }
        
//         serialize_mptokenissuanceid_field(FIELD_MPTOKEN_ISSUANCE_ID, &self.mptoken.mptoken_issuance_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref holder) = self.mptoken.holder {
//             encode_field_id(STI_ACCOUNT, FIELD_HOLDER, buf)?;
//             serialize_length_prefix(holder.0.len(), buf)?;
//             buf.extend_from_slice(&holder.0)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct MPTokenAuthorizeFields {
//     pub mptoken_issuance_id: MPTokenIssuanceID,
//     pub holder: Option<AccountID>,
// }

// pub struct MPTokenAuthorizeTxn {
//     pub common: CommonFields,
//     pub mptoken: MPTokenAuthorizeFields,
// }

// impl MPTokenAuthorizeTxn {
//     pub fn new(common: CommonFields, mptoken: MPTokenAuthorizeFields) -> Self {
//         Self { common, mptoken }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_mptokenissuanceid_field(FIELD_MPTOKEN_ISSUANCE_ID, &self.mptoken.mptoken_issuance_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref holder) = self.mptoken.holder {
//             encode_field_id(STI_ACCOUNT, FIELD_HOLDER, buf)?;
//             serialize_length_prefix(holder.0.len(), buf)?;
//             buf.extend_from_slice(&holder.0)?;
//         }
        
//         Ok(())
//     }
// }