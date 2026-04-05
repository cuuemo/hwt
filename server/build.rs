fn main() {
    // Allow overriding cloud URL at build time via CLOUD_BASE_URL env var
    if let Ok(url) = std::env::var("CLOUD_BASE_URL") {
        println!("cargo:rustc-env=CLOUD_BASE_URL={}", url);
    }
}
