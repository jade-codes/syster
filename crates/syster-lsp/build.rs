use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output directory (target/debug or target/release)
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let stdlib_src = Path::new("../../crates/syster-base/sysml.library");
    let stdlib_dest = target_dir.join("sysml.library");

    // Only copy if source exists
    if stdlib_src.exists() {
        // Remove old copy if it exists
        if stdlib_dest.exists() {
            let _ = fs::remove_dir_all(&stdlib_dest);
        }

        // Copy the directory
        if let Err(e) = copy_dir_all(stdlib_src, &stdlib_dest) {
            eprintln!("Warning: Failed to copy stdlib: {e}");
        }
    }

    // Tell cargo to rerun if stdlib changes
    println!("cargo:rerun-if-changed=../../crates/syster-base/sysml.library");
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
