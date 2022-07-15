use crate::global::init_global;
use v8::{self};

pub fn run(code: &str) {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
    let isolate = &mut v8::Isolate::new(Default::default());
    let scope = &mut v8::HandleScope::new(isolate);
    let global = v8::ObjectTemplate::new(scope);

    let context = v8::Context::new_from_template(scope, global);
    let scope = &mut v8::ContextScope::new(scope, context);
    init_global(scope);

    let mut scope = v8::TryCatch::new(scope);
    let script = v8::String::new(&mut scope, code).unwrap();

    let script = if let Some(script) = v8::Script::compile(&mut scope, script, None) {
        script
    } else {
        assert!(scope.has_caught());
        return;
    };

    if let Some(_result) = script.run(&mut scope) {
        /* do nothing
        println!(
            "{}",
            result
                .to_string(&mut scope)
                .unwrap()
                .to_rust_string_lossy(&mut scope)
        );
        */
    } else {
        assert!(scope.has_caught());
    }
}
