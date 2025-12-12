fn main() {
    println!("cargo:rerun-if-changed=src/lib.h");
    println!("cargo:rerun-if-changed=src/lib.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let mut cc_build = cxx_build::bridge("src/lib.rs");
    cc_build.file("src/lib.cpp");
    cc_build.flag("-std=c++17");
    #[cfg(feature = "source")]
    {
        cc_build.define("GLOG_USE_GLOG_EXPORT", None);

        cc_build.includes(std::env::split_paths(
            &std::env::var("DEP_CERES_INCLUDE").unwrap(),
        ));

        println!("cargo:rustc-link-lib=static=glog");
        println!("cargo:rustc-link-lib=static=ceres");
    }
    #[cfg(not(feature = "source"))]
    {
        if let Ok(library) = pkg_config::Config::new()
            .range_version("3.3.4".."4.0.0")
            .probe("eigen3")
        {
            library.include_paths.into_iter().for_each(|path| {
                cc_build.include(path);
            });
        }
        match pkg_config::Config::new()
            .range_version("2.2.0".."3.0.0")
            .probe("ceres")
        {
            Ok(library) => library.include_paths.into_iter().for_each(|path| {
                cc_build.include(path);
            }),
            Err(_) => {
                println!("cargo:rustc-link-lib=dylib=ceres");
                // Ceres installed with Homebrew on Apple Silicon
                #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
                {
                    cc_build.include("/opt/homebrew/include");
                    cc_build.include("/opt/homebrew/include/eigen3");
                    println!("cargo:rustc-link-search=/opt/homebrew/lib");
                }
            }
        }
    }
    cc_build.compile("ceres-solver-sys");
}
