use std::{fs, path};
mod console;
mod bindings;
mod global;
mod settimeout;

pub fn run_script_code(content: &str) {
    // todo
    bindings::run(content);
}

pub fn run_script_file(filepath: &str) {
    match fs::read_to_string(path::Path::new(filepath)) {
        Ok(content) => {
            run_script_code(content.as_str());
        },
        _ => (),
    }
}
