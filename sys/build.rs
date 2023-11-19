#![allow(clippy::uninlined_format_args)]

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    // copy the flite files to OUT_DIR
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let flite_root = out.join("flite/");
    if !flite_root.exists() {
        std::fs::create_dir_all(&flite_root).unwrap();
        fs_extra::dir::copy("./flite", &out, &Default::default()).unwrap_or_else(|e| {
            panic!(
                "Failed to copy flite source into {}: {}",
                flite_root.display(),
                e
            );
        });
    }

    // make bindings
    if env::var("FLITE_DONT_GENERATE_BINDINGS").is_ok() {
        let _: u64 = std::fs::copy("src/bindings.rs", out.join("bindings.rs"))
            .expect("Failed to copy bindings.rs");
    } else {
        // we need to make bindings with bindgen
        let bindings = bindgen::Builder::default()
            .header("wrapper.h")
            .clang_arg("-Iflite/")
            .clang_arg("-Iflite/src")
            .clang_arg("-Iflite/include")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate();

        match bindings {
            Ok(b) => {
                let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
                b.write_to_file(out_path.join("bindings.rs"))
                    .expect("Couldn't write bindings!");
            }
            Err(e) => {
                println!("cargo:warning=Unable to generate bindings: {}", e);
                println!("cargo:warning=Using bundled bindings.rs, which may be out of date");
                // copy src/bindings.rs to OUT_DIR
                std::fs::copy("src/bindings.rs", out.join("bindings.rs"))
                    .expect("Unable to copy bindings.rs");
            }
        }
    };

    // stop on docs.rs
    if env::var("DOCS_RS").is_ok() {
        return;
    }

    // ** build flite **

    // configure flite
    let mut configure = std::process::Command::new("./configure");
    configure.current_dir(&flite_root);
    let child = configure.spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) => panic!("Failed to spawn configure: {}", e),
    };
    let status = child.wait();
    let status = match status {
        Ok(s) => s,
        Err(e) => panic!("Failed to wait for configure: {}", e),
    };
    if !status.success() {
        panic!("Failed to configure flite: exit code {}", status);
    }

    // build flite
    let mut make = std::process::Command::new("make");
    make.current_dir(&flite_root).env(
        "MAKEFLAGS",
        env::var("CARGO_MAKEFLAGS").expect("CARGO_MAKEFLAGS not set: cargo bug?"),
    );
    make.arg("-j1"); // race condition in makefile (wtf)
    make.arg("all");
    let child = make.spawn();
    let mut child = match child {
        Ok(c) => c,
        Err(e) => panic!("Failed to spawn make: {}", e),
    };
    let status = child.wait();
    let status = match status {
        Ok(s) => s,
        Err(e) => panic!("Failed to wait for make: {}", e),
    };
    if !status.success() {
        panic!("Failed to build flite: exit code {}", status);
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    // copy the flite shared library to OUT_DIR
    let flite_lib = flite_root.join(format!(
        "build/{}-{}-{}/lib/libflite.a",
        target_arch, target_os, target_env
    ));
    let flite_lib_out = out.join("libflite.a");
    let _: u64 = std::fs::copy(flite_lib, flite_lib_out).expect("Failed to copy flite library");

    // link the flite library
    println!("cargo:rustc-link-search=native={}", out.display());
    println!("cargo:rustc-link-lib=static=flite");
    // link with ALSA
    println!("cargo:rustc-link-lib=asound");

    // and now copy + link for every enabled feature
    let features = env::vars()
        .filter_map(|(k, _)| {
            if k.starts_with("CARGO_FEATURE_") {
                Some(k[14..].to_lowercase())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    for feature in features {
        let flite_lib = flite_root.join(format!(
            "build/{}-{}-{}/lib/libflite_{}.a",
            target_arch, target_os, target_env, feature
        ));
        let flite_lib_out = out.join(format!("libflite_{}.a", feature));
        println!("Copying flite library for feature {}", feature);
        println!(
            "Got paths: {} -> {}",
            flite_lib.display(),
            flite_lib_out.display()
        );
        let _: u64 = std::fs::copy(flite_lib, flite_lib_out).expect("Failed to copy flite library");
        println!("cargo:rustc-link-search=native={}", out.display());
        println!("cargo:rustc-link-lib=static=flite_{}", feature);
    }
}
