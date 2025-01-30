# Rust_101

This repo is for learning purposes and based on the official [**Rust Documantations**](https://doc.rust-lang.org/book/) everything i learn will be included here for the future sake :)

## Starting with Rust

for installing rust based on your OS you can refere to the official website of github page. here i will explain the installation for **Linux**(specifically for **Ubuntu**)

```bash

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

this will download and install Rust on your machine.

after installation you will see a 'installation successfull' message! this means you are all setup to go.

### Create a new project

rust comes with its own package manager named `cargo`; whenever you install Rust it will get installed too.

for create a new project as i know 'till this moment there are two possible ways:

- #### 1. Using `cargo new`

if you does not yet created a directory for your project(or you did anyways!) you can use this command so it will create a new standard-structured **Rust** project. below is an example of it:

```bash

cargo new -[project-name]
```

for example:

```bash

cargo new rust-101
```

which will create a new project named `rust-101`

*also note that creating a new project named `test` is not possible due to the automation testings of rust itself.

- #### 2. Using `cargo init`

if you already are sure what the hell and where the hell you want to start coding this will become handy!
i'll create a `Cargo.toml` and a folder named `src` which contains a `main.rs` file.
`Cargo.toml` file represents the dependencies of the current project.

**Now, you have a rust project created on your machine!** 🎉

### Run the project

To run the rust you just need a simple command:

```bash

cargo run
```

automatically, it will run the `main.rs` file in the src folder but you can specify a name so it will compile and run that instead.(e.g.`cargo run [name]`)

after running the file, a new `Cargo.lock` file gets created that shows all the necessary dependencies for running this project, also a `target` folder will be created which i'm not totally aware of its purpose but, as soon as i figured that out it will be shown here.

### Add the DEPENDENCIES

ofcourse some dependencies will be required in the way sooo here is how you can add some:

```bash

cargo add [dependency-name]
```

And That's it! all done **!!!**

## Rustyntax

Rust syntax is actually strait foreward and you can master it easy peasy.(The hard parts are yet to reveal:) )

if you take a look at the `main.rs` file thats in the src folder you'll see something like this:

```rust
fn main(){
    println!("Hello, world!");
}
```

`fn` is the keyword for defining the functions. `main` is the functions name and `println!` is a macro function that prints the values onto the terminal.

*`!` is the macro operator which is used to call the macro functions.
