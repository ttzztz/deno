// Copyright 2018-2024 the Deno authors. All rights reserved. MIT license.

use deno_core::error::AnyError;
use deno_core::OpState;

use std::mem::size_of;
use std::os::raw::c_char;
use std::os::raw::c_short;
use std::path::Path;

mod call;
mod callback;
mod dlfcn;
mod ir;
mod repr;
mod r#static;
mod symbol;
mod turbocall;

use call::op_ffi_call_nonblocking;
use call::op_ffi_call_ptr;
use call::op_ffi_call_ptr_nonblocking;
use callback::op_ffi_unsafe_callback_close;
use callback::op_ffi_unsafe_callback_create;
use callback::op_ffi_unsafe_callback_ref;
use dlfcn::op_ffi_load;
use dlfcn::ForeignFunction;
use r#static::op_ffi_get_static;
use repr::*;
use symbol::NativeType;
use symbol::Symbol;

#[cfg(not(target_pointer_width = "64"))]
compile_error!("platform not supported");

const _: () = {
  assert!(size_of::<c_char>() == 1);
  assert!(size_of::<c_short>() == 2);
  assert!(size_of::<*const ()>() == 8);
};

pub const UNSTABLE_FEATURE_NAME: &str = "ffi";

fn check_unstable(state: &OpState, api_name: &str) {
  // TODO(bartlomieju): replace with `state.feature_checker.check_or_exit`
  // once we phase out `check_or_exit_with_legacy_fallback`
  state
    .feature_checker
    .check_or_exit_with_legacy_fallback(UNSTABLE_FEATURE_NAME, api_name)
}

pub trait FfiPermissions {
  fn check_partial(&mut self, path: Option<&Path>) -> Result<(), AnyError>;
}

deno_core::extension!(deno_ffi,
  deps = [ deno_web ],
  parameters = [P: FfiPermissions],
  ops = [
    op_ffi_load<P>,
    op_ffi_get_static,
    op_ffi_call_nonblocking,
    op_ffi_call_ptr<P>,
    op_ffi_call_ptr_nonblocking<P>,
    op_ffi_ptr_create<P>,
    op_ffi_ptr_equals<P>,
    op_ffi_ptr_of<P>,
    op_ffi_ptr_of_exact<P>,
    op_ffi_ptr_offset<P>,
    op_ffi_ptr_value<P>,
    op_ffi_get_buf<P>,
    op_ffi_buf_copy_into<P>,
    op_ffi_cstr_read<P>,
    op_ffi_read_bool<P>,
    op_ffi_read_u8<P>,
    op_ffi_read_i8<P>,
    op_ffi_read_u16<P>,
    op_ffi_read_i16<P>,
    op_ffi_read_u32<P>,
    op_ffi_read_i32<P>,
    op_ffi_read_u64<P>,
    op_ffi_read_i64<P>,
    op_ffi_read_f32<P>,
    op_ffi_read_f64<P>,
    op_ffi_read_ptr<P>,
    op_ffi_unsafe_callback_create<P>,
    op_ffi_unsafe_callback_close,
    op_ffi_unsafe_callback_ref,
  ],
  esm = [ "00_ffi.js" ],
);
