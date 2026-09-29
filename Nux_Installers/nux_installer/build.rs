use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let payload_path = Path::new(&manifest_dir).join("payload.zip");

    // Tell Cargo to re-run this build script if payload.zip changes
    println!("cargo:rerun-if-changed={}", payload_path.display());

    // Tell Cargo to re-run if the payload directory changes (so the python script knows to pack)
    println!("cargo:rerun-if-changed=../windows/payload");

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        // Request administrator privileges
        res.set_manifest(r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#);
        res.compile().unwrap();
    }
}
