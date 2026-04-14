fn main() {
    println!("cargo:rerun-if-env-changed=CLOUD_PUBLIC_KEY_PEM");
    if let Ok(pem) = std::env::var("CLOUD_PUBLIC_KEY_PEM") {
        println!("cargo:rustc-env=CLOUD_PUBLIC_KEY_PEM={}", pem);
    }

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
