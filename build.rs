use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=shaders/");
    if cfg!(target_os = "windows") {
        let status = Command::new("cmd")
            .args(["/C", "compile-shaders.bat"])
            .current_dir("./shaders")
            .status()
            .expect("Failed to compile shaders");

        if !status.success() {
            panic!("failed to compile shaders")
        }
    } else {
        panic!("unknown OS for auto shader compile. Update build.rs to compile or do it yourself.")
    }
}
