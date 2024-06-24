# Crate `ioctl-id`
Rust identifiers for `ioctl(2)` calls on Unix-like systems.

This brings definitions from the `ioctl.h` header into Rust as constant functions for use in
writing other Rust-based libraries for interacting with character and block devices.

## Platform Support

| Operating System  | Supported Platforms         | Untested Platforms      |
| ----------------- | --------------------------- | ----------------------- |
| FreeBSD           | (in progress)               |                         |
| Linux             | arm*, aarch64, i686, x86_64 | mips*, ppc*, sparc*     |
| MacOS             | (in progress)               |                         |
