# Rust Basics

## Rust Housekeeping

- `rustup` is a command line tool for managing Rust versions and associated tools. It is installed by default with the Rust toolchain installer.

- To check the version of `rustup` installed on your system, run the following command:

    ```rust
    rustc --version
    ```
- Update Rust toolchain to the latest stable version:

    ```rust
    rustup update
    ```

## Hello World

### Basic Hello World program

```rust
fn main() {
    println!("Hello, world!");
}
```

Executing the program:

```rust
rustc helloworld.rs
.\helloworld.exe
```

### Hello World Anatomy

- The `main` function is the entry point of the program. The execution of the program starts from the `main` function.

    ```rust
    fn main() {
    }
    ```

- The body of the function has the code:

    ```rust
    println!("Hello, world!");
    ```

    - `println!` is a macro that prints text to the console. If it were a function, it would be called as `println` (without the `!`).
    - Rust style is to indent with four spaces, not a tab.
    - "Hello, world!" is a string that is passed as an argument to the `println!` macro.
    - The expression is ended with a semicolon `;`.

### Compilation

```pwsh
rustc helloworld.rs
```

In Windows, this will create an executable file named `helloworld.exe`. Additionally, it also creates a `helloworld.pdb` file, which is a debug file.
In Linux, this will create an executable file named `helloworld`.

### Running the program

Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. 

```pwsh
.\helloworld.exe
```

If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.

## Cargo Fundamentals

### What is Cargo?

Cargo is Rust's build system and package manager. 
Most Rustaceans use Cargo to manage their Rust projects because Cargo handles a lot of tasks for you, such as:

- building your code
- downloading the libraries your code depends on 
- building those libraries.

```rust
cargo --version
```

### Creating a project with Cargo

```rust
cargo new hello_cargo
cd hello_cargo
```

Go into the `hello_cargo`` directory and list the files.

You’ll see that Cargo has generated two files and one directory for us: 
- a Cargo.toml file and 
- a `src` directory with a `main.rs` file inside.

It has also initialized a new Git repository along with a `.gitignore` file. 
Git files won’t be generated if you run cargo new within an existing Git repository.

The toml (Tom's Obvious, Minimal Language) file is Cargo’s configuration file.

- The first line, [package] is a section heading that indicates that the following statements are configuring a package.
- The last line, [dependencies] is a section heading for configuring dependencies.

In Rust, packages of code are referred to as `crates`.

### Building and Running a Cargo Project

```rust
cargo build
```

This command creates an executable file in `target/debug/hello_cargo` (or `target\debug\hello_cargo.exe` on Windows) rather than in your current directory. 
Because the default build is a debug build, Cargo puts the binary in a directory named debug.

```rust
.\target\debug\hello_cargo.exe
```

Running `cargo build` causes Cargo to create a new file named `Cargo.lock` in the same directory as the `Cargo.toml` file.
The `Cargo.lock` file keeps track of the exact versions of dependencies in your project.

We can also use `cargo run` to **compile and run** the code in one command.

```rust
cargo run
```


Let’s recap what we’ve learned so far about Cargo:

- We can create a project using `cargo new`.
- We can build a project using `cargo build`.
- We can build and run a project in one step using `cargo run`.
- We can build a project without producing a binary to check for errors using `cargo check`.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the `target/debug directory`.

### Building for Release

This command will create an executable in `target/release` instead of `target/debug`. 
The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. 
This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. 

If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in `target/release`.

```rust
cargo build --release
```
