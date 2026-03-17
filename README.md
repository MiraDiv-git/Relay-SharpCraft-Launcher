# Build dependencies
- [Rust](https://rust-lang.org/) - main language
- [mingw-w64](https://www.mingw-w64.org/) - for Microsoft Windows OS compilation
- [Wine](https://www.winehq.org/) - For test on Linux

# Build for Linux
### On Linux:
```bash
cargo run --package Relay-SharpCraft-Launcher --bin Relay-SharpCraft-Launcher
```

### On Windows:
```cmd
cargo build --target x86_64-unknown-linux-gnu
```

# Build for Windows
### On Linux:
1. Add target:
   rustup target add x86_64-pc-windows-gnu

2. Set linker (add to *~/.bashrc* or export before build):
   ```bash
   export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc
   ```

3. Build:
    ```bash
    cargo build --target x86_64-pc-windows-gnu
    ```

4. Run it using Wine

### On Windows:
```cmd
cargo run --package Relay-SharpCraft-Launcher --bin Relay-SharpCraft-Launcher
```