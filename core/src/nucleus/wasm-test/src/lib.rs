extern crate holochain_wasm_utils;

use holochain_wasm_utils::*;

extern {
  fn debug(encoded_allocation_of_input: i32) -> i32;
}

//-------------------------------------------------------------------------------------------------
// HC DEBUG Function Call
//-------------------------------------------------------------------------------------------------

/// Call HC API DEBUG function with proper input struct: a string
/// return error code
fn hc_debug(mem_stack: &mut SinglePageStack, s: &str) {
  // Write input string on stack
  let allocation_of_input =  serialize(mem_stack, s);
  // Call WASMI-able DEBUG
  unsafe {
    debug(allocation_of_input.encode() as i32);
  }
  // Free input allocation and all allocations made inside print()
  mem_stack.deallocate(allocation_of_input).expect("deallocate failed");
}


//-------------------------------------------------------------------------------------------------
//  Generatable Dispatch function
//-------------------------------------------------------------------------------------------------

/// Function called by Holochain Instance
/// encoded_allocation_of_input : encoded memory offset and length of the memory allocation
/// holding input arguments
#[no_mangle]
pub extern "C" fn debug_hello_dispatch(encoded_allocation_of_input: usize) -> i32 {
  let mut mem_stack = SinglePageStack::new_from_encoded(encoded_allocation_of_input as u32);
  hc_debug(&mut mem_stack, "Hello world!");
  return 0;
}

/// Function called by Holochain Instance
/// encoded_allocation_of_input : encoded memory offset and length of the memory allocation
/// holding input arguments
#[no_mangle]
pub extern "C" fn debug_multiple_dispatch(encoded_allocation_of_input: usize) -> i32 {
  let mut mem_stack = SinglePageStack::new_from_encoded(encoded_allocation_of_input as u32);
  hc_debug(&mut mem_stack, "Hello");
  hc_debug(&mut mem_stack, "world");
  hc_debug(&mut mem_stack, "!");
  return 0;
}
