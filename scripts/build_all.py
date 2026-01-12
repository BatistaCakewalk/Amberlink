# scripts/build_all.py
import os
import subprocess

def build():
    print("Building Amber-Core (Rust)...")
    subprocess.run(["cargo", "build"], cwd="../amber-core")

    print("Building Amber-VM (C++)...")
    if not os.path.exists("../amber-vm/build"):
        os.makedirs("../amber-vm/build")
    subprocess.run(["cmake", ".."], cwd="../amber-vm/build")
    subprocess.run(["make"], cwd="../amber-vm/build")

    print("Amberlink build complete.")

if __name__ == "__main__":
    build()
