#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
extern crate std;

use xrpl_std::host::emit_event;
use xrpl_std::core::data::codec::{STDataManager, STEventBuilder};


#[unsafe(no_mangle)]
pub extern "C" fn events() -> i32 {

    // Method 1: Using the builder pattern (most flexible)
    let result = STEventBuilder::new()
        .add_str("id", "1234")
        .add_u32("amount", 5000)
        .add_str("status", "completed")
        .emit("event1");
    
    if result.is_err() {
        return result.unwrap_err();
    }
    return 0; // Return success code
}