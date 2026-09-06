import subprocess
import sys

print("Running Nux...")
result = subprocess.run(["cargo", "run", "--bin", "nux", "run", "test_lumina.nux"], capture_output=True, text=True)
print(f"Exit code: {result.returncode}")
print("STDOUT:")
print(result.stdout)
print("STDERR:")
print(result.stderr)
