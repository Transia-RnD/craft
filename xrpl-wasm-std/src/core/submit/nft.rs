// // ============================================================================
// // NFT TRANSACTIONS
// // ============================================================================

// use crate::core::types::account_id::AccountID;
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::types::hash::Hash256;
// use crate::core::types::blob::Blob;
// use crate::core::types::nft::NFTokenID;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_uint16_field, serialize_uint32_field,
//     serialize_blob_field, serialize_hash256_field,
//     serialize_nftokenid_field,
//     // Field codes
//     FIELD_TRANSFER_FEE, FIELD_NFTOKEN_TAXON, FIELD_EXPIRATION,
//     FIELD_NFTOKEN_ID, FIELD_URI, FIELD_ISSUER,
//     FIELD_NFTOKEN_BUY_OFFER, FIELD_NFTOKEN_SELL_OFFER, FIELD_NFTOKEN_BROKER_FEE,
//     FIELD_NFTOKEN_OFFERS, FIELD_OWNER, FIELD_DESTINATION,
// };
// use crate::core::type_codes::{
//     STI_UINT16, STI_UINT32,
//     STI_HASH256, STI_AMOUNT, STI_BLOB,
//     STI_ACCOUNT, STI_ARRAY,
// };

// pub struct NFTokenMintFields {
//     pub nftoken_taxon: u32,
//     pub transfer_fee: Option<u16>,
//     pub issuer: Option<AccountID>,
//     pub uri: Option<Blob>,
//     pub amount: Option<TokenAmount>,
//     pub destination: Option<AccountID>,
//     pub expiration: Option<u32>,
// }

// pub struct NFTokenMintTxn {
//     pub common: CommonFields,
//     pub nft: NFTokenMintFields,
// }

// impl NFTokenMintTxn {
//     pub fn new(common: CommonFields, nft: NFTokenMintFields) -> Self {
//         Self { common, nft }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(transfer_fee) = self.nft.transfer_fee {
//             serialize_uint16_field(FIELD_TRANSFER_FEE, transfer_fee, buf)?;
//         }
        
//         serialize_uint32_field(FIELD_NFTOKEN_TAXON, self.nft.nftoken_taxon, buf)?;
        
//         if let Some(expiration) = self.nft.expiration {
//             serialize_uint32_field(FIELD_EXPIRATION, expiration, buf)?;
//         }
        
//         if let Some(ref amount) = self.nft.amount {
//             serialize_amount(amount, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref uri) = self.nft.uri {
//             serialize_blob_field(FIELD_URI, uri, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref destination) = self.nft.destination {
//             encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
//             serialize_length_prefix(destination.0.len(), buf)?;
//             buf.extend_from_slice(&destination.0)?;
//         }
        
//         if let Some(ref issuer) = self.nft.issuer {
//             encode_field_id(STI_ACCOUNT, FIELD_ISSUER, buf)?;
//             serialize_length_prefix(issuer.0.len(), buf)?;
//             buf.extend_from_slice(&issuer.0)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct NFTokenBurnFields {
//     pub nftoken_id: NFTokenID,
//     pub owner: Option<AccountID>,
// }

// pub struct NFTokenBurnTxn {
//     pub common: CommonFields,
//     pub nft: NFTokenBurnFields,
// }

// impl NFTokenBurnTxn {
//     pub fn new(common: CommonFields, nft: NFTokenBurnFields) -> Self {
//         Self { common, nft }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_nftokenid_field(FIELD_NFTOKEN_ID, &self.nft.nftoken_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref owner) = self.nft.owner {
//             encode_field_id(STI_ACCOUNT, FIELD_OWNER, buf)?;
//             serialize_length_prefix(owner.0.len(), buf)?;
//             buf.extend_from_slice(&owner.0)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct NFTokenCreateOfferFields {
//     pub nftoken_id: NFTokenID,
//     pub amount: TokenAmount,
//     pub destination: Option<AccountID>,
//     pub owner: Option<AccountID>,
//     pub expiration: Option<u32>,
// }

// pub struct NFTokenCreateOfferTxn {
//     pub common: CommonFields,
//     pub nft: NFTokenCreateOfferFields,
// }

// impl NFTokenCreateOfferTxn {
//     pub fn new(common: CommonFields, nft: NFTokenCreateOfferFields) -> Self {
//         Self { common, nft }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(expiration) = self.nft.expiration {
//             serialize_uint32_field(FIELD_EXPIRATION, expiration, buf)?;
//         }
        
//         serialize_amount(&self.nft.amount, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_nftokenid_field(FIELD_NFTOKEN_ID, &self.nft.nftoken_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref destination) = self.nft.destination {
//             encode_field_id(STI_ACCOUNT, FIELD_DESTINATION, buf)?;
//             serialize_length_prefix(destination.0.len(), buf)?;
//             buf.extend_from_slice(&destination.0)?;
//         }
        
//         if let Some(ref owner) = self.nft.owner {
//             encode_field_id(STI_ACCOUNT, FIELD_OWNER, buf)?;
//             serialize_length_prefix(owner.0.len(), buf)?;
//             buf.extend_from_slice(&owner.0)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct NFTokenCancelOfferFields {
//     pub nftoken_offers: Vec<Hash256>,
// }

// pub struct NFTokenCancelOfferTxn {
//     pub common: CommonFields,
//     pub nft: NFTokenCancelOfferFields,
// }

// impl NFTokenCancelOfferTxn {
//     pub fn new(common: CommonFields, nft: NFTokenCancelOfferFields) -> Self {
//         Self { common, nft }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         // Serialize NFTokenOffers array
//         encode_field_id(STI_ARRAY, FIELD_NFTOKEN_OFFERS, buf)?;
//         for offer in &self.nft.nftoken_offers {
//             serialize_hash256_field(FIELD_NFTOKEN_OFFERS, offer, buf)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct NFTokenAcceptOfferFields {
//     pub nftoken_buy_offer: Option<Hash256>,
//     pub nftoken_sell_offer: Option<Hash256>,
//     pub nftoken_broker_fee: Option<TokenAmount>,
// }

// pub struct NFTokenAcceptOfferTxn {
//     pub common: CommonFields,
//     pub nft: NFTokenAcceptOfferFields,
// }

// impl NFTokenAcceptOfferTxn {
//     pub fn new(common: CommonFields, nft: NFTokenAcceptOfferFields) -> Self {
//         Self { common, nft }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(ref broker_fee) = self.nft.nftoken_broker_fee {
//             encode_field_id(STI_AMOUNT, FIELD_NFTOKEN_BROKER_FEE, buf)?;
//             serialize_amount(broker_fee, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         if let Some(ref buy_offer) = self.nft.nftoken_buy_offer {
//             serialize_hash256_field(FIELD_NFTOKEN_BUY_OFFER, buy_offer, buf)?;
//         }
        
//         if let Some(ref sell_offer) = self.nft.nftoken_sell_offer {
//             serialize_hash256_field(FIELD_NFTOKEN_SELL_OFFER, sell_offer, buf)?;
//         }
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct NFTokenModifyFields {
//     pub nftoken_id: NFTokenID,
//     pub owner: Option<AccountID>,
//     pub uri: Option<Blob>,
// }

// pub struct NFTokenModifyTxn {
//     pub common: CommonFields,
//     pub nft: NFTokenModifyFields,
// }

// impl NFTokenModifyTxn {
//     pub fn new(common: CommonFields, nft: NFTokenModifyFields) -> Self {
//         Self { common, nft }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref uri) = self.nft.uri {
//             serialize_blob_field(FIELD_URI, uri, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_nftokenid_field(FIELD_NFTOKEN_ID, &self.nft.nftoken_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref owner) = self.nft.owner {
//             encode_field_id(STI_ACCOUNT, FIELD_OWNER, buf)?;
//             serialize_length_prefix(owner.0.len(), buf)?;
//             buf.extend_from_slice(&owner.0)?;
//         }
        
//         Ok(())
//     }
// }