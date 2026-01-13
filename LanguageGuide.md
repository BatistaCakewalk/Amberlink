# 📘 Amberlink Language Guide

Welcome to Amberlink! This guide covers the syntax and features of the language. Amberlink is designed to be familiar to Java and C++ developers but with a lighter, script-like feel.

## 1. Basics

### Entry Point
Unlike Java, Amberlink does not require a `public static void main`. Code executes from top to bottom.

```java
print "Hello, World!"
```

### Comments
Use `//` for single-line comments.
```java
// This is a comment
int x = 5 // Comments can go here too
```

## 2. Variables & Types

Amberlink is statically typed, but supports type inference using `var`.

### Primitive Types
*   `int`: 32-bit signed integer.
*   `String`: Heap-allocated text string.
*   `void`: Used for functions that do not return a value.

### Declaration
You can declare variables using explicit types or `var`.

```java
int count = 10
String name = "Amber"
var explicit = 500
```

## 3. Control Flow

Parentheses around conditions are optional, making the code cleaner.

### If / Else
```java
if count > 5 {
    print "Count is big"
} else {
    print "Count is small"
}
```

### While Loop
```java
while count > 0 {
    print count
    count = count - 1
}
```

## 4. Functions

Functions are defined with a return type, a name, and parameters. They can be defined anywhere in the file.

```java
int add(int a, int b) {
    return a + b
}

void greet(String name) {
    print "Hello " + name
}

// Calling functions
int result = add(10, 20)
greet("Developer")
```

## 5. Arrays

Arrays are heap-allocated objects. You must specify the size when creating them.

```java
// Create an array of integers with size 5
int[] list = new int[5]

// Set values
list[0] = 42
list[1] = 100

// Access values
print list[0]
```

## 6. Classes & Objects (OOP)

Amberlink supports class-based Object-Oriented Programming.

### Defining a Class
Classes contain fields (variables) and methods (functions).

```java
class Counter {
    int value

    void increment() {
        // Use 'this' to access fields
        this.value = this.value + 1
    }
}
```

### Using Objects
Use the `new` keyword to create an instance.

```java
var c = new Counter()
c.value = 0
c.increment()
print c.value
```