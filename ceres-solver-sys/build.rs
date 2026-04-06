fn main() {
    println!("cargo:rerun-if-changed=src/lib.h");
    println!("cargo:rerun-if-changed=src/lib.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");

    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    let mut cc_build = cxx_build::bridge("src/lib.rs");
    cc_build.file("src/lib.cpp");
    // MSVC uses /std:c++17; GCC/Clang use -std=c++17
    if target_env == "msvc" {
        cc_build.flag("/std:c++17");
    } else {
        cc_build.flag("-std=c++17");
    }
    cc_build.define("GLOG_USE_GLOG_EXPORT", None);
    // On Windows, glog's abbreviated severity macros (ERROR, WARNING, INFO, …)
    // conflict with macros defined in <windows.h>.  This define tells glog not
    // to create those short aliases, which is all we need to break the conflict.
    if target_os == "windows" {
        cc_build.define("GLOG_NO_ABBREVIATED_SEVERITIES", None);
    }
    #[cfg(feature = "source")]
    {
        cc_build.includes(std::env::split_paths(
            &std::env::var("DEP_CERES_INCLUDE").unwrap(),
        ));

        println!("cargo:rustc-link-lib=static=glog");
        println!("cargo:rustc-link-lib=static=ceres");
    }
    #[cfg(not(feature = "source"))]
    {
        if target_os == "windows" {
            // On Windows, pkg-config is not standard.  Locate Ceres via the
            // vcpkg crate (reads VCPKG_ROOT + VCPKGRS_TRIPLET) or fall back to
            // explicit environment variables when vcpkg is unavailable:
            //   CERES_INCLUDE_DIR   — Ceres/Eigen headers
            //   EIGEN3_INCLUDE_DIR  — Eigen headers (if separate)
            //   CERES_LIB_DIR       — directory containing ceres.lib
            let found = vcpkg::find_package("ceres")
                .map(|lib| {
                    lib.include_paths.iter().for_each(|p| {
                        cc_build.include(p);
                        // In vcpkg, Eigen3 headers live under include/eigen3/ rather than
                        // include/ directly.  Ceres' own headers use `#include <Eigen/Core>`,
                        // so we must add the eigen3 sub-directory as an include root.
                        let eigen_sub = p.join("eigen3");
                        if eigen_sub.exists() {
                            cc_build.include(eigen_sub);
                        }
                    });
                    true
                })
                .unwrap_or(false);
            // Shlwapi is a Windows system library required by glog (for path utilities);
            // always link it regardless of whether ceres was found via vcpkg or env vars.
            println!("cargo:rustc-link-lib=dylib=shlwapi");
            if !found {
                if let Ok(dir) = std::env::var("CERES_INCLUDE_DIR") {
                    cc_build.include(&dir);
                }
                if let Ok(dir) = std::env::var("EIGEN3_INCLUDE_DIR") {
                    cc_build.include(&dir);
                }
                if let Ok(dir) = std::env::var("CERES_LIB_DIR") {
                    println!("cargo:rustc-link-search=native={}", dir);
                }
                // Ceres and its required dependencies when linking statically
                println!("cargo:rustc-link-lib=ceres");
                println!("cargo:rustc-link-lib=glog");
            }
        } else {
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
    }
    cc_build.compile("ceres-solver-sys");
}
