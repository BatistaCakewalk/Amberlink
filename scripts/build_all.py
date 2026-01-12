# scripts/build_all.py
import os
import subprocess
import sys
import shutil

# Color codes for clean 2026 terminal output
GREEN = "\033[92m"
BLUE = "\033[94m"
RESET = "\033[0m"

def run_command(command, cwd, description):
    print(f"{BLUE}[Amberlink Build]{RESET} {description}...")
    result = subprocess.run(command, cwd=cwd, shell=True)
    if result.returncode != 0:
        print(f"Error: {description} failed.")
        sys.exit(1)

def build():
    # 1. Setup paths
    root_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    bin_dir = os.path.join(root_dir, "bin")
    
    if not os.path.exists(bin_dir):
        os.makedirs(bin_dir)

    # 2. Build Amber-Core (Rust)
    core_dir = os.path.join(root_dir, "amber-core")
    run_command("cargo build --release", core_dir, "Compiling Rust Core")
    
    # Copy Rust binary to /bin
    rust_bin_name = "amber-core.exe" if os.name == "nt" else "amber-core"
    shutil.copy(os.path.join(core_dir, "target", "release", rust_bin_name), 
                os.path.join(bin_dir, "ambc"))

    # 3. Build Amber-VM (C++)
    vm_dir = os.path.join(root_dir, "amber-vm")
    build_dir = os.path.join(vm_dir, "build")
    
    if not os.path.exists(build_dir):
        os.makedirs(build_dir)
        
    run_command("cmake ..", build_dir, "Configuring C++ VM (CMake)")
    run_command("cmake --build . --config Release", build_dir, "Compiling C++ VM")

    # Copy C++ binary to /bin
    cpp_bin_name = "avm.exe" if os.name == "nt" else "avm"
    shutil.copy(os.path.join(build_dir, cpp_bin_name), 
                os.path.join(bin_dir, "avm"))

    print(f"\n{GREEN}Success!{RESET} Amberlink is ready.")
    print(f"Compiler: {os.path.join(bin_dir, 'ambc')}")
    print(f"VM:       {os.path.join(bin_dir, 'avm')}")

if __name__ == "__main__":
    build()
