use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct DstDirs {
    include: PathBuf,
    lib: PathBuf,
}

fn install_eigen(vendor_dir: &Path) -> DstDirs {
    let src_dir = {
        let mut dir = vendor_dir.to_owned();
        dir.push("eigen");
        dir
    };
    let dst = cmake::Config::new(src_dir).build();
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
    }
}

fn install_glog(vendor_dir: &Path) -> DstDirs {
    let src_dir = {
        let mut dir = vendor_dir.to_owned();
        dir.push("glog");
        dir
    };
    let dst = cmake::Config::new(src_dir)
        .profile("Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("WITH_GFLAGS", "OFF")
        .define("WITH_GTEST", "OFF")
        .define("WITH_PKGCONFIG", "OFF")
        .define("WITH_UNWIND", "OFF")
        .define("CMAKE_CXX_STANDARD", "14")
        .build();
    let dst_lib = {
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
        lib: dst_lib,
    }
}

fn install_ceres(vendor_dir: &Path) -> DstDirs {
    let src_dir = {
        let mut dir = vendor_dir.to_owned();
        dir.push("ceres-solver");
        dir
    };
    let dst = cmake::Config::new(src_dir)
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
        .define("BUILD_DOCUMENTATION", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .build();
    let dst_include = {
        let mut dir = dst.clone();
        dir.push("include");
        dir
    };
    let dst_lib = {
        let mut dir = dst;
        dir.push("lib");
        dir
    };
    DstDirs {
        include: dst_include,
        lib: dst_lib,
    }
}

fn main() {
    let vendor_dir: PathBuf = [env::var("CARGO_MANIFEST_DIR").unwrap(), "vendor".into()]
        .into_iter()
        .collect();

    let eigen_dirs = install_eigen(&vendor_dir);
    let glog_dirs = install_glog(&vendor_dir);
    let ceres_dirs = install_ceres(&vendor_dir);

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
