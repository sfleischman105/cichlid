use std::{env};

fn main() {
    let target = env::var("TARGET").unwrap();
    //let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    //let name = env::var("CARGO_PKG_NAME").unwrap();

    if target.starts_with("thumbv6m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv6m");
    } else if target.starts_with("thumbv7m-") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv7m");
    } else if target.starts_with("thumbv7em-") {
        println!("cargo:rustc-cfg=cortex_m");
        //println!("cargo:rustc-cfg=armv7m");
        println!("cargo:rustc-cfg=armv7em");
    } else if target.starts_with("thumbv8m") {
        println!("cargo:rustc-cfg=cortex_m");
        println!("cargo:rustc-cfg=armv8m");
    }

    if target.ends_with("-eabihf") {
        println!("cargo:rustc-cfg=has_fpu");
    }
}