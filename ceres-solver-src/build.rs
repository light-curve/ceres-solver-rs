use std::env;
use std::ffi::OsString;
use std::fs;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};

fn copy_dir(from: impl AsRef<Path>, to: impl AsRef<Path>) -> std::io::Result<()> {
    let from = from.as_ref();
    let to = to.as_ref();

    if !from.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} is not a directory", from.display()),
        ));
    }
    if !to.exists() {
        fs::create_dir_all(to)?;
    }
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let from_path = entry.path();
        let to_path = to.join(from_path.file_name().unwrap());
        if from_path.is_dir() {
            copy_dir(from_path, to_path)?;
        } else {
            fs::copy(from_path, to_path)?;
        }
    }
    Ok(())
}

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
    let eigen_include_dir = {
        let mut dir = eigen_dir.clone();
        dir.push("Eigen");
        dir
    };
    let dst_eigen = {
        let mut dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        dir.push("include");
        dir
    };
    let dst_eigen_include = {
        let mut dir = dst_eigen.clone();
        dir.push("Eigen");
        dir
    };

    copy_dir(&eigen_include_dir, &dst_eigen_include).unwrap();

    let dst_ceres = cmake::Config::new(ceres_dir)
        .profile("Release")
        .pic(true)
        .env("EIGEN3_ROOT_DIR", &eigen_dir)
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
    let dst_ceres_include = {
        let mut dir = dst_ceres.clone();
        dir.push("include");
        dir
    };
    let dst_ceres_miniglog_include = {
        let mut dir = dst_ceres_include.clone();
        dir.push("ceres");
        dir.push("internal");
        dir.push("miniglog");
        dir
    };
    let dst_ceres_lib = {
        let mut dir = dst_ceres.clone();
        dir.push("lib");
        dir
    };

    println!("cargo:rustc-link-search=native={}", dst_ceres_lib.display());
    println!("cargo:rustc-link-lib=static=ceres");
    println!(
        "cargo:include={}",
        env::join_paths([&dst_ceres_include, &dst_ceres_miniglog_include, &dst_eigen])
            .unwrap()
            .into_string()
            .unwrap()
    );
}
