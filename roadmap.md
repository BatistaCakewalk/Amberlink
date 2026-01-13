# Amberlink Development Roadmap

The development of Amberlink is planned in distinct phases, each building upon the last to create a robust and feature-rich language.

## Phase 1: Core Language Features (The "Usable" Milestone)
This phase focuses on implementing the fundamental building blocks required to write simple, yet complete, programs.

- [x] **Control Flow:** Implement `if/else` statements and `while` loops. This requires adding jump opcodes (`JUMP`, `JUMP_IF_FALSE`) to the VM and the corresponding logic in the compiler's Emitter.
- [x] **Full Function Support:** Parse function bodies (`{...}`), parameters, and `return` statements. Implement a proper call stack in the VM with `CALL` and `RETURN` opcodes.
- [x] **Scoping:** Differentiate between global and local (stack-allocated) variables to enable proper encapsulation and recursion.

## Phase 2: Data Structures & Memory (The "Robust" Milestone)
This phase moves beyond simple numbers and introduces the ability to manage more complex data.

- [x] **String & Constant Pool:** Implement heap-allocated strings and a "constant pool" in the bytecode to efficiently store and reuse literals.
- [x] **GC Root Scanning:** Fully implement the Garbage Collector by teaching it to scan the VM's stack and global variables for "roots" to determine which objects are still in use.
- [x] **Arrays:** Introduce a built-in array/list type as the first user-creatable, heap-allocated collection.

## Phase 3: Object-Oriented Programming (The "Modern" Milestone)
This phase brings Amberlink closer to its goal of being a modern, Java-like language.

- [x] **Classes & Instances:** Implement `class` definitions, fields, and object instantiation (`new MyClass()`).
- [x] **Methods & `this`:** Allow methods to be defined within classes and implement the `this` keyword to refer to the current instance.
- [ ] **Inheritance:** Implement single inheritance for classes, allowing for code reuse and polymorphism.

## Phase 4: Ecosystem & Tooling (The "Mature" Milestone)
This phase focuses on building the tools and libraries that make a language productive and enjoyable to use.

- [ ] **Standard Library:** Create a foundational `stdlib` with modules for I/O, collections (e.g., HashMap), and string utilities.
- [ ] **Amberlink Archive Format:** Develop a custom container format (e.g., `.ama`) to package compiled bytecode and resources, mirroring the functionality of Java JARs.
- [ ] **Module System:** Implement `import` statements and file linking to organize code across multiple files and support the package manager.
- [ ] **Developer Tooling:**
    - **Language Server (LSP):** Provide IDE support for features like autocompletion and error highlighting.
    - **Debugger:** Create a tool to step through Amberlink code, inspect variables, and analyze the stack.
    - **Package Manager:** Develop a system for sharing and managing third-party Amberlink libraries.

## Phase 5: Performance & Interoperability (The "Power" Milestone)
This phase unlocks the raw performance and flexibility promised by the "Dual-Backend" architecture.

- [ ] **Native Compilation:** Implement the second backend to compile Amberlink source code directly to native machine code (via LLVM or C transpilation).
- [ ] **Foreign Function Interface (FFI):** Enable Amberlink code to call C functions, allowing access to existing system libraries.
- [ ] **JIT Compilation:** Implement a Just-In-Time compiler within the AVM to compile hot bytecode paths to machine code at runtime.

## Phase 6: Advanced Language Features (The "Expressive" Milestone)
This phase introduces sophisticated features for complex application development.

- [ ] **Generics:** Implement type parameters (e.g., `List<T>`) to allow for type-safe, reusable data structures.
- [ ] **Exception Handling:** Introduce `try`, `catch`, and `throw` keywords for robust error management.
- [ ] **Pattern Matching:** Add support for advanced control flow structures like `match` or `switch` expressions.

## Phase 7: JVM Integration (The "Universal" Milestone)
This phase expands Amberlink's reach by integrating with the vast Java ecosystem.

- [ ] **Java Interop:** Enable seamless interoperability, allowing Amberlink projects to import Java classes and libraries, and vice versa.
- [ ] **Hybrid Project Support:** Enable the compiler to handle mixed source directories of Amberlink and Java files, allowing direct usage of Java code within Amberlink projects.