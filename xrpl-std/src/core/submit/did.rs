// // ============================================================================
// // DID TRANSACTIONS
// // ============================================================================
// use crate::core::types::blob::Blob;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     serialize_blob_field,
//     FIELD_DID_DOCUMENT, FIELD_URI, FIELD_DATA,
// };

// pub struct DIDSetFields {
//     pub did_document: Option<Blob>,
//     pub uri: Option<Blob>,
//     pub data: Option<Blob>,
// }

// pub struct DIDSetTxn {
//     pub common: CommonFields,
//     pub did: DIDSetFields,
// }

// impl DIDSetTxn {
//     pub fn new(common: CommonFields, did: DIDSetFields) -> Self {
//         Self { common, did }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref did_doc) = self.did.did_document {
//             serialize_blob_field(FIELD_DID_DOCUMENT, did_doc, buf)?;
//         }
        
//         if let Some(ref uri) = self.did.uri {
//             serialize_blob_field(FIELD_URI, uri, buf)?;
//         }
        
//         if let Some(ref data) = self.did.data {
//             serialize_blob_field(FIELD_DATA, data, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct DIDDeleteTxn {
//     pub common: CommonFields,
// }

// impl DIDDeleteTxn {
//     pub fn new(common: CommonFields) -> Self {
//         Self { common }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         Ok(())
//     }
// }