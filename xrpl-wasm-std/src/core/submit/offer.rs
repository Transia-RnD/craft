// // ============================================================================
// // OFFER TRANSACTIONS
// // ============================================================================

// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::types::hash::Hash256;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_uint32_field, serialize_hash256_field,
//     // Field codes
//     FIELD_OFFER_SEQUENCE, FIELD_TAKER_PAYS, FIELD_TAKER_GETS, FIELD_EXPIRATION, FIELD_DOMAIN_ID,
//     FIELD_FEE, FIELD_SIGNING_PUB_KEY, FIELD_ACCOUNT,
// };
// use crate::core::type_codes::{
//     STI_AMOUNT,
// };

// pub struct OfferCreateFields {
//     pub taker_pays: TokenAmount,
//     pub taker_gets: TokenAmount,
//     pub expiration: Option<u32>,
//     pub offer_sequence: Option<u32>,
//     pub domain_id: Option<Hash256>,
// }

// pub struct OfferCreateTxn {
//     pub common: CommonFields,
//     pub offer: OfferCreateFields,
// }

// impl OfferCreateTxn {
//     pub fn new(common: CommonFields, offer: OfferCreateFields) -> Self {
//         Self { common, offer }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(expiration) = self.offer.expiration {
//             serialize_uint32_field(FIELD_EXPIRATION, expiration, buf)?;
//         }
        
//         if let Some(offer_seq) = self.offer.offer_sequence {
//             serialize_uint32_field(FIELD_OFFER_SEQUENCE, offer_seq, buf)?;
//         }
        
//         encode_field_id(STI_AMOUNT, FIELD_TAKER_GETS, buf)?;
//         serialize_amount(&self.offer.taker_gets, buf)?;
        
//         encode_field_id(STI_AMOUNT, FIELD_TAKER_PAYS, buf)?;
//         serialize_amount(&self.offer.taker_pays, buf)?;
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         if let Some(ref domain_id) = self.offer.domain_id {
//             serialize_hash256_field(FIELD_DOMAIN_ID, domain_id, buf)?;
//         }
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct OfferCancelFields {
//     pub offer_sequence: u32,
// }

// pub struct OfferCancelTxn {
//     pub common: CommonFields,
//     pub offer: OfferCancelFields,
// }

// impl OfferCancelTxn {
//     pub fn new(common: CommonFields, offer: OfferCancelFields) -> Self {
//         Self { common, offer }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint32_field(FIELD_OFFER_SEQUENCE, self.offer.offer_sequence, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         Ok(())
//     }
// }