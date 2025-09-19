// // ============================================================================
// // DEPOSIT PREAUTH TRANSACTION
// // ============================================================================

// use crate::core::types::account_id::AccountID;
// use crate::core::types::credential::CredentialID;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_array_field,
//     FIELD_AUTHORIZE, FIELD_UNAUTHORIZE,
//     FIELD_AUTHORIZE_CREDENTIALS, FIELD_UNAUTHORIZE_CREDENTIALS,
// };
// use crate::core::type_codes::STI_ACCOUNT;

// pub struct DepositPreauthFields {
//     pub authorize: Option<AccountID>,
//     pub unauthorize: Option<AccountID>,
//     pub authorize_credentials: Option<Vec<CredentialID>>,
//     pub unauthorize_credentials: Option<Vec<CredentialID>>,
// }

// pub struct DepositPreauthTxn {
//     pub common: CommonFields,
//     pub preauth: DepositPreauthFields,
// }

// impl DepositPreauthTxn {
//     pub fn new(common: CommonFields, preauth: DepositPreauthFields) -> Self {
//         Self { common, preauth }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref authorize) = self.preauth.authorize {
//             encode_field_id(STI_ACCOUNT, FIELD_AUTHORIZE, buf)?;
//             serialize_length_prefix(authorize.0.len(), buf)?;
//             buf.extend_from_slice(&authorize.0)?;
//         }
        
//         if let Some(ref unauthorize) = self.preauth.unauthorize {
//             encode_field_id(STI_ACCOUNT, FIELD_UNAUTHORIZE, buf)?;
//             serialize_length_prefix(unauthorize.0.len(), buf)?;
//             buf.extend_from_slice(&unauthorize.0)?;
//         }
        
//         if let Some(ref auth_creds) = self.preauth.authorize_credentials {
//             serialize_array_field(FIELD_AUTHORIZE_CREDENTIALS, auth_creds, buf)?;
//         }
        
//         if let Some(ref unauth_creds) = self.preauth.unauthorize_credentials {
//             serialize_array_field(FIELD_UNAUTHORIZE_CREDENTIALS, unauth_creds, buf)?;
//         }
        
//         Ok(())
//     }
// }