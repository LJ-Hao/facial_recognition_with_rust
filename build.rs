use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Print build information
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=OPENCV_VERSION");
    println!("cargo:rerun-if-env-changed=OPENCV_DISABLE_PROBES");
    
    // Set environment variables to help with OpenCV detection
    if env::var("OPENCV_DISABLE_PROBES").is_err() {
        println!("cargo:rustc-env=OPENCV_DISABLE_PROBES=true");
    }
    
    if env::var("OPENCV_VERSION").is_err() {
        println!("cargo:rustc-env=OPENCV_VERSION=4.6.0");
    }
    
    // Try to find OpenCV installation
    if let Ok(opencv_dir) = env::var("OpenCV_DIR") {
        println!("cargo:rustc-link-search=native={}", opencv_dir);
    }
    
    // Try to find OpenCV using pkg-config
    match pkg_config::probe_library("opencv4") {
        Ok(_) => {
            println!("Found OpenCV via pkg-config");
        }
        Err(_) => {
            // Try alternative method
            if let Ok(lib_dir) = env::var("LD_LIBRARY_PATH") {
                for path in lib_dir.split(":") {
                    let opencv_lib_path = Path::new(path).join("libopencv_core.so");
                    if opencv_lib_path.exists() {
                        println!("cargo:rustc-link-search=native={}", path);
                        break;
                    }
                }
            }
        }
    }
    
    // Create directories if they don't exist
    fs::create_dir_all("database").unwrap();
    fs::create_dir_all("photos").unwrap();
    
    println!("Build script completed successfully");
}