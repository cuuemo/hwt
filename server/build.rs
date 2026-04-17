fn main() {
    println!("cargo:rerun-if-env-changed=CLOUD_BASE_URL");
    println!("cargo:rerun-if-env-changed=CLOUD_PUBLIC_KEY_PEM");
    if let Ok(url) = std::env::var("CLOUD_BASE_URL") {
        println!("cargo:rustc-env=CLOUD_BASE_URL={}", url);
    }
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let pem_path = std::path::Path::new(&out_dir).join("cloud_public_key.pem");
    let pem = std::env::var("CLOUD_PUBLIC_KEY_PEM").unwrap_or_default();
    std::fs::write(&pem_path, pem).expect("failed to write cloud_public_key.pem");
}
