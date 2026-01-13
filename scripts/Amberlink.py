import sys
import compile
import os
import subprocess

def main():
    if len(sys.argv) < 2:
        print("Usage: python Amberlink.py [init|build <file>|install]")
        return

    command = sys.argv[1].lower()
    
    if command == "init":
        compile.build()
    elif command == "build":
        if len(sys.argv) < 3:
            print("Usage: python Amberlink.py build <file.amb>")
            return

        filename = sys.argv[2]

        # Locate the compiler binary
        root_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        bin_dir = os.path.join(root_dir, "bin")
        compiler_name = "ambc.exe" if os.name == "nt" else "ambc"
        compiler_path = os.path.join(bin_dir, compiler_name)

        if not os.path.exists(compiler_path):
            print(f"Error: Compiler not found at {compiler_path}")
            print("Run 'python Amberlink.py init' first.")
            return

        subprocess.run([compiler_path, filename])
    elif command == "install":
        print("This option isn't done yet")
    else:
        print(f"Unknown command: {command}")

if __name__ == "__main__":
    main()