use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct DstDirs {
    include: PathBuf,
    lib: PathBuf,
    /// Directory containing the cmake package config (for find_package).
    cmake_dir: Option<PathBuf>,
}

/// On Windows with the Visual Studio cmake generator, Release-build libraries
/// land in `<prefix>/lib/Release/` rather than `<prefix>/lib/`.  Fall back
/// gracefully so the same code works on all platforms.
fn resolve_lib_dir(base: PathBuf) -> PathBuf {
    let release = base.join("Release");
    if release.exists() { release } else { base }
}

/// Create a `cmake::Config` for the given source directory with Windows-specific settings.
///
/// On Windows, the default Visual Studio generator is replaced with Ninja so
/// that cmake respects the MSVC compilers found on PATH via ilammy/msvc-dev-cmd.
/// Fortran is explicitly disabled to prevent cmake from finding GFortran
/// (e.g. from MinGW or Strawberry Perl) and failing its compiler test in the
/// MSVC + Ninja environment.
fn cmake_config(src: impl AsRef<Path>) -> cmake::Config {
    let mut config = cmake::Config::new(src);
    if env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        config.generator("Ninja");
        // Specify the MSVC compiler by name so cmake finds it via PATH (set by
        // ilammy/msvc-dev-cmd) rather than via the cmake crate's own registry
        // lookup, which may produce a path with spaces that confuses cmake.
        config.define("CMAKE_C_COMPILER", "cl");
        config.define("CMAKE_CXX_COMPILER", "cl");
        // Explicitly disable Fortran: cmake may find GFortran from MinGW/Strawberry
        // Perl tools and fail its compiler test in the MSVC + Ninja environment.
        config.define("CMAKE_Fortran_COMPILER", "CMAKE_Fortran_COMPILER-NOTFOUND");
    }
    config
}

fn install_eigen(vendor_dir: &Path) -> DstDirs {
    let src_dir = {
        let mut dir = vendor_dir.to_owned();
        dir.push("eigen");
        dir
    };
    let dst = cmake_config(&src_dir)
        // Disable all optional components that trigger Fortran language detection.
        // On Windows, cmake may find GFortran from Strawberry Perl and fail its
        // compiler test in the MSVC + Ninja environment.
        .define("BUILD_TESTING", "OFF")
        .define("EIGEN_BUILD_TESTING", "OFF")
        .define("EIGEN_BUILD_BLAS", "OFF")
        .define("EIGEN_BUILD_LAPACK", "OFF")
        .build();
    let dst_include = {
        let mut dir = dst.clone();
        dir.push("include");
        dir.push("eigen3");
        dir
    };
    let dst_lib = {
        let mut dir = dst;
        dir.push("lib");
        dir
    }; // probably doesn't exist because it is header only

    DstDirs {
        include: dst_include,
        lib: dst_lib,
        cmake_dir: None,
    }
}

fn install_glog(vendor_dir: &Path) -> DstDirs {
    let src_dir = {
        let mut dir = vendor_dir.to_owned();
        dir.push("glog");
        dir
    };
    let mut config = cmake_config(&src_dir);
    config
        .profile("Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("WITH_GFLAGS", "OFF")
        .define("WITH_GTEST", "OFF")
        .define("WITH_PKGCONFIG", "OFF")
        .define("WITH_UNWIND", "OFF")
        .define("CMAKE_CXX_STANDARD", "14");
    let dst = config.build();
    let raw_lib = {
        let mut dir = dst.clone();
        dir.push("lib");
        dir
    };
    let dst_lib = resolve_lib_dir(raw_lib.clone());
    let dst_include = {
        let mut dir = dst;
        dir.push("include");
        dir
    };
    // glog installs its cmake config to lib/cmake/glog/
    let cmake_dir = raw_lib.join("cmake").join("glog");
    DstDirs {
        include: dst_include,
        lib: dst_lib,
        cmake_dir: Some(cmake_dir),
    }
}

fn install_ceres(vendor_dir: &Path, glog_dirs: &DstDirs) -> DstDirs {
    let src_dir = {
        let mut dir = vendor_dir.to_owned();
        dir.push("ceres-solver");
        dir
    };
    let mut config = cmake_config(&src_dir);
    config
        .profile("Release")
        .pic(true)
        // Most of the options described here:
        // http://ceres-solver.org/installation.html#customizing-the-build
        .define("CUDA", "OFF")
        .define("LAPACK", "OFF")
        .define("EIGENSPARSE", "ON")
        .define("SUITESPARSE", "OFF")
        .define("ACCELERATESPARSE", "OFF")
        .define("EIGENMETIS", "OFF")
        .define("GFLAGS", "OFF")
        .define("MINIGLOG", "OFF")
        .define("SCHUR_SPECIALIZATIONS", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("EXPORT_BUILD_DIR", "OFF")
        .define("BUILD_BENCHMARKS", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_DOCUMENTATION", "OFF")
        .define("BUILD_EXAMPLES", "OFF");
    // Help cmake's find_package(glog) locate our vendored glog installation.
    if let Some(ref cmake_dir) = glog_dirs.cmake_dir {
        config.define("glog_DIR", cmake_dir);
    }
    let dst = config.build();
    let raw_lib = {
        let mut dir = dst.clone();
        dir.push("lib");
        dir
    };
    let dst_include = {
        let mut dir = dst;
        dir.push("include");
        dir
    };
    DstDirs {
        include: dst_include,
        lib: resolve_lib_dir(raw_lib),
        cmake_dir: None,
    }
}

fn main() {
    let vendor_dir: PathBuf = [env::var("CARGO_MANIFEST_DIR").unwrap(), "vendor".into()]
        .into_iter()
        .collect();

    let eigen_dirs = install_eigen(&vendor_dir);
    let glog_dirs = install_glog(&vendor_dir);
    let ceres_dirs = install_ceres(&vendor_dir, &glog_dirs);

    println!("cargo:rustc-link-search=native={}", glog_dirs.lib.display());
    println!(
        "cargo:rustc-link-search=native={}",
        ceres_dirs.lib.display()
    );
    println!(
        "cargo:include={}",
        env::join_paths([&eigen_dirs.include, &glog_dirs.include, &ceres_dirs.include,])
            .unwrap()
            .into_string()
            .unwrap()
    );
}
