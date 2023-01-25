use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.h");
    println!("cargo:rerun-if-changed=src/lib.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");

    cxx_build::bridge("src/lib.rs")
        .file("src/lib.cpp")
        .flag("-std=c++17")
        .include(env::var("DEP_CERES_INCLUDE").unwrap())
        .include("/opt/homebrew/opt/eigen/include/eigen3")
        .include("/opt/homebrew/opt/glog/include")
        .include("/opt/homebrew/opt/gflags/include")
        .compile("ceres-solver-sys");

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
}
