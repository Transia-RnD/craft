// // ============================================================================
// // PAYMENT CHANNEL TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::types::hash::Hash256;
// use crate::core::types::blob::Blob;
// use crate::core::types::credential::CredentialID;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_uint32_field, serialize_uint64_field,
//     serialize_blob_field, serialize_hash256_field,
//     serialize_array_field,
//     // Field codes
//     FIELD_DESTINATION_TAG, FIELD_SETTLE_DELAY, FIELD_CANCEL_AFTER,
//     FIELD_PUBLIC_KEY, FIELD_CHANNEL, FIELD_BALANCE,
//     FIELD_SIGNATURE, FIELD_CREDENTIAL_IDS, FIELD_EXPIRATION,
//     FIELD_AMOUNT, FIELD_FEE, FIELD_SIGNING_PUB_KEY, FIELD_ACCOUNT, FIELD_DESTINATION,
// };
// use crate::core::type_codes::{
//     STI_AMOUNT, STI_ACCOUNT,
// };

// pub struct PaymentChannelCreateFields {
//     pub destination: AccountID,
//     pub amount: TokenAmount,
//     pub settle_delay: u32,
//     pub public_key: Blob,
//     pub cancel_after: Option<u64>,
//     pub destination_tag: Option<u32>,
// }

// pub struct PaymentChannelCreateTxn {
//     pub common: CommonFields,
//     pub paychan: PaymentChannelCreateFields,
// }

// impl PaymentChannelCreateTxn {
//     pub fn new(common: CommonFields, paychan: PaymentChannelCreateFields) -> Self {
//         Self { common, paychan }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(tag) = self.paychan.destination_tag {
//             serialize_uint32_field(FIELD_DESTINATION_TAG, tag, buf)?;
//         }
        
//         serialize_uint32_field(FIELD_SETTLE_DELAY, self.paychan.settle_delay, buf)?;
        
//         if let Some(cancel_after) = self.paychan.cancel_after {
//             serialize_uint64_field(FIELD_CANCEL_AFTER, cancel_after, buf)?;
//         }
        
//         serialize_amount(&self.paychan.amount, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_blob_field(FIELD_PUBLIC_KEY, &self.paychan.public_key, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
//         serialize_length_prefix(self.paychan.destination.0.len(), buf)?;
//         buf.extend_from_slice(&self.paychan.destination.0)?;
        
//         Ok(())
//     }
// }

// pub struct PaymentChannelFundFields {
//     pub channel: Hash256,
//     pub amount: TokenAmount,
//     pub expiration: Option<u32>,
// }

// pub struct PaymentChannelFundTxn {
//     pub common: CommonFields,
//     pub paychan: PaymentChannelFundFields,
// }

// impl PaymentChannelFundTxn {
//     pub fn new(common: CommonFields, paychan: PaymentChannelFundFields) -> Self {
//         Self { common, paychan }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(expiration) = self.paychan.expiration {
//             serialize_uint32_field(FIELD_EXPIRATION, expiration, buf)?;
//         }
        
//         serialize_hash256_field(FIELD_CHANNEL, &self.paychan.channel, buf)?;
//         serialize_amount(&self.paychan.amount, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct PaymentChannelClaimFields {
//     pub channel: Hash256,
//     pub amount: Option<TokenAmount>,
//     pub balance: Option<TokenAmount>,
//     pub signature: Option<Blob>,
//     pub public_key: Option<Blob>,
//     pub credential_ids: Option<Vec<CredentialID>>,
// }

// pub struct PaymentChannelClaimTxn {
//     pub common: CommonFields,
//     pub paychan: PaymentChannelClaimFields,
// }

// impl PaymentChannelClaimTxn {
//     pub fn new(common: CommonFields, paychan: PaymentChannelClaimFields) -> Self {
//         Self { common, paychan }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_hash256_field(FIELD_CHANNEL, &self.paychan.channel, buf)?;
        
//         if let Some(ref amount) = self.paychan.amount {
//             serialize_amount(amount, buf)?;
//         }
        
//         if let Some(ref balance) = self.paychan.balance {
//             encode_field_id(STI_AMOUNT, FIELD_BALANCE, buf)?;
//             serialize_amount(balance, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref signature) = self.paychan.signature {
//             serialize_blob_field(FIELD_SIGNATURE, signature, buf)?;
//         }
        
//         if let Some(ref public_key) = self.paychan.public_key {
//             serialize_blob_field(FIELD_PUBLIC_KEY, public_key, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref creds) = self.paychan.credential_ids {
//             serialize_array_field(FIELD_CREDENTIAL_IDS, creds, buf)?;
//         }
        
//         Ok(())
//     }
// }