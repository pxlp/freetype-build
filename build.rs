extern crate cmake;
extern crate pkg_config;

use cmake::Config;
use std::env;
use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::ErrorKind;

fn main() {
    let target = env::var("TARGET").unwrap();
    let generator = env::var("GENERATOR");

    // Compile assimp from source
    // Disable unnecessary stuff, it takes long enough to compile already
    let mut cfg = Config::new("freetype");
    cfg.define("CMAKE_BUILD_TYPE", "Release")
        .define("WITH_PNG", "OFF")
        .define("WITH_HarfBuzz", "OFF")
        .define("WITH_ZLIB", "OFF")
        .define("WITH_BZip2", "OFF");
    if let Ok(generator) = generator {
        cfg.generator(generator);
    }
    let dst = cfg.build();
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=freetype");

    if target.contains("windows-gnu") {
      // freetype-sys is currently configured to look for libfreetype-6 on windows:
      // https://github.com/PistonDevelopers/freetype-sys/blob/master/build.rs
      fs::copy(dst.join("lib").join("libfreetype.a"), dst.join("lib").join("libfreetype-6.a")).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
}
