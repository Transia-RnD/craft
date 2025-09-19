// // ============================================================================
// // DELEGATE SET TRANSACTION
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix,
//     serialize_array_field,
//     FIELD_AUTHORIZE, FIELD_PERMISSIONS,
// };
// use crate::core::type_codes::STI_ACCOUNT;

// pub struct DelegateSetFields {
//     pub authorize: AccountID,
//     pub permissions: Vec<Permission>,
// }

// pub struct DelegateSetTxn {
//     pub common: CommonFields,
//     pub delegate: DelegateSetFields,
// }

// impl DelegateSetTxn {
//     pub fn new(common: CommonFields, delegate: DelegateSetFields) -> Self {
//         Self { common, delegate }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         encode_field_id(STI_ACCOUNT, FIELD_AUTHORIZE, buf)?;
//         serialize_length_prefix(self.delegate.authorize.0.len(), buf)?;
//         buf.extend_from_slice(&self.delegate.authorize.0)?;
        
//         serialize_array_field(FIELD_PERMISSIONS, &self.delegate.permissions, buf)?;
        
//         Ok(())
//     }
// }