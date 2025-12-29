# Clarigggz OS

A microkernel-based operating system for the Clarigggz AR platform.

## Architecture

- **Kernel**: Core microkernel (MPL-2.0). Handles scheduling, memory management, and IPC.
- **HAL**: Hardware Abstraction Layer (Apache-2.0). Defines traits for hardware components.
- **Drivers**: Userspace/Kernel drivers (Apache-2.0). Implements HAL traits for specific hardware.
- **Userland**: System services and applications (Apache-2.0).
  - `lib-abi`: System call interface.
  - `init`: The first userspace process.
  - `vfs`: Virtual File System service.

## Building

Requires Rust 2024 and the `riscv64gc-unknown-none-elf` target.

```bash
cargo build --target riscv64gc-unknown-none-elf
```

## Licensing

- Kernel: [MPL-2.0](LICENSE-MPL)
- HAL, Drivers, Userland, Tooling: [Apache-2.0](LICENSE-APACHE)
- SDK: [MIT](LICENSE-MIT)
