fn main() {
    #[cfg(debug_assertions)]
    println!(
        "cargo:rustc-env=CSS_STYLESHEET={}",
        "styles/output.css"
    );
    #[cfg(not(debug_assertions))]
    _production_css();
    #[cfg(feature = "reload")]
    _warm_reload_styles();
}
fn _production_css() {
    let tailwind_output = std::process::Command::new("tailwindcss")
        .arg("-i")
        .arg("public/styles/input.css")
        .arg("-o")
        .arg("public/styles/prod.css")
        .arg("--minify")
        .output()
        .expect("Failed to run tailwindcss command");

    if !tailwind_output.status.success() {
        panic!("Tailwind CSS command failed");
    }
    let new_path = _rename_prod_file();
    println!("cargo:rustc-env=CSS_STYLESHEET={}", new_path);
}
fn _rename_prod_file() -> String {
    let output_hash = std::fs::metadata("public/styles/prod.css")
        .expect("Failed to read output.css metadata")
        .modified()
        .expect("Failed to get modified time")
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Failed to get duration since epoch")
        .as_secs();
    let new_path = format!("public/styles/output-{}.css", output_hash);
    std::fs::rename("public/styles/prod.css", new_path).expect("Failed to rename output.css");
    format!("styles/output-{}.css", output_hash)
}
fn _warm_reload_styles() {
    _cleanup();
    let tailwind_output = std::process::Command::new("tailwindcss")
        .arg("-i")
        .arg("public/styles/input.css")
        .arg("-o")
        .arg("public/styles/output.css")
        .output()
        .expect("Failed to run tailwindcss command");

    if !tailwind_output.status.success() {
        panic!("Tailwind CSS command failed");
    }
    //get the hash of the outpt file
    let new_path = _rename_output_file();
    //  change file name to include the hash
    // set CSS_STYLESHEET env varable at compile time to the new output
    println!("cargo:rustc-env=CSS_STYLESHEET={}", new_path);
}
fn _rename_output_file() -> String {
    let output_hash = std::fs::metadata("public/styles/output.css")
        .expect("Failed to read output.css metadata")
        .modified()
        .expect("Failed to get modified time")
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Failed to get duration since epoch")
        .as_secs();
    let new_path = format!("public/styles/output-{}.css", output_hash);
    std::fs::rename("public/styles/output.css", new_path).expect("Failed to rename output.css");
    format!("styles/output-{}.css", output_hash)
}
fn _cleanup() {
    // Delete old output file if it exists
    let styles_dir = std::fs::read_dir("public/styles").expect("Failed to read styles directory");
    for entry in styles_dir {
        let entry = entry.expect("Failed to read entry");
        let file_name = entry.file_name();
        if file_name.to_str().unwrap().starts_with("output") {
            let _ = std::fs::remove_file(entry.path()).expect("Failed to remove file");
        }
    }
}
