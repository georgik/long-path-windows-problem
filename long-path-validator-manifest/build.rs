use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let rc_path = PathBuf::from(&out_dir).join("long_path_manifest.rc");
    let res_path = PathBuf::from(&out_dir).join("long_path_manifest.res");

    // Debug print to ensure build.rs is invoked
    println!("Building long_path_manifest.rc and long_path_manifest.res");

    // Write the resource script
    fs::write(
        &rc_path,
        r#"
1 24 "long_path_manifest.xml"
"#,
    ).unwrap();

    // Compile the resource file using rc.exe
    let output = Command::new("rc.exe")
        .arg("/fo")
        .arg(&res_path)
        .arg(&rc_path)
        .output()
        .expect("Failed to execute rc.exe");

    if !output.status.success() {
        panic!("rc.exe failed: {:?}", output);
    }

    // Link the .res file using link.exe
    println!("cargo:rustc-link-arg-bin=long-path-validator-manifest={}", res_path.to_str().unwrap());

    // Ensure the build script reruns if the manifest file changes
    println!("cargo:rerun-if-changed=long_path_manifest.xml");
    println!("cargo:rerun-if-changed=build.rs");
}
