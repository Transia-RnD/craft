// // ============================================================================
// // TICKET TRANSACTION
// // ============================================================================

// use crate::core::types::account_id::AccountID;
// use crate::core::submit::common::{
//     CommonFields, SerializationBuffer,
//     serialize_common_fields, serialize_fee_field,
//     serialize_signing_pub_key_field, serialize_account_field,
//     serialize_uint32_field,
//     FIELD_TICKET_COUNT,
// };

// pub struct TicketCreateFields {
//     pub ticket_count: u32,
// }

// pub struct TicketCreateTxn {
//     pub common: CommonFields,
//     pub ticket: TicketCreateFields,
// }

// impl TicketCreateTxn {
//     pub fn new(common: CommonFields, ticket: TicketCreateFields) -> Self {
//         Self { common, ticket }
//     }

//     pub fn serialize(&self, buf: &mut SerializationBuffer) -> Result<(), &'static str> {
//         serialize_common_fields(&self.common, buf)?;
//         serialize_uint32_field(FIELD_TICKET_COUNT, self.ticket.ticket_count, buf)?;
//         serialize_fee_field(&self.common.fee, buf)?;
//         serialize_signing_pub_key_field(&self.common.signing_pub_key, buf)?;
//         serialize_account_field(&self.common.account, buf)?;
//         Ok(())
//     }
// }