# TodoGuy
A simple terminal-based todo-list application built in Rust.

## Getting started
To use TodoGuy, you need Cargo and Rust installed on your machine.

To build TodoGuy, you'll use `cargo`.
After cloning the repository, in the TodoGuy's directory, run:
```console
$ cargo build --release
```
And to use it, run:
```console
$ ./target/release/todoguy # or target\release\todoguy.exe on Windows
```

## Commands

| **Command**   | **Description**                             |
| ------------- | ------------------------------------------- |
| `N`           | Create a new todo                           |
| `X`           | Delete the selected todo                    |
| `W`/`S`       | Move selection up/down                      |
| `A`/`D`       | Toggle checked/unchecked todo               |
| `C`           | Show the credits                            |
| `H`           | Show the help message                       |

---

> By marc-dantas