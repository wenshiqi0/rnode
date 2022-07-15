use v8::{
    FunctionCallbackArguments, FunctionTemplate, HandleScope, Local, ObjectTemplate, ReturnValue,
};

#[allow(clippy::needless_pass_by_value)]
fn log_callback(
    scope: &mut HandleScope,
    args: FunctionCallbackArguments,
    mut _retval: ReturnValue,
) {
    let message = args
        .get(0)
        .to_string(scope)
        .unwrap()
        .to_rust_string_lossy(scope);
    println!("{}", message);
}

pub fn init_console_tmpl(scope: &mut HandleScope<()>, tmpl: &mut Local<ObjectTemplate>) {
    tmpl.set(
        v8::String::new(scope, "log").unwrap().into(),
        FunctionTemplate::new(scope, log_callback).into(),
    );
}
