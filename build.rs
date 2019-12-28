extern crate cc;

use std::env;
use std::path::Path;

fn main() {
    let java_home = env::var("JAVA_HOME").expect("$JAVA_HOME not found");
    let include_path = Path::new(&java_home).join("include");
    let mut build = cc::Build::new();
    if cfg!(target_os = "linux") {
        build.include(include_path.join("linux").as_path());
    } else if cfg!(target_os = "windows") {
        build.include(include_path.join("win32").as_path());
    }
    build.file("src/tools.c").file("src/callbacks.c")
        .include(include_path.as_path()).compile("libtools.a")
}