extern crate libc;
use std::os::raw::{c_char};
use std::{ffi, ptr, slice};
use cardano::util::base58::*;
use std::ffi::CStr;
use std::ffi::CString;
  

#[no_mangle]
pub extern "C"
fn my_b58_encode(p: *const u8, p_size: u32, encoded: *mut c_char) -> i8
{
  let p_slice = unsafe { slice::from_raw_parts(p, p_size as usize) };

  let str = encode(p_slice);  /* CALL */

  let bytes = str.as_bytes();
  let cs = CString::new(bytes).unwrap();  // will fail if bytes has "gap" (null) in sequence
  unsafe {
    libc::strcpy( encoded, cs.as_ptr() );
  }

  return 1;
}

#[no_mangle]
pub extern "C"
fn my_b58_decode(encoded: *const c_char, p: *mut u8) -> i8
{
  let c_str: &CStr = unsafe { CStr::from_ptr(encoded) };
  let tmp_str: &str = c_str.to_str().unwrap();  // will fail if "gap" (null) in string, prevent in caller
  let str: String = tmp_str.to_owned();

  let vec = decode(&str);  /* CALL */

  if vec.is_err() {
    return -1; 
  }
 
  let mut tmp_ptr_u8 = p as *mut u8;

  let mut s: u32 = 0;

  for byte in vec.unwrap() {
    s=s+1;
    unsafe {
      *tmp_ptr_u8 = byte;
      tmp_ptr_u8 = tmp_ptr_u8.offset(1);
    }
  }

  return 1;
}

