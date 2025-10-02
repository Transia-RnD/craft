// // ============================================================================
// // ACCOUNT TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::hash::{Hash256, Hash128};
// use crate::core::types::blob::Blob;
// use crate::core::types::credential::CredentialID;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_uint8_field, serialize_uint32_field,
//     serialize_blob_field, serialize_hash256_field, serialize_hash128_field,
//     serialize_array_field,
//     FIELD_EMAIL_HASH, FIELD_WALLET_LOCATOR, FIELD_WALLET_SIZE, FIELD_MESSAGE_KEY,
//     FIELD_DOMAIN, FIELD_TRANSFER_RATE, FIELD_SET_FLAG, FIELD_CLEAR_FLAG,
//     FIELD_TICK_SIZE, FIELD_NFTOKEN_MINTER, FIELD_REGULAR_KEY,
//     FIELD_DESTINATION, FIELD_DESTINATION_TAG, FIELD_CREDENTIAL_IDS,
// };
// use crate::core::type_codes::{
//     STI_UINT8, STI_UINT32,
//     STI_HASH128, STI_HASH256, STI_BLOB,
//     STI_ACCOUNT,
// };

// pub struct AccountSetFields {
//     pub email_hash: Option<Hash128>,
//     pub wallet_locator: Option<Hash256>,
//     pub wallet_size: Option<u32>,
//     pub message_key: Option<Blob>,
//     pub domain: Option<Blob>,
//     pub transfer_rate: Option<u32>,
//     pub set_flag: Option<u32>,
//     pub clear_flag: Option<u32>,
//     pub tick_size: Option<u8>,
//     pub nftoken_minter: Option<AccountID>,
// }

// pub struct AccountSetTxn {
//     pub common: CommonFields,
//     pub account_set: AccountSetFields,
// }

// impl AccountSetTxn {
//     pub fn new(common: CommonFields, account_set: AccountSetFields) -> Self {
//         Self { common, account_set }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(ref email) = self.account_set.email_hash {
//             serialize_hash128_field(FIELD_EMAIL_HASH, email, buf)?;
//         }
        
//         if let Some(ref msg_key) = self.account_set.message_key {
//             serialize_blob_field(FIELD_MESSAGE_KEY, msg_key, buf)?;
//         }
        
//         if let Some(ref domain) = self.account_set.domain {
//             serialize_blob_field(FIELD_DOMAIN, domain, buf)?;
//         }
        
//         if let Some(transfer_rate) = self.account_set.transfer_rate {
//             serialize_uint32_field(FIELD_TRANSFER_RATE, transfer_rate, buf)?;
//         }
        
//         if let Some(tick_size) = self.account_set.tick_size {
//             serialize_uint8_field(FIELD_TICK_SIZE, tick_size, buf)?;
//         }
        
//         if let Some(ref wallet_loc) = self.account_set.wallet_locator {
//             serialize_hash256_field(FIELD_WALLET_LOCATOR, wallet_loc, buf)?;
//         }
        
//         if let Some(wallet_size) = self.account_set.wallet_size {
//             serialize_uint32_field(FIELD_WALLET_SIZE, wallet_size, buf)?;
//         }
        
//         if let Some(set_flag) = self.account_set.set_flag {
//             serialize_uint32_field(FIELD_SET_FLAG, set_flag, buf)?;
//         }
        
//         if let Some(clear_flag) = self.account_set.clear_flag {
//             serialize_uint32_field(FIELD_CLEAR_FLAG, clear_flag, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref minter) = self.account_set.nftoken_minter {
//             encode_field_id(STI_ACCOUNT, FIELD_NFTOKEN_MINTER, buf)?;
//             serialize_length_prefix(minter.0.len(), buf)?;
//             buf.extend_from_slice(&minter.0)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct SetRegularKeyFields {
//     pub regular_key: Option<AccountID>,
// }

// pub struct SetRegularKeyTxn {
//     pub common: CommonFields,
//     pub regular_key: SetRegularKeyFields,
// }

// impl SetRegularKeyTxn {
//     pub fn new(common: CommonFields, regular_key: SetRegularKeyFields) -> Self {
//         Self { common, regular_key }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref key) = self.regular_key.regular_key {
//             encode_field_id(STI_ACCOUNT, FIELD_REGULAR_KEY, buf)?;
//             serialize_length_prefix(key.0.len(), buf)?;
//             buf.extend_from_slice(&key.0)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct AccountDeleteFields {
//     pub destination: AccountID,
//     pub destination_tag: Option<u32>,
//     pub credential_ids: Option<Vec<CredentialID>>,
// }

// pub struct AccountDeleteTxn {
//     pub common: CommonFields,
//     pub account_delete: AccountDeleteFields,
// }

// impl AccountDeleteTxn {
//     pub fn new(common: CommonFields, account_delete: AccountDeleteFields) -> Self {
//         Self { common, account_delete }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(tag) = self.account_delete.destination_tag {
//             serialize_uint32_field(FIELD_DESTINATION_TAG, tag, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
//         serialize_length_prefix(self.account_delete.destination.0.len(), buf)?;
//         buf.extend_from_slice(&self.account_delete.destination.0)?;
        
//         if let Some(ref creds) = self.account_delete.credential_ids {
//             serialize_array_field(FIELD_CREDENTIAL_IDS, creds, buf)?;
//         }
        
//         Ok(())
//     }
// }