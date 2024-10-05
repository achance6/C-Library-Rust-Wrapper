## Project Setup
Some of this won't matter. Hyperspecific for now.

### WSL
- See https://www.jetbrains.com/help/clion/how-to-use-wsl-development-environment-in-product.html#wsl-general
- distro: Ubuntu (default)
- Run these commands after install: ```sudo apt-get update``` ```sudo apt-get install cmake gcc clang gdb build-essential```
- Install zlib dev: ```sudo apt install zlib1g-dev```

### Other
- IDE: CLion + JetBrains Rust plugin. Using CLion on Windows and remoting to WSL

- C Compiler: Clang

- linker: ld (I think...didn't have to download this specifically)

- Rust Compiler: Rustc

- Build Tool: Cargo

### Testing
Once you are setup, you should be able to run ```cargo test``` with a successful outcome
