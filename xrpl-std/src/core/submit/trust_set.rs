// // ============================================================================
// // TRUST SET TRANSACTION
// // ============================================================================
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_uint32_field,
//     FIELD_LIMIT_AMOUNT, FIELD_QUALITY_IN, FIELD_QUALITY_OUT,
// };
// use crate::core::type_codes::STI_AMOUNT;

// pub struct TrustSetFields {
//     pub limit_amount: Option<TokenAmount>,
//     pub quality_in: Option<u32>,
//     pub quality_out: Option<u32>,
// }

// pub struct TrustSetTxn {
//     pub common: CommonFields,
//     pub trust: TrustSetFields,
// }

// impl TrustSetTxn {
//     pub fn new(common: CommonFields, trust: TrustSetFields) -> Self {
//         Self { common, trust }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(quality_in) = self.trust.quality_in {
//             serialize_uint32_field(FIELD_QUALITY_IN, quality_in, buf)?;
//         }
        
//         if let Some(quality_out) = self.trust.quality_out {
//             serialize_uint32_field(FIELD_QUALITY_OUT, quality_out, buf)?;
//         }
        
//         if let Some(ref limit) = self.trust.limit_amount {
//             encode_field_id(STI_AMOUNT, FIELD_LIMIT_AMOUNT, buf)?;
//             serialize_amount(limit, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }