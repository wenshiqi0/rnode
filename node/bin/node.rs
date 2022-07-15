use fes_core::run_script_file;
use std::{env, ffi, path};

fn main() {
    let args = env::args();
    args.for_each(move |arg| {
        match path::Path::new(arg.as_str())
            .extension()
            .and_then(ffi::OsStr::to_str)
        {
            Some(str) => {
                if str == "js" || str == "ts" {
                    run_script_file(arg.as_str());
                }
            }
            _ => (),
        }
    })
}
