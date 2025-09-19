// // ============================================================================
// // ORACLE TRANSACTIONS
// // ============================================================================
// use crate::core::types::blob::Blob;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     serialize_uint32_field, serialize_uint64_field,
//     serialize_blob_field, serialize_array_field,
//     FIELD_ORACLE_DOCUMENT_ID, FIELD_PROVIDER, FIELD_ASSET_CLASS,
//     FIELD_LAST_UPDATE_TIME, FIELD_PRICE_DATA_SERIES, FIELD_URI,
// };

// pub struct OracleSetFields {
//     pub oracle_document_id: OracleDocumentID,
//     pub provider: Option<Blob>,
//     pub uri: Option<Blob>,
//     pub asset_class: Option<Blob>,
//     pub last_update_time: u64,
//     pub price_data_series: Vec<PriceDataPoint>,
// }

// pub struct OracleSetTxn {
//     pub common: CommonFields,
//     pub oracle: OracleSetFields,
// }

// impl OracleSetTxn {
//     pub fn new(common: CommonFields, oracle: OracleSetFields) -> Self {
//         Self { common, oracle }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint32_field(FIELD_ORACLE_DOCUMENT_ID, self.oracle.oracle_document_id.0, buf)?;
//         serialize_uint64_field(FIELD_LAST_UPDATE_TIME, self.oracle.last_update_time, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref provider) = self.oracle.provider {
//             serialize_blob_field(FIELD_PROVIDER, provider, buf)?;
//         }
        
//         if let Some(ref uri) = self.oracle.uri {
//             serialize_blob_field(FIELD_URI, uri, buf)?;
//         }
        
//         if let Some(ref asset_class) = self.oracle.asset_class {
//             serialize_blob_field(FIELD_ASSET_CLASS, asset_class, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         // Serialize price data series array
//         serialize_array_field(FIELD_PRICE_DATA_SERIES, &self.oracle.price_data_series, buf)?;
        
//         Ok(())
//     }
// }

// pub struct OracleDeleteFields {
//     pub oracle_document_id: OracleDocumentID,
// }

// pub struct OracleDeleteTxn {
//     pub common: CommonFields,
//     pub oracle: OracleDeleteFields,
// }

// impl OracleDeleteTxn {
//     pub fn new(common: CommonFields, oracle: OracleDeleteFields) -> Self {
//         Self { common, oracle }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint32_field(FIELD_ORACLE_DOCUMENT_ID, self.oracle.oracle_document_id.0, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         Ok(())
//     }
// }
