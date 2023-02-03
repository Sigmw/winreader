<p align="center">
    <img width="330" height="200" src="./assets/winreader.png" alt="Gopher"">
</p>
<h1 align="center"> Winreader </h1>

Winreader is a process memory reader for Windows, implemented and developed in the Rust language, using the official [Microsoft Rust API](https://crates.io/crates/winapi).

Winreader is able to return information such as: process path, memory address, buffer address and size of allocated memory.

### Usage

All information you need to use winreader is the PID (Process Identifier) ​​of the program that will be read, you can use Windows Task Manager or PowerShell's tasklist command.

![](assets/tasklist.gif)

```
winreader --help
winreader: Read buffer memory in processes on Windows

Usage: winreader.exe [PID]

Arguments:
  [PID]  Program Process Identifier

Options:
  -h, --help     Print help
  -V, --version  Print version
```

A file called `WINREADER-DUMP.txt` will be created inside the directory where the `winreader.exe` is located, with all buffer memory information.

### TODO

| Tool      | Description|
|-----------|------------|
| Buffer    | Still incomplete, having erratic output in the text file. |
| GUI       | Intent to move software from CLI to GUI, using WinAPI.       |

PRs are very welcome! 

### Install

The purpose and main focus of winreader is only for Windows NT operating system, to install it you will need rust:
- Rust ([rustup](https://rustup.rs))

Or you can just download the release executable:
- [Winreader Release](https://github.com/Sigmw/winreader/releases)

Also the project is on crates.io:
```
cargo install winreader
```

#### License

Winreader is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

-----
