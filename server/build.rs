fn main() {
    println!("cargo:rerun-if-env-changed=CLOUD_BASE_URL");
    println!("cargo:rerun-if-env-changed=CLOUD_PUBLIC_KEY_PEM");
    if let Ok(url) = std::env::var("CLOUD_BASE_URL") {
        println!("cargo:rustc-env=CLOUD_BASE_URL={}", url);
    }
    if let Ok(pem) = std::env::var("CLOUD_PUBLIC_KEY_PEM") {
        println!("cargo:rustc-env=CLOUD_PUBLIC_KEY_PEM={}", pem);
    }
}
