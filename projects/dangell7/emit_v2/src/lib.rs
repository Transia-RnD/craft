#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::core::current_tx::contract_call::{ContractCall, get_current_contract_call};
use xrpl_std::core::current_tx::traits::TransactionCommonFields;
use xrpl_std::core::submit::inner_objects::build_memo;
use xrpl_std::core::transaction_types::TT_PAYMENT;
use xrpl_std::core::types::account_id::AccountID;
use xrpl_std::host::{add_txn_field, build_txn, emit_built_txn};

// ============================================================================
// Constants
// ============================================================================

/// Custom error code for transaction failures
const CUSTOM_ERROR_CODE: i32 = -18;

/// XRPL transaction field identifiers
mod field_ids {
    pub const SF_AMOUNT: i32 = 393217;      // (11 << 16) | 1
    pub const SF_DESTINATION: i32 = 524291;  // (13 << 16) | 3
    pub const SF_MEMOS: i32 = 983049;       // (15 << 16) | 9
}

/// XRPL encoding markers
mod markers {
    pub const ARRAY_END: u8 = 0xF1;
    pub const OBJECT_END: u8 = 0xE1;
}

/// Buffer sizes
mod buffer_sizes {
    pub const MEMO_BUFFER: usize = 256;
    pub const MEMOS_ARRAY: usize = 1024;
    pub const DESTINATION: usize = 21;
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Builds a complete memos array from individual memo buffers
/// 
/// # Arguments
/// * `buffer` - Output buffer for the complete memos array
/// * `memo_buffers` - Slice of memo data and their lengths
/// 
/// # Returns
/// Total length of the memos array including the end marker
fn build_memos_array(
    buffer: &mut [u8; buffer_sizes::MEMOS_ARRAY],
    memo_buffers: &[(&[u8], usize)]
) -> usize {
    let mut position = 0;
    
    // Copy each memo into the array
    for (memo_data, memo_length) in memo_buffers {
        buffer[position..position + memo_length].copy_from_slice(&memo_data[..*memo_length]);
        position += memo_length;
    }
    
    // Terminate the array
    buffer[position] = markers::ARRAY_END;
    position + 1
}

/// Adds the amount field to the transaction
/// 
/// # Arguments
/// * `txn_index` - Transaction builder index
/// * `amount_drops` - Amount in drops (192 in this example)
/// 
/// # Returns
/// Result code from add_txn_field
unsafe fn add_amount_field(txn_index: i32) -> i32 {
    // 192 drops encoded as XRPL Amount
    const AMOUNT_BYTES: [u8; 8] = [
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0
    ];
    
    add_txn_field(
        txn_index, 
        field_ids::SF_AMOUNT, 
        AMOUNT_BYTES.as_ptr(), 
        AMOUNT_BYTES.len()
    )
}

/// Adds the destination field to the transaction
/// 
/// # Arguments
/// * `txn_index` - Transaction builder index
/// * `destination` - Destination account ID
/// 
/// # Returns
/// Result code from add_txn_field
unsafe fn add_destination_field(txn_index: i32, destination: &AccountID) -> i32 {
    let mut dest_buffer = [0u8; buffer_sizes::DESTINATION];
    dest_buffer[0] = 0x14; // Length prefix for 20-byte account
    dest_buffer[1..21].copy_from_slice(&destination.0);
    
    add_txn_field(
        txn_index,
        field_ids::SF_DESTINATION,
        dest_buffer.as_ptr(),
        dest_buffer.len()
    )
}

/// Adds the memos field to the transaction
/// 
/// # Arguments
/// * `txn_index` - Transaction builder index
/// 
/// # Returns
/// Result code from add_txn_field
unsafe fn add_memos_field(txn_index: i32) -> i32 {
    // Create memo buffers on stack
    let mut memo1_buffer = [0u8; buffer_sizes::MEMO_BUFFER];
    let memo1_len = build_memo(
        &mut memo1_buffer,
        Some(b"invoice"),
        Some(b"INV-2024-001"),
        Some(b"text/plain")
    );
    
    let mut memo2_buffer = [0u8; buffer_sizes::MEMO_BUFFER];
    let memo2_len = build_memo(
        &mut memo2_buffer,
        Some(b"note"),
        Some(b"Payment for consulting services"),
        Some(b"text/plain")
    );
    
    let mut memo3_buffer = [0u8; buffer_sizes::MEMO_BUFFER];
    let memo3_len = build_memo(
        &mut memo3_buffer,
        None,
        Some(b"Additional reference: Project Alpha"),
        None
    );
    
    // Prepare memo references for array building
    let memo_refs = [
        (&memo1_buffer[..], memo1_len),
        (&memo2_buffer[..], memo2_len),
        (&memo3_buffer[..], memo3_len),
    ];
    
    // Build the complete memos array
    let mut memos_array = [0u8; buffer_sizes::MEMOS_ARRAY];
    let memos_array_len = build_memos_array(&mut memos_array, &memo_refs);
    
    // Add to transaction
    add_txn_field(
        txn_index,
        field_ids::SF_MEMOS,
        memos_array.as_ptr(),
        memos_array_len
    )
}

// ============================================================================
// Main Entry Point
// ============================================================================

/// Main hook function that builds and emits a payment transaction with memos
/// 
/// This function:
/// 1. Retrieves the current contract call context
/// 2. Builds a payment transaction
/// 3. Adds amount, destination, and memos fields
/// 4. Emits the completed transaction
/// 
/// # Returns
/// - 0 on success
/// - Negative error code on failure
#[unsafe(no_mangle)]
pub extern "C" fn emit() -> i32 {
    // Get contract context
    let contract_call: ContractCall = get_current_contract_call();
    let account = contract_call.get_account().unwrap();

    // Initialize payment transaction
    let txn_index = 0;
    let build_result = unsafe { build_txn(TT_PAYMENT) };
    if build_result < 0 {
        return CUSTOM_ERROR_CODE;
    }

    // Build transaction fields
    unsafe {
        // Add amount field
        if add_amount_field(txn_index) < 0 {
            return CUSTOM_ERROR_CODE;
        }

        // Add destination field
        if add_destination_field(txn_index, &account) < 0 {
            return CUSTOM_ERROR_CODE;
        }

        // Add memos field
        if add_memos_field(txn_index) < 0 {
            return CUSTOM_ERROR_CODE;
        }

        // Emit the completed transaction
        let emission_result = emit_built_txn(txn_index);
        if emission_result < 0 {
            return emission_result;
        }
    }

    0 // Success
}