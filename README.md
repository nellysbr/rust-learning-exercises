# Rust Learning Exercises

A collection of exercises and examples that follow the official Rust Book chapters. This is not a production project, but rather a learning journey through Rust concepts as I study the language.

## 🎯 Purpose

This repository contains practical exercises and code examples that demonstrate various Rust concepts. Each exercise corresponds to different chapters and topics from the [Rust Book](https://doc.rust-lang.org/book/), serving as a hands-on learning companion.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system
- A terminal or command prompt

### Running the Project

1. **Clone or download this repository**
   ```bash
   git clone <repository-url>
   cd rust-learning-exercises
   ```

2. **Run the interactive menu**
   ```bash
   cargo run
   ```

3. **Choose an exercise**
   - The program will display a menu with available exercises
   - Enter the number corresponding to the exercise you want to run
   - Follow the on-screen instructions

## 📚 Available Exercises

### 1. Guess the Number
**Chapter**: Getting Started, Basic Syntax
- A simple number guessing game
- Demonstrates: variables, user input, control flow, error handling
- **What it does**: Generates a random number (1-10) and asks the user to guess it

### 2. Arrays
**Chapter**: Common Programming Concepts, Compound Types
- Working with arrays and indexing
- Demonstrates: array declaration, indexing, user input parsing, bounds checking
- **What it does**: Creates an array and allows the user to access elements by index

### 3. Tuples
**Chapter**: Common Programming Concepts, Compound Types
- Exploring tuple data structures
- Demonstrates: tuple declaration, destructuring, type annotations
- **What it does**: Creates a tuple with different data types and demonstrates destructuring

## 🏗️ Project Structure

```
src/
├── main.rs              # Main program with interactive menu
└── modules/             # Exercise modules organized by topic
    ├── guess_number.rs  # Number guessing game
    ├── arrays_module.rs # Array manipulation exercises
    └── tuples_module.rs # Tuple examples
```

## 📖 Learning Path

This project follows the Rust Book structure:

- ✅ **Chapter 1-2**: Getting Started, Programming a Guessing Game
- ✅ **Chapter 3**: Common Programming Concepts (Variables, Data Types)
- 🔄 **Chapter 4**: Understanding Ownership
- 🔄 **Chapter 5**: Using Structs to Structure Related Data
- 🔄 **Chapter 6**: Enums and Pattern Matching
- 🔄 **Chapter 7**: Managing Growing Projects with Packages, Crates, and Modules
- 🔄 **Chapter 8**: Common Collections
- 🔄 **Chapter 9**: Error Handling
- 🔄 **Chapter 10**: Generic Types, Traits, and Lifetimes
- 🔄 **Chapter 11**: Testing
- 🔄 **Chapter 12**: An I/O Project: Building a Command Line Program
- 🔄 **Chapter 13**: Functional Language Features: Iterators and Closures
- 🔄 **Chapter 14**: More about Cargo and Crates.io
- 🔄 **Chapter 15**: Smart Pointers
- 🔄 **Chapter 16**: Fearless Concurrency
- 🔄 **Chapter 17**: Object Oriented Programming Features
- 🔄 **Chapter 18**: Patterns and Matching
- 🔄 **Chapter 19**: Advanced Features
- 🔄 **Chapter 20**: Final Project: Building a Multithreaded Web Server

## 🛠️ Development

### Adding New Exercises

1. Create a new file in `src/modules/` (e.g., `ownership_examples.rs`)
2. Add your exercise functions with `pub` visibility
3. Update `src/main.rs`:
   - Add the module declaration in the `mod modules` block
   - Add the import statement
   - Add the menu option and match case

### Example: Adding a New Exercise

```rust
// src/modules/ownership_examples.rs
pub fn demonstrate_ownership() {
    println!("This demonstrates ownership concepts!");
}

// In src/main.rs, add:
// pub mod ownership_examples;
// use modules::ownership_examples::demonstrate_ownership;
// And in the match statement: 4 => demonstrate_ownership(),
```

## 📝 Notes

- This is a learning project, not production code
- Exercises are designed to be simple and educational
- Code includes comments explaining Rust concepts
- Each exercise focuses on specific language features

## 🤝 Contributing

This is a personal learning project, but suggestions and improvements are welcome! Feel free to:
- Report bugs or issues
- Suggest new exercises
- Improve existing code examples
- Add better explanations

## 📚 Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Reference](https://doc.rust-lang.org/reference/)
- [Rustlings](https://github.com/rust-lang/rustlings)

## 📄 License

This project is for educational purposes. Feel free to use and modify for your own learning journey.

---

**Happy Rusting! 🦀** 