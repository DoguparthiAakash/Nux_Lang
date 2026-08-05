import os
import zipfile
import tarfile
from pathlib import Path

# The source directory containing the raw Nux source code
SOURCE_DIR = Path(r"e:\nux\Nux_Lang\nux\nux_oleg\nux_dist")
OUT_DIR = Path(r"e:\nux\Nux_Lang\Nux_Installers\release_archives")

def is_allowed_file(filename: str) -> bool:
    # Only allow pure Nux files
    return filename.endswith(".nux") or filename.endswith(".nuxc")

def pack():
    if not OUT_DIR.exists():
        OUT_DIR.mkdir(parents=True)
        
    zip_path = OUT_DIR / "nux_source_only.zip"
    tar_path = OUT_DIR / "nux_source_only.tar.gz"
    
    print(f"Packaging allowed Nux files from {SOURCE_DIR}...")
    
    with zipfile.ZipFile(zip_path, 'w', zipfile.ZIP_DEFLATED) as zipf, \
         tarfile.open(tar_path, "w:gz") as tarf:
        
        # We only want to include specific subdirectories or files that are pure Nux
        for root, dirs, files in os.walk(SOURCE_DIR):
            # Skip build directories or unrelated folders
            if "target" in root or ".cargo" in root or "scratch" in root:
                continue
                
            for file in files:
                if is_allowed_file(file):
                    full_path = Path(root) / file
                    rel_path = full_path.relative_to(SOURCE_DIR)
                    
                    zipf.write(full_path, arcname=rel_path)
                    tarf.add(full_path, arcname=rel_path)
                    print(f"Added: {rel_path}")

    print(f"\nSuccessfully created source archives:\n- {zip_path}\n- {tar_path}")

if __name__ == "__main__":
    pack()
