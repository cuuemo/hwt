fn main() {
    println!("cargo:rerun-if-env-changed=CLOUD_PUBLIC_KEY_PEM");
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let pem_path = std::path::Path::new(&out_dir).join("cloud_public_key.pem");
    let pem = std::env::var("CLOUD_PUBLIC_KEY_PEM").unwrap_or_default();
    std::fs::write(&pem_path, pem).expect("failed to write cloud_public_key.pem");

    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest(
            r#"
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
              <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                <security>
                  <requestedPrivileges>
                    <requestedExecutionLevel level="requireAdministrator"/>
                  </requestedPrivileges>
                </security>
              </trustInfo>
            </assembly>
        "#,
        );
        res.compile().unwrap();
    }
}
