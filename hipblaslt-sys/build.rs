use std::{
    env::{var, VarError},
    path::PathBuf,
};

const DEFAULT_HIP_PATH: &str = "/opt/rocm/hip";

fn try_get_path_from_var(v: &str) -> Option<PathBuf> {
    match var(v) {
        Ok(v) => {
            // Check that the path exists.
            let pb = PathBuf::from(&v);
            if !pb.exists() {
                panic!(
                    "{}: {v} is set to '{}', but that path doesn't exist",
                    env!("CARGO_PKG_NAME"),
                    pb.display()
                );
            }
            Some(pb)
        }
        Err(VarError::NotPresent) => None,
        Err(e @ VarError::NotUnicode(_)) => panic!("{}: {v}: {}", env!("CARGO_PKG_NAME"), e),
    }
}

fn main() {
    // Link to hipblaslt. Search for HIPBLASLT_PATH and HIP_BLASLT_PATH.
    // If neither are defined, then use HIP_PATH, failing that, a default 
    // hipblaslt directory.

    println!("cargo:rerun-if-env-changed=HIP_PATH");
    println!("cargo:rerun-if-env-changed=HIP_BLASLT_PATH");
    println!("cargo:rerun-if-env-changed=HIPBLASLT_PATH");

    // Try to get HIP_PATH.
    let hip_path =
        try_get_path_from_var("HIP_PATH").unwrap_or_else(|| PathBuf::from(DEFAULT_HIP_PATH));

    println!(
        "cargo:warning={}: Using '{}' as HIP_PATH",
        env!("CARGO_PKG_NAME"),
        hip_path.display()
    );

    let hip_blaslt_path = {
        // Now try to get the hipblaslt path.
        let mut hip_blaslt_path = None;
        for hip_blaslt_path_var in ["HIP_BLASLT_PATH", "HIPBLASLT_PATH"] {
            if let Some(v) = try_get_path_from_var(hip_blaslt_path_var) {
                println!(
                    "cargo:warning={}: Using {hip_blaslt_path_var}",
                    env!("CARGO_PKG_NAME")
                );
                hip_blaslt_path = Some(v);
                break;
            }
            println!(
                "cargo:warning={}: {hip_blaslt_path_var} wasn't defined",
                env!("CARGO_PKG_NAME")
            );
        }

        let hip_blaslt_path = match hip_blaslt_path {
            Some(p) => p,
            None => {
                let pb = hip_path.join("..").join("hipblaslt");
                println!("cargo:warning={}: Assuming HIP_PATH is related to hipblaslt; checking if '{}' exists", env!("CARGO_PKG_NAME"), pb.display());
                pb.canonicalize().unwrap()
            }
        };

        hip_blaslt_path
    };

    println!(
        "cargo:warning={}: Using '{}' as HIP_BLASLT_PATH",
        env!("CARGO_PKG_NAME"),
        hip_blaslt_path.display()
    );

    let hip_blaslt_lib = hip_blaslt_path.join("lib");
    println!("cargo:rustc-link-search=native={}", hip_blaslt_lib.display());
    println!("cargo:rustc-link-lib=dylib=hipblaslt");

    #[cfg(feature = "bindgen")]
    {
        // The bindgen::Builder is the main entry point to bindgen, and lets you
        // build up options for the resulting bindings.
        println!("cargo:rerun-if-changed=wrapper.h");
        let bindings = bindgen::Builder::default()
            .raw_line("#![allow(non_camel_case_types)]")
            .raw_line("#![allow(non_upper_case_globals)]")
            .raw_line("#![allow(non_snake_case)]")
            // The input header we would like to generate bindings for.
            .header("wrapper.h")
            // Enable C++ mode
            .clang_arg("-xc++")
            .clang_arg("-std=c++14")
            // Define HIP platform for AMD/ROCm
            .clang_arg("-D__HIP_PLATFORM_AMD__")
            .clang_arg(format!("-I{}", hip_blaslt_path.join("include").display()))
            .clang_arg(format!("-I{}", hip_blaslt_path.join("include").join("hipblaslt").display()))
            .clang_arg(format!("-I{}", hip_path.join("include").display()))
            // Also include regular hipblas headers as hipblaslt often depends on them
            .clang_arg({
                let hipblas_include = hip_path.join("..").join("hipblas").join("include");
                if hipblas_include.exists() {
                    format!("-I{}", hipblas_include.display())
                } else {
                    // Fallback: try to find hipblas in the same parent directory
                    format!("-I{}", hip_blaslt_path.join("..").join("hipblas").join("include").display())
                }
            })
            // Only generate bindings for hipblaslt and hip functions/types
            .allowlist_function("hipblasLt.*")
            .allowlist_function("hip.*")
            .allowlist_type("hipblasLt.*")
            .allowlist_type("hip.*")
            .allowlist_var("hipblasLt.*")
            .allowlist_var("hip.*")
            .allowlist_var("HIP.*")
            // Block std library bindings
            .blocklist_type("std.*")
            .blocklist_function("std.*")
            .rustified_non_exhaustive_enum("hip.*")
            .generate_block(false)
            .size_t_is_usize(true)
            .ctypes_prefix("::libc")
            .derive_default(true)
            .derive_eq(true)
            .derive_ord(true)
            .derive_hash(true)
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");
        bindings
            .write_to_file("src/bindings.rs")
            .expect("Couldn't write bindings!");
    }
}