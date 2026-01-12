import sys
import compile

def main():
    if len(sys.argv) < 2:
        print("Usage: python Amberlink.py [compile|install]")
        return

    command = sys.argv[1].lower()
    
    if command == "compile":
        compile.build()
    elif command == "install":
        print("This option isn't done yet")
    else:
        print(f"Unknown command: {command}")

if __name__ == "__main__":
    main()