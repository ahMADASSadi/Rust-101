# Rust_101

This repo is for learning purposes and based on the official [**Rust Documantations**](https://doc.rust-lang.org/book/) everything i learn will be included here for the future sake :)

## Starting with Rust

for installing rust based on your OS you can refere to the official website of github page. here i will explain the installation for **Linux**(specifically for **Ubuntu**)

```bash

curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

this will download and install Rust on your machine.

after installation you will see a 'installation successfull' message! this means you are all setup to go.

## Create a new project

rust comes with its own package manager named `cargo`; whenever you install Rust it will get installed too.

for create a new project as i know 'till this moment there are two possible ways:

<details>

<summary>
Here is the possible ways
</summary>

### 1. Using `cargo new`

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

### 2. Using `cargo init`

if you already are sure what the hell and where the hell you want to start coding this will become handy!
i'll create a `Cargo.toml` and a folder named `src` which contains a `main.rs` file.
`Cargo.toml` file represents the dependencies of the current project.

</details>
