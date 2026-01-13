# 📄 Amberlink: Technical Specification (v0.3.1)
Amberlink is a high-performance, multi-paradigm programming language designed to bridge the gap between the safety of Java and the raw power of C++. It utilizes a unique Dual-Backend approach, allowing code to run either on a dedicated Virtual Machine (AVM) or as a native binary.

1. System Architecture
Amberlink is split into two primary components to ensure memory safety and execution speed:
• The Brain (Amber-Core): Built in Rust. It handles the frontend tasks: Lexing, Parsing, Semantic Analysis, and Bytecode Generation. Using Rust ensures the compiler is immune to memory-related crashes.
• The Body (Amber-VM / AVM): Built in C++. A high-performance, stack-based virtual machine featuring a custom Mark-and-Sweep Garbage Collector and a lean object header system.
2. The Compilation Pipeline
Amberlink uses a Two-Pass Compilation strategy to solve the "Forward Declaration" problem found in older languages like C++.
1. Pass 1 (Discovery): The compiler scans the .amb source file to identify all function signatures, class definitions, and global variables. It populates the Symbol Table.
2. Pass 2 (Validation & Emission): The compiler verifies the logic and types. If successful, the Emitter generates a highly optimized binary file with the .amc (Amber Compiled) extension.
3. Language Design Philosophy
Amberlink is designed to be cleaner and more stable than Java, offering a familiar environment for existing developers while removing boilerplate.
*   **Familiar Syntax:** Uses C-style static typing and function definitions (`int add(int a, int b)`) to feel intuitive for developers coming from Java, C++, or C#.
*   **Script-like Simplicity:** No mandatory classes or `public static void main` boilerplate. Code executes from top to bottom, making it easy to write simple scripts and test ideas quickly.
*   **Newline-Based:** No semicolons required. The parser uses significant newlines to delimit statements, leading to cleaner code.
*   **Multi-Paradigm:** Supports both Object-Oriented Programming (OOP) via `class` and Functional Programming via standalone functions.

### Language Features Overview

**Variables & Types**
Amberlink is statically typed. Variables must be declared with a type (`int`, `String`, `void`).
```java
int count = 10
String message = "Hello World"
```

**Arrays**
Arrays are heap-allocated and garbage collected.
```java
int[] numbers = new int[5]
numbers[0] = 100
print numbers[0]
```

**Control Flow**
Standard `if/else` and `while` loops are supported, but parentheses are optional.
```java
if count < 20 {
    print "Small"
}

while count > 0 {
    count = count - 1
}
```

4. The AVM Bytecode Format (.amc)
The `.amc` binary format is designed to be compact and fast to load. It consists of a simple header followed by the raw bytecode instructions.

| Offset | Size (bytes) | Description                               |
|:-------|:-------------|:------------------------------------------|
| 0      | 4            | **Magic Number:** `AMBR` (0x41, 0x4D, 0x42, 0x52) |
| 4      | 2            | **Version:** A `u16` for the bytecode version.    |
| 6      | 4            | **Entry Point:** A `u32` offset to the `main` function (future use). |
| 10     | 4            | **Pool Count:** A `u32` count of strings in the constant pool. |
| 14     | Variable     | **Constant Pool:** Sequence of [Len(u32) + Bytes] for each string. |
| ...    | 4            | **Code Length:** A `u32` indicating the size of the code section. |
| ...    | N            | **Code Section:** The raw bytecode instructions.  |

5. Memory Management (The GC)
Unlike the heavy, unpredictable JVM Garbage Collector, the Amber-VM uses a lean and efficient Mark-and-Sweep collector.
*   **Object Header:** A minimal header per object stores metadata required by the GC, such as the "marked" flag.
*   **GC Strategy:** A classic Mark-and-Sweep algorithm. The "Mark" phase traverses all reachable objects from a set of roots (e.g., the stack, global variables), and the "Sweep" phase frees all unmarked objects.
*   **Future Work:** The design allows for future enhancements like a generational collector and manual GC hinting for performance-critical sections of code.


6. Project Structure
Amberlink/
├── amber-core/    # Rust: Lexer, Parser, Emitter
├── amber-vm/      # C++: Interpreter, GC, Loader
├── amber-native/  # 
├── bin/           # Final tool binaries (ambc, avm)
├── stdlib/        # Standard Amberlink libraries
└── scripts/       # Python build automation


7. Build and Run
Amberlink uses a Python script (`scripts/Amberlink.py`) as a unified interface for building the toolchain and compiling user code.

1. **Prerequisites:** Ensure Rust (Cargo), C++ (CMake), and Python 3 are installed.

2. **Initialize the Toolchain:**
   This compiles `amber-core` (Rust) and `amber-vm` (C++) and places binaries in `bin/`.
   ```bash
   python scripts/Amberlink.py init
   ```

3. **Build Code:**
   Compiles an `.amb` file to `.amc` bytecode using the built compiler.
   ```bash
   python scripts/Amberlink.py build main.amb
   ```

4. **Run Bytecode:**
   Execute the compiled file using the VM.
   ```bash
   ./bin/avm output.amc      # Linux/macOS
   .\bin\avm.exe output.amc  # Windows
   ```


8. Development Roadmap
The development of Amberlink is planned in distinct phases, each building upon the last to create a robust and feature-rich language.

### Phase 1: Core Language Features (The "Usable" Milestone)
This phase focuses on implementing the fundamental building blocks required to write simple, yet complete, programs.
1.  **Control Flow (Completed):** Implement `if/else` statements and `while` loops. This requires adding jump opcodes (`JUMP`, `JUMP_IF_FALSE`) to the VM and the corresponding logic in the compiler's Emitter.
2.  **Full Function Support (Completed):** Parse function bodies (`{...}`), parameters, and `return` statements. Implement a proper call stack in the VM with `CALL` and `RETURN` opcodes.
3.  **Scoping (Completed):** Differentiate between global and local (stack-allocated) variables to enable proper encapsulation and recursion.

### Phase 2: Data Structures & Memory (The "Robust" Milestone)
This phase moves beyond simple numbers and introduces the ability to manage more complex data.
1.  **String & Constant Pool (Completed):** Implement heap-allocated strings and a "constant pool" in the bytecode to efficiently store and reuse literals.
2.  **GC Root Scanning (Completed):** Fully implement the Garbage Collector by teaching it to scan the VM's stack and global variables for "roots" to determine which objects are still in use.
3.  **Arrays (Completed):** Introduce a built-in array/list type as the first user-creatable, heap-allocated collection.

### Phase 3: Object-Oriented Programming (The "Modern" Milestone)
This phase brings Amberlink closer to its goal of being a modern, Java-like language.
1.  **Classes & Instances:** Implement `class` definitions, fields, and object instantiation (`new MyClass()`).
2.  **Methods & `this`:** Allow methods to be defined within classes and implement the `this` keyword to refer to the current instance.
3.  **Inheritance:** Implement single inheritance for classes, allowing for code reuse and polymorphism.

### Phase 4: Ecosystem & Tooling (The "Mature" Milestone)
This phase focuses on building the tools and libraries that make a language productive and enjoyable to use.
1.  **Standard Library:** Create a foundational `stdlib` with modules for I/O, collections (e.g., HashMap), and string utilities.
2.  **Developer Tooling:**
    *   **Language Server (LSP):** Provide IDE support for features like autocompletion and error highlighting.
    *   **Debugger:** Create a tool to step through Amberlink code, inspect variables, and analyze the stack.
    *   **Package Manager:** Develop a system for sharing and managing third-party Amberlink libraries.
