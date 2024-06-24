//! Provides constant functions to compute for `ioctl(2)` identifiers.
//!
//! Currently, this only supports Linux. The long term goal is to support `ioctl` identifiers for other Unix-like
//! operating systems, including FreeBSD and macOS.
#![no_std]
#![warn(missing_docs)]

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(all(
    test,
    not(any(
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "mips64r6",
        target_arch = "powerpc",
        target_arch = "sparc",
        target_arch = "sparc64"
    ))
))]
mod ccompat_tests_generic;
