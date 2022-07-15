use crate::console::init_console_tmpl;
use v8::{HandleScope, Local, ObjectTemplate};

pub fn init_global(scope: &mut HandleScope) {
    let context = scope.get_current_context();
    let global = context.global(scope);

    // create console object template
    let mut console_tmpl = ObjectTemplate::new(scope);
    init_console_tmpl(scope, &mut console_tmpl);

    let console_class = Local::<ObjectTemplate>::new(scope, console_tmpl);
    let console_instance = console_class.new_instance(scope).unwrap();
    let object_key = v8::String::new(scope, "console");

    global.set(
        scope.into(),
        object_key.unwrap().into(),
        console_instance.into(),
    );
}
