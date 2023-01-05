use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "android" || target_os == "linux" {
        println!("cargo:rustc-link-lib=stdc++");
    } else {
        println!("cargo:rustc-link-lib=c++");
    }

    #[cfg(feature = "source")]
    {
        println!("cargo:rustc-link-lib=static=ceres");
    }
    #[cfg(not(feature = "source"))]
    {
        if let Err(pkg_config_error) = pkg_config::Config::new()
            // the earliest version tested, it may work with elder versions
            .range_version("1.14.0".."3.0.0")
            .probe("ceres")
        {
            dbg!(pkg_config_error);
            println!("cargo:rustc-link-lib=ceres");
        }
    }

    let bindings_builder = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let bindings_builder = if cfg!(feature = "source") {
        bindings_builder.clang_arg(format!("-I{}", env::var("DEP_CERES_INCLUDE").unwrap()))
    } else {
        bindings_builder
    };
    let bindings = bindings_builder
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
