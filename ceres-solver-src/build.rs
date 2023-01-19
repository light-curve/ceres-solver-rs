use std::env;
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;

fn main() {
    let vendor_dir: PathBuf = [env::var("CARGO_MANIFEST_DIR").unwrap(), "vendor".into()]
        .into_iter()
        .collect();
    let ceres_dir = {
        let mut dir = vendor_dir.clone();
        dir.push("ceres-solver");
        dir
    };
    let eigen_dir = {
        let mut dir = vendor_dir;
        dir.push("eigen");
        dir
    };
    let eigen_cmake_dir = {
        let mut dir = eigen_dir.clone();
        dir.push("cmake");
        #[allow(unused_mut)]
        let mut os_str: OsString = dir.into();
        // Cmake would like to have all paths to be separated by "/" on all platforms.
        #[cfg(target_os = "windows")]
        {
            let forward_slash = AsRef::<std::ffi::OsStr>::as_ref("/")
                .encode_wide()
                .next()
                .unwrap();
            let backward_slash = AsRef::<std::ffi::OsStr>::as_ref(r#"\"#)
                .encode_wide()
                .next()
                .unwrap();
            let v: Vec<_> = os_str
                .encode_wide()
                .map(|char| {
                    if char == backward_slash {
                        forward_slash
                    } else {
                        char
                    }
                })
                .collect();
            os_str = OsString::from_wide(&v);
        }
        os_str
    };

    let dst = cmake::Config::new(ceres_dir)
        .profile("Release")
        .env("EIGEN3_ROOT_DIR", eigen_dir)
        .define("CMAKE_MODULE_PATH", eigen_cmake_dir)
        // Most of the options described here:
        // http://ceres-solver.org/installation.html#customizing-the-build
        .define("CUDA", "OFF")
        .define("LAPACK", "OFF")
        .define("EIGENSPARSE", "ON")
        .define("SUITESPARSE", "OFF")
        .define("CXSPARSE", "OFF")
        .define("ACCELERATESPARSE", "OFF")
        .define("GFLAGS", "OFF")
        .define("MINIGLOG", "ON")
        .define("SCHUR_SPECIALIZATIONS", "OFF")
        // .define("CERES_THREADING_MODEL", "CXX_THREADS") // doesn't have obvious defaults
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("EXPORT_BUILD_DIR", "OFF")
        .define("BUILD_BENCHMARKS", "OFF")
        .define("BUILD_DOCUMENTATION", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define("MSVC_USE_STATIC_CRT", "OFF") // ??? we use default
        .define("LIB_SUFFIX", "")
        .build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=ceres");
    println!("cargo:include={}/include", dst.display());
}
