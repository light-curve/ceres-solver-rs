use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn apply_patches(src_dir: &Path, patches_dir: &Path) {
    if !patches_dir.exists() {
        return;
    }

    let mut patches: Vec<_> = fs::read_dir(patches_dir)
        .expect("Failed to read patches directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|ext| ext == "patch")
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .collect();

    patches.sort();

    for patch_path in patches {
        println!("cargo:warning=Applying patch: {}", patch_path.display());
        
        // Try to apply the patch with --forward, which skips already applied patches
        let output = Command::new("patch")
            .arg("-p1")
            .arg("-d")
            .arg(src_dir)
            .arg("-i")
            .arg(&patch_path)
            .arg("--forward")
            .arg("--reject-file=-")
            .output()
            .expect("Failed to execute patch command");

        // Check if patch failed (exit code != 0) and it's not because patch was already applied
        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            let combined = format!("{}{}", stdout, stderr);
            
            // If the error is NOT "Reversed (or previously applied) patch detected", then panic
            if !combined.contains("Reversed (or previously applied) patch detected") {
                eprintln!("Patch stdout: {}", stdout);
                eprintln!("Patch stderr: {}", stderr);
                panic!("Failed to apply patch: {}", patch_path.display());
            }
            // Otherwise, the patch was already applied, which is fine
            println!("cargo:warning=Patch already applied: {}", patch_path.display());
        }
    }
}

fn install_ceres(vendor_dir: &Path, manifest_dir: &Path) -> DstDirs {
    let src_dir = {
        let mut dir = vendor_dir.to_owned();
        dir.push("ceres-solver");
        dir
    };

    let patches_dir = {
        let mut dir = manifest_dir.to_owned();
        dir.push("patches");
        dir
    };

    apply_patches(&src_dir, &patches_dir);

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
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let vendor_dir: PathBuf = [manifest_dir.to_str().unwrap(), "vendor"]
        .into_iter()
        .collect();

    let eigen_dirs = install_eigen(&vendor_dir);
    let glog_dirs = install_glog(&vendor_dir);
    let ceres_dirs = install_ceres(&vendor_dir, &manifest_dir);

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
