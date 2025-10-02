// // ============================================================================
// // PERMISSIONED DOMAIN TRANSACTIONS
// // ============================================================================
// use crate::core::types::hash::Hash256;
// use crate::core::types::credential::CredentialID;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     serialize_hash256_field, serialize_array_field,
//     FIELD_DOMAIN_ID, FIELD_ACCEPTED_CREDENTIALS,
// };

// pub struct PermissionedDomainSetFields {
//     pub domain_id: Option<Hash256>,
//     pub accepted_credentials: Vec<CredentialID>,
// }

// pub struct PermissionedDomainSetTxn {
//     pub common: CommonFields,
//     pub domain: PermissionedDomainSetFields,
// }

// impl PermissionedDomainSetTxn {
//     pub fn new(common: CommonFields, domain: PermissionedDomainSetFields) -> Self {
//         Self { common, domain }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         if let Some(ref domain_id) = self.domain.domain_id {
//             serialize_hash256_field(FIELD_DOMAIN_ID, domain_id, buf)?;
//         }
        
//         serialize_account_field(&self.common.account, buf)?;
//         serialize_array_field(FIELD_ACCEPTED_CREDENTIALS, &self.domain.accepted_credentials, buf)?;
        
//         Ok(())
//     }
// }

// pub struct PermissionedDomainDeleteFields {
//     pub domain_id: Hash256,
// }

// pub struct PermissionedDomainDeleteTxn {
//     pub common: CommonFields,
//     pub domain: PermissionedDomainDeleteFields,
// }

// impl PermissionedDomainDeleteTxn {
//     pub fn new(common: CommonFields, domain: PermissionedDomainDeleteFields) -> Self {
//         Self { common, domain }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_hash256_field(FIELD_DOMAIN_ID, &self.domain.domain_id, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         Ok(())
//     }
// }