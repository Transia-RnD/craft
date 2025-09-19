// // ============================================================================
// // CONTRACT TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::hash::Hash256;
// use crate::core::types::blob::Blob;
// use crate::core::types::contract::ContractAccount;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     serialize_blob_field, serialize_hash256_field,
//     serialize_array_field, serialize_contractaccount_field,
//     // Field codes
//     FIELD_CONTRACT_CODE, FIELD_CONTRACT_HASH,
//     FIELD_FUNCTIONS, FIELD_INSTANCE_PARAMETERS, FIELD_INSTANCE_PARAMETER_VALUES,
//     FIELD_CONTRACT_ACCOUNT, FIELD_FUNCTION_NAME, FIELD_PARAMETERS,
//     FIELD_COMPUTATION_ALLOWANCE, FIELD_URI
// };

// pub struct ContractCreateFields {
//     pub contract_code: Option<Blob>,
//     pub contract_hash: Option<Hash256>,
//     pub functions: Option<Vec<Function>>,
//     pub instance_parameters: Option<Vec<InstanceParameter>>,
//     pub instance_parameter_values: Option<Vec<InstanceParameterValue>>,
//     pub uri: Option<Blob>,
// }

// pub struct Function {
//     pub function_name: Blob,
//     pub parameters: Option<Vec<Parameter>>,
// }

// pub struct InstanceParameter {
//     pub parameter_flag: u32,
//     pub parameter_name: Blob,
//     pub parameter_type: Blob,
// }

// pub struct InstanceParameterValue {
//     pub parameter_flag: u32,
//     pub parameter_value: Blob,
// }

// pub struct Parameter {
//     pub parameter_flag: Option<u32>,
//     pub parameter_name: Option<Blob>,
//     pub parameter_type: Option<Blob>,
//     pub parameter_value: Option<Blob>,
// }

// pub struct ContractCreateTxn {
//     pub common: CommonFields,
//     pub contract: ContractCreateFields,
// }

// impl ContractCreateTxn {
//     pub fn new(common: CommonFields, contract: ContractCreateFields) -> Self {
//         Self { common, contract }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
        
//         if let Some(ref code) = self.contract.contract_code {
//             serialize_blob_field(FIELD_CONTRACT_CODE, code, buf)?;
//         }
        
//         if let Some(ref uri) = self.contract.uri {
//             serialize_blob_field(FIELD_URI, uri, buf)?;
//         }
        
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         if let Some(ref hash) = self.contract.contract_hash {
//             serialize_hash256_field(FIELD_CONTRACT_HASH, hash, buf)?;
//         }
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref functions) = self.contract.functions {
//             serialize_array_field(FIELD_FUNCTIONS, functions, buf)?;
//         }
        
//         if let Some(ref params) = self.contract.instance_parameters {
//             serialize_array_field(FIELD_INSTANCE_PARAMETERS, params, buf)?;
//         }
        
//         if let Some(ref values) = self.contract.instance_parameter_values {
//             serialize_array_field(FIELD_INSTANCE_PARAMETER_VALUES, values, buf)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct ContractCallFields {
//     pub contract_account: ContractAccount,
//     pub function_name: Blob,
//     pub parameters: Option<Vec<Parameter>>,
//     pub computation_allowance: u64,
// }

// pub struct ContractCallTxn {
//     pub common: CommonFields,
//     pub contract: ContractCallFields,
// }

// impl ContractCallTxn {
//     pub fn new(common: CommonFields, contract: ContractCallFields) -> Self {
//         Self { common, contract }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint64_field(FIELD_COMPUTATION_ALLOWANCE, self.contract.computation_allowance, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_blob_field(FIELD_FUNCTION_NAME, &self.contract.function_name, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         serialize_contractaccount_field(FIELD_CONTRACT_ACCOUNT, &self.contract.contract_account, buf)?;
        
//         if let Some(ref params) = self.contract.parameters {
//             serialize_array_field(FIELD_PARAMETERS, params, buf)?;
//         }
        
//         Ok(())
//     }
// }