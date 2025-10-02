// // ============================================================================
// // SIGNER LIST TRANSACTION
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::hash::Hash256;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id,
//     serialize_uint16_field, serialize_uint32_field,
//     serialize_hash256_field,
//     // Field codes
//     FIELD_SIGNER_QUORUM, FIELD_SIGNER_ENTRIES, FIELD_SIGNER_WEIGHT, FIELD_WALLET_LOCATOR,
// };
// use crate::core::type_codes::{
//     STI_ARRAY,
// };

// pub struct SignerEntry {
//     pub account: AccountID,
//     pub signer_weight: u16,
//     pub wallet_locator: Option<Hash256>,
// }

// pub struct SignerListSetFields {
//     pub signer_quorum: u32,
//     pub signer_entries: Option<Vec<SignerEntry>>,
// }

// pub struct SignerListSetTxn {
//     pub common: CommonFields,
//     pub signer_list: SignerListSetFields,
// }

// impl SignerListSetTxn {
//     pub fn new(common: CommonFields, signer_list: SignerListSetFields) -> Self {
//         Self { common, signer_list }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint32_field(FIELD_SIGNER_QUORUM, self.signer_list.signer_quorum, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref entries) = self.signer_list.signer_entries {
//             // Serialize signer entries array
//             encode_field_id(STI_ARRAY, FIELD_SIGNER_ENTRIES, buf)?;
//             for entry in entries {
//                 // Each entry is an STObject
//                 serialize_account_field(&entry.account, buf)?;
//                 serialize_uint16_field(FIELD_SIGNER_WEIGHT, entry.signer_weight, buf)?;
//                 if let Some(ref loc) = entry.wallet_locator {
//                     serialize_hash256_field(FIELD_WALLET_LOCATOR, loc, buf)?;
//                 }
//             }
//         }
        
//         Ok(())
//     }
// }