// // ============================================================================
// // XCHAIN TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_uint32_field, serialize_uint64_field,
//     serialize_object_field,
//     FIELD_DESTINATION, FIELD_DESTINATION_TAG,
//     FIELD_XCHAIN_BRIDGE, FIELD_SIGNATURE_REWARD, FIELD_OTHER_CHAIN_SOURCE,
//     FIELD_XCHAIN_CLAIM_ID, FIELD_OTHER_CHAIN_DESTINATION,
//     FIELD_MIN_ACCOUNT_CREATE_AMOUNT,
// };
// use crate::core::type_codes::{
//     STI_AMOUNT, STI_ACCOUNT,
// };

// pub struct XChainCreateClaimIDFields {
//     pub xchain_bridge: XChainBridge,
//     pub signature_reward: TokenAmount,
//     pub other_chain_source: AccountID,
// }

// pub struct XChainCreateClaimIDTxn {
//     pub common: CommonFields,
//     pub xchain: XChainCreateClaimIDFields,
// }

// impl XChainCreateClaimIDTxn {
//     pub fn new(common: CommonFields, xchain: XChainCreateClaimIDFields) -> Self {
//         Self { common, xchain }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         encode_field_id(STI_AMOUNT, FIELD_SIGNATURE_REWARD, buf)?;
//         serialize_amount(&self.xchain.signature_reward, buf)?;
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_OTHER_CHAIN_SOURCE, buf)?;
//         serialize_length_prefix(self.xchain.other_chain_source.0.len(), buf)?;
//         buf.extend_from_slice(&self.xchain.other_chain_source.0)?;
        
//         serialize_object_field(FIELD_XCHAIN_BRIDGE, &self.xchain.xchain_bridge, buf)?;
        
//         Ok(())
//     }
// }

// pub struct XChainCommitFields {
//     pub xchain_bridge: XChainBridge,
//     pub xchain_claim_id: XChainClaimID,
//     pub amount: TokenAmount,
//     pub other_chain_destination: Option<AccountID>,
// }

// pub struct XChainCommitTxn {
//     pub common: CommonFields,
//     pub xchain: XChainCommitFields,
// }

// impl XChainCommitTxn {
//     pub fn new(common: CommonFields, xchain: XChainCommitFields) -> Self {
//         Self { common, xchain }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint64_field(FIELD_XCHAIN_CLAIM_ID, self.xchain.xchain_claim_id.0, buf)?;
//         serialize_amount(&self.xchain.amount, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref dest) = self.xchain.other_chain_destination {
//             encode_field_id(STI_ACCOUNT, FIELD_OTHER_CHAIN_DESTINATION, buf)?;
//             serialize_length_prefix(dest.0.len(), buf)?;
//             buf.extend_from_slice(&dest.0)?;
//         }
        
//         serialize_object_field(FIELD_XCHAIN_BRIDGE, &self.xchain.xchain_bridge, buf)?;
        
//         Ok(())
//     }
// }

// pub struct XChainClaimFields {
//     pub xchain_bridge: XChainBridge,
//     pub xchain_claim_id: XChainClaimID,
//     pub destination: AccountID,
//     pub destination_tag: Option<u32>,
//     pub amount: TokenAmount,
// }

// pub struct XChainClaimTxn {
//     pub common: CommonFields,
//     pub xchain: XChainClaimFields,
// }

// impl XChainClaimTxn {
//     pub fn new(common: CommonFields, xchain: XChainClaimFields) -> Self {
//         Self { common, xchain }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(tag) = self.xchain.destination_tag {
//             serialize_uint32_field(FIELD_DESTINATION_TAG, tag, buf)?;
//         }
        
//         serialize_uint64_field(FIELD_XCHAIN_CLAIM_ID, self.xchain.xchain_claim_id.0, buf)?;
//         serialize_amount(&self.xchain.amount, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
//         serialize_length_prefix(self.xchain.destination.0.len(), buf)?;
//         buf.extend_from_slice(&self.xchain.destination.0)?;
        
//         serialize_object_field(FIELD_XCHAIN_BRIDGE, &self.xchain.xchain_bridge, buf)?;
        
//         Ok(())
//     }
// }

// pub struct XChainAccountCreateCommitFields {
//     pub xchain_bridge: XChainBridge,
//     pub destination: AccountID,
//     pub amount: TokenAmount,
//     pub signature_reward: TokenAmount,
// }

// pub struct XChainAccountCreateCommitTxn {
//     pub common: CommonFields,
//     pub xchain: XChainAccountCreateCommitFields,
// }

// impl XChainAccountCreateCommitTxn {
//     pub fn new(common: CommonFields, xchain: XChainAccountCreateCommitFields) -> Self {
//         Self { common, xchain }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_amount(&self.xchain.amount, buf)?;
        
//         encode_field_id(STI_AMOUNT, FIELD_SIGNATURE_REWARD, buf)?;
//         serialize_amount(&self.xchain.signature_reward, buf)?;
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
//         serialize_length_prefix(self.xchain.destination.0.len(), buf)?;
//         buf.extend_from_slice(&self.xchain.destination.0)?;
        
//         serialize_object_field(FIELD_XCHAIN_BRIDGE, &self.xchain.xchain_bridge, buf)?;
        
//         Ok(())
//     }
// }

// pub struct XChainCreateBridgeFields {
//     pub xchain_bridge: XChainBridge,
//     pub signature_reward: TokenAmount,
//     pub min_account_create_amount: Option<TokenAmount>,
// }

// pub struct XChainCreateBridgeTxn {
//     pub common: CommonFields,
//     pub xchain: XChainCreateBridgeFields,
// }

// impl XChainCreateBridgeTxn {
//     pub fn new(common: CommonFields, xchain: XChainCreateBridgeFields) -> Self {
//         Self { common, xchain }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         encode_field_id(STI_AMOUNT, FIELD_SIGNATURE_REWARD, buf)?;
//         serialize_amount(&self.xchain.signature_reward, buf)?;
        
//         if let Some(ref min_amount) = self.xchain.min_account_create_amount {
//             encode_field_id(STI_AMOUNT, FIELD_MIN_ACCOUNT_CREATE_AMOUNT, buf)?;
//             serialize_amount(min_amount, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         serialize_object_field(FIELD_XCHAIN_BRIDGE, &self.xchain.xchain_bridge, buf)?;
        
//         Ok(())
//     }
// }

// pub struct XChainModifyBridgeFields {
//     pub xchain_bridge: XChainBridge,
//     pub signature_reward: Option<TokenAmount>,
//     pub min_account_create_amount: Option<TokenAmount>,
// }

// pub struct XChainModifyBridgeTxn {
//     pub common: CommonFields,
//     pub xchain: XChainModifyBridgeFields,
// }

// impl XChainModifyBridgeTxn {
//     pub fn new(common: CommonFields, xchain: XChainModifyBridgeFields) -> Self {
//         Self { common, xchain }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(ref sig_reward) = self.xchain.signature_reward {
//             encode_field_id(STI_AMOUNT, FIELD_SIGNATURE_REWARD, buf)?;
//             serialize_amount(sig_reward, buf)?;
//         }
        
//         if let Some(ref min_amount) = self.xchain.min_account_create_amount {
//             encode_field_id(STI_AMOUNT, FIELD_MIN_ACCOUNT_CREATE_AMOUNT, buf)?;
//             serialize_amount(min_amount, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         serialize_object_field(FIELD_XCHAIN_BRIDGE, &self.xchain.xchain_bridge, buf)?;
        
//         Ok(())
//     }
// }