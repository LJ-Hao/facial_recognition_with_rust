use std::process::Command;

fn main() {
    // Download Haar cascade file if it doesn't exist
    if !std::path::Path::new("haarcascade_frontalface_alt.xml").exists() {
        let output = Command::new("wget")
            .arg("-O")
            .arg("haarcascade_frontalface_alt.xml")
            .arg("https://raw.githubusercontent.com/opencv/opencv/master/data/haarcascades/haarcascade_frontalface_alt.xml")
            .output()
            .expect("Failed to download Haar cascade file");
            
        if !output.status.success() {
            panic!("Failed to download Haar cascade file: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
    
    println!("cargo:rerun-if-changed=build.rs");
}