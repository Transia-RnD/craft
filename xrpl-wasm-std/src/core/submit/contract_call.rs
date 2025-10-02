// use crate::core::types::account_id::AccountID;
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer, 
//     serialize_common_fields, serialize_amount, serialize_fee_field, 
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_length_prefix
// };

// use crate::core::type_codes::{
//     STI_AMOUNT, STI_ACCOUNT, STI_VL, STI_UINT32, STI_UINT64, STI_OBJECT, STI_ARRAY, 
//     STI_DATA, STI_DATA_TYPE, STI_UINT8, STI_UINT16, STI_UINT128, STI_UINT160, 
//     STI_UINT192, STI_UINT256, STI_ISSUE, STI_CURRENCY
// };

// // Field codes for ContractCall transaction
// const FIELD_CONTRACT_ACCOUNT: u8 = 25; // Assuming field code for ContractAccount
// const FIELD_FUNCTION_NAME: u8 = 34; // Assuming field code for FunctionName
// const FIELD_PARAMETERS: u8 = 36; // Assuming field code for Parameters
// const FIELD_PARAMETER: u8 = 40; // Assuming field code for Parameter
// const FIELD_COMPUTATION_ALLOWANCE: u8 = 56; // Assuming field code for ComputationAllowance
// const FIELD_PARAMETER_FLAG: u8 = 58; // Assuming field code for ParameterFlag
// const FIELD_PARAMETER_NAME: u8 = 35; // Assuming field code for ParameterName
// const FIELD_PARAMETER_TYPE: u8 = 1; // Assuming field code for ParameterType
// const FIELD_PARAMETER_VALUE: u8 = 1; // Assuming field code for ParameterValue

// // XRPL serialization markers
// const ARRAY_END: u8 = 0xF1;
// const OBJECT_END: u8 = 0xE1;

// /// Parameter value types - matching STData inner types
// #[derive(Debug, Clone)]
// pub enum ParameterValueType<'a> {
//     UInt8(u8),
//     UInt16(u16),
//     UInt32(u32),
//     UInt64(u64),
//     UInt128([u8; 16]),
//     UInt160([u8; 20]),
//     UInt192([u8; 24]),
//     UInt256([u8; 32]),
//     Blob(&'a [u8]),
//     Account(AccountID),
//     Amount(TokenAmount),
//     Issue([u8; 40]),      // 20 bytes currency + 20 bytes account
//     Currency([u8; 20]),   // 20 bytes for currency
// }

// /// Parameter value with type information
// #[derive(Debug, Clone)]
// pub struct ParameterValue<'a> {
//     pub value_type: &'a str,
//     pub value: ParameterValueType<'a>,
// }

// /// Individual parameter
// #[derive(Debug, Clone)]
// pub struct Parameter<'a> {
//     pub parameter_flag: Option<u32>,
//     pub parameter_name: Option<&'a str>,
//     pub parameter_type: Option<&'a str>,
//     pub parameter_value: Option<ParameterValue<'a>>,
// }

// /// ContractCall-specific transaction fields
// pub struct ContractCallTemplate<'a> {
//     pub contract_account: AccountID,
//     pub function_name: &'a [u8], // Hex-encoded function name
//     pub parameters: Option<&'a [Parameter<'a>]>,
//     pub computation_allowance: u32,
// }

// /// Complete ContractCall transaction
// pub struct ContractCallTxn<'a> {
//     pub common: CommonFields,
//     pub contract_call: ContractCallTemplate<'a>,
// }

// impl<'a> ContractCallTxn<'a> {
//     /// Create a new ContractCall transaction
//     pub fn new(common: CommonFields, contract_call: ContractCallTemplate<'a>) -> Self {
//         Self { common, contract_call }
//     }

//     /// Serialize the ContractCall transaction according to XRPL canonical field ordering
//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         // Serialize fields in canonical order (by field ID: (type_code << 16) | field_code)
        
//         // 1-3. Common fields: TransactionType, Flags, Sequence
//         serialize_common_fields(&self.common, buf)?;

//         // 4. ComputationAllowance (Type 3, Field 15) - UInt32
//         encode_field_id(STI_UINT32, FIELD_COMPUTATION_ALLOWANCE, buf)?;
//         buf.extend_from_slice(&self.contract_call.computation_allowance.to_be_bytes())?;

//         // 5. Fee (Type 6, Field 8) = 393224 - Amount
//         serialize_fee_field(&self.common.fee, buf)?;

//         // 6. FunctionName (Type 7, Field 13) - Blob
//         encode_field_id(STI_VL, FIELD_FUNCTION_NAME, buf)?;
//         serialize_length_prefix(self.contract_call.function_name.len(), buf)?;
//         buf.extend_from_slice(&self.contract_call.function_name)?;

//         // 7. SigningPubKey (Type 7, Field 3) = 458755 - Blob
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;

//         // 8. Account (Type 8, Field 1) = 524289 - AccountID
//         serialize_account_field(&self.common.account, buf)?;

//         // 9. ContractAccount (Type 8, Field 12) - AccountID
//         encode_field_id(STI_ACCOUNT, FIELD_CONTRACT_ACCOUNT, buf)?;
//         serialize_length_prefix(self.contract_call.contract_account.0.len(), buf)?;
//         buf.extend_from_slice(&self.contract_call.contract_account.0)?;

//         // 10. Parameters (Type 19, Field 14) - Array (if present)
//         if let Some(parameters) = self.contract_call.parameters {
//             encode_field_id(STI_ARRAY, FIELD_PARAMETERS, buf)?;
//             self.serialize_parameters(parameters, buf)?;
//         }

//         Ok(())
//     }

//     /// Serialize the parameters array
//     fn serialize_parameters(&self, parameters: &[Parameter], buf: &mut SerializationBuffer) -> Result<(), &'static str> {        
//         for parameter in parameters {
//             // Each parameter is an object in the array
//             self.serialize_parameter(parameter, buf)?;
//         }
        
//         // Array end marker
//         buf.push(ARRAY_END)?;
//         Ok(())
//     }

//     /// Serialize an individual parameter as an object
//     fn serialize_parameter(&self, parameter: &Parameter, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         encode_field_id(STI_OBJECT, FIELD_PARAMETER, buf)?;

//         // ParameterFlag (optional) - serialize fields in canonical order
//         if let Some(flag) = parameter.parameter_flag {
//             encode_field_id(STI_UINT32, FIELD_PARAMETER_FLAG, buf)?;
//             buf.extend_from_slice(&flag.to_be_bytes())?;
//         }

//         // ParameterName (optional)
//         if let Some(name) = parameter.parameter_name {
//             encode_field_id(STI_VL, FIELD_PARAMETER_NAME, buf)?;
//             serialize_length_prefix(name.len(), buf)?;
//             buf.extend_from_slice(name.as_bytes())?;
//         }

//         // ParameterType (optional) - Use STI_DATA_TYPE for type specification
//         if let Some(param_type) = parameter.parameter_type {
//             encode_field_id(STI_DATA_TYPE, FIELD_PARAMETER_TYPE, buf)?;
//             self.serialize_parameter_type(param_type, buf)?;
//         }

//         // ParameterValue (optional) - Use STI_DATA for the actual value
//         if let Some(value) = &parameter.parameter_value {
//             self.serialize_parameter_value(value, buf)?;
//         }

//         // Object end marker
//         buf.push(OBJECT_END)?;
//         Ok(())
//     }

//     /// Serialize parameter type using STDataType format
//     fn serialize_parameter_type(&self, param_type: &str, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         // STDataType serializes the inner type ID as a 16-bit value
//         let type_id = match param_type {
//             "UINT8" => STI_UINT8,
//             "UINT16" => STI_UINT16, 
//             "UINT32" => STI_UINT32,
//             "UINT64" => STI_UINT64,
//             "UINT128" => STI_UINT128,
//             "UINT160" => STI_UINT160,
//             "UINT192" => STI_UINT192,
//             "UINT256" => STI_UINT256,
//             "VL" => STI_VL,
//             "ACCOUNT" => STI_ACCOUNT,
//             "AMOUNT" => STI_AMOUNT,
//             "ISSUE" => STI_ISSUE,
//             "CURRENCY" => STI_CURRENCY,
//             _ => return Err("Unknown parameter type"),
//         };
        
//         // Serialize the type ID as 16-bit big-endian
//         buf.extend_from_slice(&(type_id as u16).to_be_bytes())?;
//         Ok(())
//     }

//     /// Serialize a parameter value using STData format
//     fn serialize_parameter_value(&self, value: &ParameterValue<'a>, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         // Encode the ParameterValue field as STI_DATA
//         encode_field_id(STI_DATA, FIELD_PARAMETER_VALUE, buf)?;
        
//         // STData first serializes the inner type ID as 16-bit value
//         let type_id = match &value.value {
//             ParameterValueType::UInt8(_) => STI_UINT8,
//             ParameterValueType::UInt16(_) => STI_UINT16,
//             ParameterValueType::UInt32(_) => STI_UINT32,
//             ParameterValueType::UInt64(_) => STI_UINT64,
//             ParameterValueType::UInt128(_) => STI_UINT128,
//             ParameterValueType::UInt160(_) => STI_UINT160,
//             ParameterValueType::UInt192(_) => STI_UINT192,
//             ParameterValueType::UInt256(_) => STI_UINT256,
//             ParameterValueType::Blob(_) => STI_VL,
//             ParameterValueType::Account(_) => STI_ACCOUNT,
//             ParameterValueType::Amount(_) => STI_AMOUNT,
//             ParameterValueType::Issue(_) => STI_ISSUE,
//             ParameterValueType::Currency(_) => STI_CURRENCY,
//         };
        
//         // Serialize type ID first (as per STData::add method)
//         buf.extend_from_slice(&(type_id as u16).to_be_bytes())?;

//         // Then serialize the actual value based on type
//         match &value.value {
//             ParameterValueType::UInt8(v) => {
//                 buf.extend_from_slice(&[*v])?;
//             },
//             ParameterValueType::UInt16(v) => {
//                 buf.extend_from_slice(&v.to_be_bytes())?;
//             },
//             ParameterValueType::UInt32(v) => {
//                 buf.extend_from_slice(&v.to_be_bytes())?;
//             },
//             ParameterValueType::UInt64(v) => {
//                 buf.extend_from_slice(&v.to_be_bytes())?;
//             },
//             ParameterValueType::UInt128(v) => {
//                 buf.extend_from_slice(v)?;
//             },
//             ParameterValueType::UInt160(v) => {
//                 buf.extend_from_slice(v)?;
//             },
//             ParameterValueType::UInt192(v) => {
//                 buf.extend_from_slice(v)?;
//             },
//             ParameterValueType::UInt256(v) => {
//                 buf.extend_from_slice(v)?;
//             },
//             ParameterValueType::Blob(v) => {
//                 // VL types need length prefix
//                 serialize_length_prefix(v.len(), buf)?;
//                 buf.extend_from_slice(v)?;
//             },
//             ParameterValueType::Account(account) => {
//                 // Account IDs are serialized as VL with length prefix
//                 serialize_length_prefix(account.0.len(), buf)?;
//                 buf.extend_from_slice(&account.0)?;
//             },
//             ParameterValueType::Amount(amount) => {
//                 // Use existing amount serialization
//                 serialize_amount(amount, buf)?;
//             },
//             ParameterValueType::Issue(issue_data) => {
//                 // Issue is 40 bytes: 20 for currency + 20 for account
//                 buf.extend_from_slice(issue_data)?;
//             },
//             ParameterValueType::Currency(currency_data) => {
//                 // Currency is 20 bytes
//                 buf.extend_from_slice(currency_data)?;
//             },
//         }

//         Ok(())
//     }
// }

// impl<'a> ParameterValue<'a> {
//     /// Create a new ParameterValue from a numeric value for integer types
//     pub fn new_uint8(value: u8) -> Self {
//         ParameterValue {
//             value_type: "UINT8",
//             value: ParameterValueType::UInt8(value),
//         }
//     }

//     pub fn new_uint16(value: u16) -> Self {
//         ParameterValue {
//             value_type: "UINT16",
//             value: ParameterValueType::UInt16(value),
//         }
//     }

//     pub fn new_uint32(value: u32) -> Self {
//         ParameterValue {
//             value_type: "UINT32",
//             value: ParameterValueType::UInt32(value),
//         }
//     }

//     pub fn new_uint64(value: u64) -> Self {
//         ParameterValue {
//             value_type: "UINT64",
//             value: ParameterValueType::UInt64(value),
//         }
//     }

//     pub fn new_uint128(value: [u8; 16]) -> Self {
//         ParameterValue {
//             value_type: "UINT128",
//             value: ParameterValueType::UInt128(value),
//         }
//     }

//     pub fn new_uint160(value: [u8; 20]) -> Self {
//         ParameterValue {
//             value_type: "UINT160",
//             value: ParameterValueType::UInt160(value),
//         }
//     }

//     pub fn new_uint192(value: [u8; 24]) -> Self {
//         ParameterValue {
//             value_type: "UINT192",
//             value: ParameterValueType::UInt192(value),
//         }
//     }

//     pub fn new_uint256(value: [u8; 32]) -> Self {
//         ParameterValue {
//             value_type: "UINT256",
//             value: ParameterValueType::UInt256(value),
//         }
//     }

//     pub fn new_blob(value: &'a [u8]) -> Self {
//         ParameterValue {
//             value_type: "VL",
//             value: ParameterValueType::Blob(value),
//         }
//     }

//     pub fn new_account(value: AccountID) -> Self {
//         ParameterValue {
//             value_type: "ACCOUNT",
//             value: ParameterValueType::Account(value),
//         }
//     }

//     pub fn new_amount(value: TokenAmount) -> Self {
//         ParameterValue {
//             value_type: "AMOUNT",
//             value: ParameterValueType::Amount(value),
//         }
//     }

//     pub fn new_issue(value: [u8; 40]) -> Self {
//         ParameterValue {
//             value_type: "ISSUE",
//             value: ParameterValueType::Issue(value),
//         }
//     }

//     pub fn new_currency(value: [u8; 20]) -> Self {
//         ParameterValue {
//             value_type: "CURRENCY",
//             value: ParameterValueType::Currency(value),
//         }
//     }
// }