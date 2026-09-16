use std::process::Command;
use std::path::Path;

pub struct Linker;

impl Linker {
    pub fn link_bare_metal<P1: AsRef<Path>, P2: AsRef<Path>>(
        obj_file: P1,
        out_bin: P2,
        linker_script: Option<P1>,
    ) -> Result<(), String> {
        let mut cmd = Command::new("ld");
        
        if let Some(script) = linker_script {
            cmd.arg("-T").arg(script.as_ref());
        }

        cmd.arg("-o").arg(out_bin.as_ref());
        cmd.arg(obj_file.as_ref());

        let output = cmd.output().map_err(|e| format!("Linker execution failed: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Linker error: {}", stderr));
        }

        Ok(())
    }
}
