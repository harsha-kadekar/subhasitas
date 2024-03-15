# Overview

A simple service that can provide random subhasitas (maxims). Sanskrit: सुभाषित, subhāṣita.

# Getting Started

## Environment setup

**Dependencies**

- Rust Compiler
    - Visual Studio C++ Build tools (on Windows)
    - gcc (on Linux)
- Docker Engine

Instructions to install the dependencies can be found in the links below. They have also been reproduced in this doc
for ease of access and potential deviations from official docs (example: winget not mentioned in Rust docs).

[Install Rust](https://www.rust-lang.org/tools/install)

[Install Linux on Windows with WSL](https://learn.microsoft.com/en-us/windows/wsl/install)

[Moving WSL2 to alternate drive](https://superuser.com/questions/1550622/move-wsl2-file-system-to-another-drive)

**Windows instructions**

```
winget install --id Docker.DockerDesktop
winget install --id Rustlang.Rustup

# Restart terminal after that and verify this works:
rustc --version

# If Visual Studio is already installed, make sure it is up to date and reboot if necessary.
```

**Windows Subsystem for Linux (WSL) instructions**

1. Install a Linux distribution if one is not already installed
    - To install to default location, which maybe the system drive:
    ```
    # Choose the distro to install using:
    wsl --list --online

    # Install the desired distro
    wsl --install -d Ubuntu-22.04
    ```

    - To move it to an alternate drive:
    ```
    wsl --shutdown

    # Note: Adjust target paths according to your needs

    mkdir D:\wsl
    pushd D:\wsl
    wsl --export Ubuntu-22.04 .\ubuntu2204.tar

    wsl --unregister Ubuntu-22.04

    wsl --import Ubuntu-22.04 D:\wsl\ D:\wsl\ubuntu2204.tar

    # Ensure the distro is registered
    wsl --list

    # To update username from the default 'root' to something else:
    pushd $env:UserProfile\AppData\Local\Microsoft\WindowsApps
    .\ubuntu2204.exe config --default-user <username>
    ```

2. Install the dependencies inside the distro
    ```
    sudo apt update
    sudo apt install gcc -y

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

    # Restart terminal after that and verify this works:
    rustc --version
    ```

## Build
```
# Change dir into project root
cargo build
```

## Unit Tests
```
# Run all tests
cargo test

# Run a single test
cargo test hello_test
```

## Run
```
cargo run
```
