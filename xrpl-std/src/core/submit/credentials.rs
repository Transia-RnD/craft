// // ============================================================================
// // CREDENTIAL TRANSACTIONS
// // ============================================================================

// use crate::core::types::account_id::AccountID;
// use crate::core::types::blob::Blob;
// use crate::core::types::credential::CredentialType;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_uint32_field, serialize_blob_field,
//     // Field codes
//     FIELD_EXPIRATION, FIELD_URI, FIELD_ISSUER, FIELD_SUBJECT, FIELD_CREDENTIAL_TYPE,
// };
// use crate::core::type_codes::STI_ACCOUNT;

// pub struct CredentialCreateFields {
//     pub subject: AccountID,
//     pub credential_type: CredentialType,
//     pub expiration: Option<u32>,
//     pub uri: Option<Blob>,
// }

// pub struct CredentialCreateTxn {
//     pub common: CommonFields,
//     pub credential: CredentialCreateFields,
// }

// impl CredentialCreateTxn {
//     pub fn new(common: CommonFields, credential: CredentialCreateFields) -> Self {
//         Self { common, credential }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(expiration) = self.credential.expiration {
//             serialize_uint32_field(FIELD_EXPIRATION, expiration, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref uri) = self.credential.uri {
//             serialize_blob_field(FIELD_URI, uri, buf)?;
//         }
        
//         serialize_blob_field(FIELD_CREDENTIAL_TYPE, &self.credential.credential_type.0, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_SUBJECT, buf)?;
//         serialize_length_prefix(self.credential.subject.0.len(), buf)?;
//         buf.extend_from_slice(&self.credential.subject.0)?;
        
//         Ok(())
//     }
// }

// pub struct CredentialAcceptFields {
//     pub issuer: AccountID,
//     pub credential_type: CredentialType,
// }

// pub struct CredentialAcceptTxn {
//     pub common: CommonFields,
//     pub credential: CredentialAcceptFields,
// }

// impl CredentialAcceptTxn {
//     pub fn new(common: CommonFields, credential: CredentialAcceptFields) -> Self {
//         Self { common, credential }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_blob_field(FIELD_CREDENTIAL_TYPE, &self.credential.credential_type.0, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_ISSUER, buf)?;
//         serialize_length_prefix(self.credential.issuer.0.len(), buf)?;
//         buf.extend_from_slice(&self.credential.issuer.0)?;
        
//         Ok(())
//     }
// }

// pub struct CredentialDeleteFields {
//     pub subject: Option<AccountID>,
//     pub issuer: Option<AccountID>,
//     pub credential_type: CredentialType,
// }

// pub struct CredentialDeleteTxn {
//     pub common: CommonFields,
//     pub credential: CredentialDeleteFields,
// }

// impl CredentialDeleteTxn {
//     pub fn new(common: CommonFields, credential: CredentialDeleteFields) -> Self {
//         Self { common, credential }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_blob_field(FIELD_CREDENTIAL_TYPE, &self.credential.credential_type.0, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref subject) = self.credential.subject {
//             encode_field_id(STI_ACCOUNT, FIELD_SUBJECT, buf)?;
//             serialize_length_prefix(subject.0.len(), buf)?;
//             buf.extend_from_slice(&subject.0)?;
//         }
        
//         if let Some(ref issuer) = self.credential.issuer {
//             encode_field_id(STI_ACCOUNT, FIELD_ISSUER, buf)?;
//             serialize_length_prefix(issuer.0.len(), buf)?;
//             buf.extend_from_slice(&issuer.0)?;
//         }
        
//         Ok(())
//     }
// }