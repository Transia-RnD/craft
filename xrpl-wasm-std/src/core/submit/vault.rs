// // ============================================================================
// // VAULT TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::types::hash::Hash256;
// use crate::core::types::blob::Blob;
// use crate::core::types::vault::VaultID;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_uint32_field, serialize_uint64_field,
//     serialize_blob_field, serialize_hash256_field,
//     serialize_vaultid_field,
//     // Field codes
//     FIELD_DESTINATION_TAG, FIELD_DOMAIN_ID, FIELD_ASSETS_MAXIMUM,
//     FIELD_WITHDRAWAL_POLICY, FIELD_VAULT_ID, FIELD_DATA,
//     FIELD_MPTOKEN_METADATA, FIELD_ASSET, FIELD_HOLDER, FIELD_DESTINATION,
// };
// use crate::core::type_codes::{
//     STI_ACCOUNT, STI_HASH256, STI_ISSUE,
// };

// pub struct VaultCreateFields {
//     pub asset: TokenAmount,
//     pub assets_maximum: Option<u64>,
//     pub mptoken_metadata: Option<Blob>,
//     pub domain_id: Option<Hash256>,
//     pub withdrawal_policy: Option<Blob>,
//     pub data: Option<Blob>,
// }

// pub struct VaultCreateTxn {
//     pub common: CommonFields,
//     pub vault: VaultCreateFields,
// }

// impl VaultCreateTxn {
//     pub fn new(common: CommonFields, vault: VaultCreateFields) -> Self {
//         Self { common, vault }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(max_assets) = self.vault.assets_maximum {
//             serialize_uint64_field(FIELD_ASSETS_MAXIMUM, max_assets, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref metadata) = self.vault.mptoken_metadata {
//             serialize_blob_field(FIELD_MPTOKEN_METADATA, metadata, buf)?;
//         }
        
//         if let Some(ref data) = self.vault.data {
//             serialize_blob_field(FIELD_DATA, data, buf)?;
//         }
        
//         if let Some(ref policy) = self.vault.withdrawal_policy {
//             serialize_blob_field(FIELD_WITHDRAWAL_POLICY, policy, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         if let Some(ref domain_id) = self.vault.domain_id {
//             serialize_hash256_field(FIELD_DOMAIN_ID, domain_id, buf)?;
//         }
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET, buf)?;
//         serialize_amount(&self.vault.asset, buf)?;
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct VaultSetFields {
//     pub vault_id: VaultID,
//     pub assets_maximum: Option<u64>,
//     pub domain_id: Option<Hash256>,
//     pub data: Option<Blob>,
// }

// pub struct VaultSetTxn {
//     pub common: CommonFields,
//     pub vault: VaultSetFields,
// }

// impl VaultSetTxn {
//     pub fn new(common: CommonFields, vault: VaultSetFields) -> Self {
//         Self { common, vault }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(max_assets) = self.vault.assets_maximum {
//             serialize_uint64_field(FIELD_ASSETS_MAXIMUM, max_assets, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref data) = self.vault.data {
//             serialize_blob_field(FIELD_DATA, data, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         if let Some(ref domain_id) = self.vault.domain_id {
//             serialize_hash256_field(FIELD_DOMAIN_ID, domain_id, buf)?;
//         }
        
//         serialize_vaultid_field(FIELD_VAULT_ID, &self.vault.vault_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct VaultDeleteFields {
//     pub vault_id: VaultID,
// }

// pub struct VaultDeleteTxn {
//     pub common: CommonFields,
//     pub vault: VaultDeleteFields,
// }

// impl VaultDeleteTxn {
//     pub fn new(common: CommonFields, vault: VaultDeleteFields) -> Self {
//         Self { common, vault }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_vaultid_field(FIELD_VAULT_ID, &self.vault.vault_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         Ok(())
//     }
// }

// pub struct VaultDepositFields {
//     pub vault_id: VaultID,
//     pub amount: TokenAmount,
// }

// pub struct VaultDepositTxn {
//     pub common: CommonFields,
//     pub vault: VaultDepositFields,
// }

// impl VaultDepositTxn {
//     pub fn new(common: CommonFields, vault: VaultDepositFields) -> Self {
//         Self { common, vault }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_amount(&self.vault.amount, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_vaultid_field(FIELD_VAULT_ID, &self.vault.vault_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         Ok(())
//     }
// }

// pub struct VaultWithdrawFields {
//     pub vault_id: VaultID,
//     pub amount: TokenAmount,
//     pub destination: Option<AccountID>,
//     pub destination_tag: Option<u32>,
// }

// pub struct VaultWithdrawTxn {
//     pub common: CommonFields,
//     pub vault: VaultWithdrawFields,
// }

// impl VaultWithdrawTxn {
//     pub fn new(common: CommonFields, vault: VaultWithdrawFields) -> Self {
//         Self { common, vault }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(tag) = self.vault.destination_tag {
//             serialize_uint32_field(FIELD_DESTINATION_TAG, tag, buf)?;
//         }
        
//         serialize_amount(&self.vault.amount, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_vaultid_field(FIELD_VAULT_ID, &self.vault.vault_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref destination) = self.vault.destination {
//             encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
//             serialize_length_prefix(destination.0.len(), buf)?;
//             buf.extend_from_slice(&destination.0)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct VaultClawbackFields {
//     pub vault_id: VaultID,
//     pub holder: AccountID,
//     pub amount: Option<TokenAmount>,
// }

// pub struct VaultClawbackTxn {
//     pub common: CommonFields,
//     pub vault: VaultClawbackFields,
// }

// impl VaultClawbackTxn {
//     pub fn new(common: CommonFields, vault: VaultClawbackFields) -> Self {
//         Self { common, vault }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(ref amount) = self.vault.amount {
//             serialize_amount(amount, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_vaultid_field(FIELD_VAULT_ID, &self.vault.vault_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_HOLDER, buf)?;
//         serialize_length_prefix(self.vault.holder.0.len(), buf)?;
//         buf.extend_from_slice(&self.vault.holder.0)?;
        
//         Ok(())
//     }
// }