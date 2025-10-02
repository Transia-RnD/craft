// // ============================================================================
// // AMM TRANSACTIONS
// // ============================================================================
// use crate::core::types::account_id::AccountID;
// use crate::core::types::amount::token_amount::TokenAmount;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_amount, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     encode_field_id, serialize_uint16_field, serialize_array_field,
//     // Field codes
//     FIELD_ASSET, FIELD_ASSET2,
//     FIELD_TRADING_FEE, FIELD_EPRICE, FIELD_LP_TOKEN_OUT, FIELD_LP_TOKEN_IN,
//     FIELD_AMOUNT2, FIELD_BID_MIN, FIELD_BID_MAX, FIELD_AUTH_ACCOUNTS,
// };
// use crate::core::type_codes::{
//     STI_AMOUNT, STI_ISSUE,
// };

// pub struct AMMCreateFields {
//     pub amount: TokenAmount,
//     pub amount2: TokenAmount,
//     pub trading_fee: u16,
// }

// pub struct AMMCreateTxn {
//     pub common: CommonFields,
//     pub amm: AMMCreateFields,
// }

// impl AMMCreateTxn {
//     pub fn new(common: CommonFields, amm: AMMCreateFields) -> Self {
//         Self { common, amm }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint16_field(FIELD_TRADING_FEE, self.amm.trading_fee, buf)?;
//         serialize_amount(&self.amm.amount, buf)?;
        
//         encode_field_id(STI_AMOUNT, FIELD_AMOUNT2, buf)?;
//         serialize_amount(&self.amm.amount2, buf)?;
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct AMMDepositFields {
//     pub asset: TokenAmount,
//     pub asset2: TokenAmount,
//     pub amount: Option<TokenAmount>,
//     pub amount2: Option<TokenAmount>,
//     pub eprice: Option<TokenAmount>,
//     pub lp_token_out: Option<TokenAmount>,
//     pub trading_fee: Option<u16>,
// }

// pub struct AMMDepositTxn {
//     pub common: CommonFields,
//     pub amm: AMMDepositFields,
// }

// impl AMMDepositTxn {
//     pub fn new(common: CommonFields, amm: AMMDepositFields) -> Self {
//         Self { common, amm }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(trading_fee) = self.amm.trading_fee {
//             serialize_uint16_field(FIELD_TRADING_FEE, trading_fee, buf)?;
//         }
        
//         if let Some(ref amount) = self.amm.amount {
//             serialize_amount(amount, buf)?;
//         }
        
//         if let Some(ref amount2) = self.amm.amount2 {
//             encode_field_id(STI_AMOUNT, FIELD_AMOUNT2, buf)?;
//             serialize_amount(amount2, buf)?;
//         }
        
//         if let Some(ref eprice) = self.amm.eprice {
//             encode_field_id(STI_AMOUNT, FIELD_EPRICE, buf)?;
//             serialize_amount(eprice, buf)?;
//         }
        
//         if let Some(ref lp_token) = self.amm.lp_token_out {
//             encode_field_id(STI_AMOUNT, FIELD_LP_TOKEN_OUT, buf)?;
//             serialize_amount(lp_token, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET, buf)?;
//         serialize_amount(&self.amm.asset, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET2, buf)?;
//         serialize_amount(&self.amm.asset2, buf)?;
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct AMMWithdrawFields {
//     pub asset: TokenAmount,
//     pub asset2: TokenAmount,
//     pub amount: Option<TokenAmount>,
//     pub amount2: Option<TokenAmount>,
//     pub eprice: Option<TokenAmount>,
//     pub lp_token_in: Option<TokenAmount>,
// }

// pub struct AMMWithdrawTxn {
//     pub common: CommonFields,
//     pub amm: AMMWithdrawFields,
// }

// impl AMMWithdrawTxn {
//     pub fn new(common: CommonFields, amm: AMMWithdrawFields) -> Self {
//         Self { common, amm }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(ref amount) = self.amm.amount {
//             serialize_amount(amount, buf)?;
//         }
        
//         if let Some(ref amount2) = self.amm.amount2 {
//             encode_field_id(STI_AMOUNT, FIELD_AMOUNT2, buf)?;
//             serialize_amount(amount2, buf)?;
//         }
        
//         if let Some(ref eprice) = self.amm.eprice {
//             encode_field_id(STI_AMOUNT, FIELD_EPRICE, buf)?;
//             serialize_amount(eprice, buf)?;
//         }
        
//         if let Some(ref lp_token) = self.amm.lp_token_in {
//             encode_field_id(STI_AMOUNT, FIELD_LP_TOKEN_IN, buf)?;
//             serialize_amount(lp_token, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET, buf)?;
//         serialize_amount(&self.amm.asset, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET2, buf)?;
//         serialize_amount(&self.amm.asset2, buf)?;
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct AMMVoteFields {
//     pub asset: TokenAmount,
//     pub asset2: TokenAmount,
//     pub trading_fee: u16,
// }

// pub struct AMMVoteTxn {
//     pub common: CommonFields,
//     pub amm: AMMVoteFields,
// }

// impl AMMVoteTxn {
//     pub fn new(common: CommonFields, amm: AMMVoteFields) -> Self {
//         Self { common, amm }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint16_field(FIELD_TRADING_FEE, self.amm.trading_fee, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET, buf)?;
//         serialize_amount(&self.amm.asset, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET2, buf)?;
//         serialize_amount(&self.amm.asset2, buf)?;
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }

// pub struct AMMBidFields {
//     pub asset: TokenAmount,
//     pub asset2: TokenAmount,
//     pub bid_min: Option<TokenAmount>,
//     pub bid_max: Option<TokenAmount>,
//     pub auth_accounts: Option<Vec<AccountID>>,
// }

// pub struct AMMBidTxn {
//     pub common: CommonFields,
//     pub amm: AMMBidFields,
// }

// impl AMMBidTxn {
//     pub fn new(common: CommonFields, amm: AMMBidFields) -> Self {
//         Self { common, amm }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
        
//         if let Some(ref bid_min) = self.amm.bid_min {
//             encode_field_id(STI_AMOUNT, FIELD_BID_MIN, buf)?;
//             serialize_amount(bid_min, buf)?;
//         }
        
//         if let Some(ref bid_max) = self.amm.bid_max {
//             encode_field_id(STI_AMOUNT, FIELD_BID_MAX, buf)?;
//             serialize_amount(bid_max, buf)?;
//         }
        
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET, buf)?;
//         serialize_amount(&self.amm.asset, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET2, buf)?;
//         serialize_amount(&self.amm.asset2, buf)?;
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         if let Some(ref auth_accounts) = self.amm.auth_accounts {
//             serialize_array_field(FIELD_AUTH_ACCOUNTS, auth_accounts, buf)?;
//         }
        
//         Ok(())
//     }
// }

// pub struct AMMDeleteFields {
//     pub asset: TokenAmount,
//     pub asset2: TokenAmount,
// }

// pub struct AMMDeleteTxn {
//     pub common: CommonFields,
//     pub amm: AMMDeleteFields,
// }

// impl AMMDeleteTxn {
//     pub fn new(common: CommonFields, amm: AMMDeleteFields) -> Self {
//         Self { common, amm }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET, buf)?;
//         serialize_amount(&self.amm.asset, buf)?;
        
//         encode_field_id(STI_ISSUE, FIELD_ASSET2, buf)?;
//         serialize_amount(&self.amm.asset2, buf)?;
        
//         serialize_account_field(&self.common.account, buf)?;
        
//         Ok(())
//     }
// }