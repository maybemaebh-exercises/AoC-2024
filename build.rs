// build.rs
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("profile_info.rs");

    let profile = env::var("PROFILE").unwrap();
    let content = format!("pub const BUILD_PROFILE: &str = \"{}\";", profile);

    fs::write(&dest_path, content).unwrap();

    // Tell Cargo when to re-run this build script.
    // If you only care about PROFILE, this is sufficient.
    // If you were using other environment variables, you'd add them here.
    println!("cargo:rerun-if-env-changed=PROFILE");
}