use v8::{
  FunctionCallbackArguments, FunctionTemplate, HandleScope, Local, ObjectTemplate, ReturnValue,
};

#[allow(clippy::needless_pass_by_value)]
pub fn settimeout_callback(
  scope: &mut HandleScope,
  args: FunctionCallbackArguments,
  mut _retval: ReturnValue,
) {
  // todo
  
}
