#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::core::types::public_key::PublicKey;
use xrpl_std::core::types::amount::token_amount::TokenAmount;
use xrpl_std::host::trace::{DataRepr, trace, trace_data, trace_num};
use xrpl_std::host::{instance_param, emit_txn, build_txn};
use xrpl_std::core::current_tx::contract_call::{ContractCall, get_current_contract_call};
use xrpl_std::core::current_tx::traits::{ContractCallFields, TransactionCommonFields};
use xrpl_std::core::ledger_objects::account::get_account_sequence;
use xrpl_std::core::submit::common::{CommonFields, SerializationBuffer};
use xrpl_std::core::submit::payment::{PaymentFields, PaymentTxn};

const CUSTOM_ERROR_CODE: i32 = -18;

#[unsafe(no_mangle)]
pub extern "C" fn emit() -> i32 {
    let _ = trace("$$$$$ STARTING WASM EXECUTION $$$$$");
    let contract_call: ContractCall = get_current_contract_call();
    let account = contract_call.get_account().unwrap();
    let _ = trace_data("  Account:", &account.0, DataRepr::AsHex);
    let contract_account = contract_call.get_contract_account().unwrap();
    let _ = trace_data("  Contract Account:", &contract_account.0, DataRepr::AsHex);

    // Create signing public key (empty for contract calls)
    const PUBKEY_SECP256K1: &[u8] = &[0x00];
    let pubkey = PublicKey::from(PUBKEY_SECP256K1);
    let _ = trace_data("  SigningPubKey:", &pubkey.0, DataRepr::AsHex);

    // For testing, return error code
    // return CUSTOM_ERROR_CODE;

    // Example destination AccountID
    const DESTINATION: [u8; 20] = [
        0xAE, 0x12, 0x3A, 0x85, 0x56, 0xF3, 0xCF, 0x91, 0x15, 0x47, 
        0x11, 0x37, 0x6A, 0xFB, 0x0F, 0x89, 0x4F, 0x83, 0x2B, 0x3D
    ];
    let destination = AccountID(DESTINATION);

    // Create fee amount (0 XRP for now)
    const FEE: [u8; 8] = [
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ];
    let fee = match TokenAmount::from_bytes(&FEE) {
        Ok(f) => f,
        Err(_) => {
            let _ = trace("Failed to parse fee TokenAmount");
            return CUSTOM_ERROR_CODE;
        }
    };

    // Create payment amount (192 drops = 0.000192 XRP)
    const AMOUNT: [u8; 8] = [
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0
    ];
    let amount = match TokenAmount::from_bytes(&AMOUNT) {
        Ok(f) => f,
        Err(_) => {
            let _ = trace("Failed to parse fee TokenAmount");
            return CUSTOM_ERROR_CODE;
        }
    };

    // Get next sequence number
    let next_sequence = get_account_sequence(&contract_account);
    let sequence_value = next_sequence.unwrap_or(0);
    let _ = trace_num("Next Sequence:", sequence_value as i64);

    // Create common transaction fields
    let common = CommonFields {
        transaction_type: 0, // Payment
        sequence: sequence_value,
        fee,
        account: contract_account,
        signing_pub_key: pubkey,
        flags: 536870912, // Standard flags
    };

    // Create payment-specific fields
    let payment_fields = PaymentFields {
        destination,
        amount,
    };

    // Create payment transaction
    let txn = PaymentTxn::new(common, payment_fields);

    // Serialize and submit transaction
    let mut txn_buf = SerializationBuffer::new();
    match txn.serialize(&mut txn_buf) {
        Ok(()) => {
            let ter_result = unsafe { 
                emit_txn(
                    txn_buf.as_slice().as_ptr(), 
                    txn_buf.len(),
                )
            };
            
            // Trace the results
            let _ = trace_data("Serialized PaymentTxn:", txn_buf.as_slice(), DataRepr::AsHex);
            let _ = trace_num("Submit Result:", ter_result.into());
        }
        Err(e) => {
            let _ = trace("Serialization error occurred");
            return CUSTOM_ERROR_CODE;
        }
    }

    0 // Return success
}