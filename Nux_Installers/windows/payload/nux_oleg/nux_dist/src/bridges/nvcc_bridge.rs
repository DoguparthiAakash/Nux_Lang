use std::process::Command;
use std::path::Path;

pub struct NvccBridge;

impl NvccBridge {
    pub fn compile_ptx_to_cubin<P1: AsRef<Path>, P2: AsRef<Path>>(
        ptx_file: P1,
        out_cubin: P2,
    ) -> Result<(), String> {
        let mut cmd = Command::new("nvcc");
        
        cmd.arg("-cubin");
        cmd.arg("-arch=sm_80"); // RTX 4060 target compatibility
        cmd.arg("-o").arg(out_cubin.as_ref());
        cmd.arg(ptx_file.as_ref());

        let output = cmd.output().map_err(|e| format!("nvcc execution failed: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("nvcc error: {}", stderr));
        }

        Ok(())
    }

    pub fn compile_ptx_to_fatbin<P1: AsRef<Path>, P2: AsRef<Path>>(
        ptx_file: P1,
        out_fatbin: P2,
    ) -> Result<(), String> {
        let mut cmd = Command::new("nvcc");
        
        cmd.arg("-fatbin");
        cmd.arg("-arch=sm_80");
        cmd.arg("-o").arg(out_fatbin.as_ref());
        cmd.arg(ptx_file.as_ref());

        let output = cmd.output().map_err(|e| format!("nvcc execution failed: {}", e))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("nvcc error: {}", stderr));
        }

        Ok(())
    }
}
