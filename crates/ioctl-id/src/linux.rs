//! Linux-specific definitions.
//!
//! On Linux, `ioctl(2)` identifiers are the size of C's `unsigned long` type,
//! equivalent to `u32` on 32-bit architectures and `u64` on 64-bit
//! architectures.
//!
//! The `ioctl(2)` identifier is divided into four fields. From most-significant to least-significant in the identifier:
//! * The direction of the `ioctl(2)` call.
//! * The size of the structure passed to the `ioctl(2)` call.
//! * A type identifier.
//! * A call identifier.
//!
//! The size of these fields varies by platform.

use core::{ffi::c_ulong, mem::size_of};

/// The number of bits available for `ioctl(2)` call identifiers.
///
/// This is the same for all architectures.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L23]
pub const IOC_NRBITS: usize = 8;

/// The number of bits available for `ioctl(2)` type identifiers.
///
/// This is the same for all architectures.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L24]
pub const IOC_TYPEBITS: usize = 8;

/// The number of bits used to represent the size of the argument to the `ioctl(2)` call.
///
/// For Alpha (not supported by Rust), MIPS, and PowerPC, this is 13 bits. For all other architectures, this is 14 bits.
///
/// For SPARC, this is 14 bits with an [overlap on the direction field][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L6-L8].
///
/// References:
/// * Alpha: [`arch/alpha/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L6]
/// * MIPS: [`arch/mips/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L14]
/// * PowerPC: [`arch/powerpc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/powerpc/include/uapi/asm/ioctl.h#L5]
/// * All others: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L32]
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "powerpc",
    target_arch = "powerpc64"
))]
pub const IOC_SIZEBITS: usize = 13;

/// The number of bits used to represent the size of the argument to the `ioctl(2)` call.
///
/// For Alpha (not supported by Rust), MIPS, and PowerPC, this is 13 bits. For all other architectures, this is 14 bits.
///
/// For SPARC, this is 14 bits with an
/// [overlap on the direction field][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L6-L8].
///
/// References:
/// * Alpha: [`arch/alpha/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L6]
/// * MIPS: [`arch/mips/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L14]
/// * PowerPC: [`arch/powerpc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/powerpc/include/uapi/asm/ioctl.h#L5]
/// * All others: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L32]
#[cfg(not(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "powerpc",
    target_arch = "powerpc64"
)))]
pub const IOC_SIZEBITS: usize = 14;

/// The number of bits used to represent the direction of the `ioctl(2)` call.
///
/// For Alpha (not supported by Rust), MIPS, PowerPC, and SPARC, this is 3 bits. For all other architectures, this is
/// 2 bits.
///
/// Note that on SPARC the direction field
/// [overlaps with the size field][[https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L6-L8].
///
/// References:
/// * Alpha: [`arch/alpha/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/alpha/include/uapi/asm/ioctl.h#L19]
/// * MIPS: [`arch/mips/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L15]
/// * PowerPC: [`arch/powerpc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L22]
/// * SPARC: [`arch/sparc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L22]
/// * All others: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L36]
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "powerpc",
    target_arch = "sparc",
    target_arch = "sparc64"
))]
pub const IOC_DIRBITS: usize = 3;

/// The number of bits used to represent the direction of the `ioctl(2)` call.
///
/// For Alpha (not supported by Rust), MIPS, PowerPC, and SPARC, this is 3 bits. For all other architectures, this is
/// 2 bits.
///
/// Note that on SPARC the direction field
/// [overlaps with the size field][[https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L6-L8].
///
/// References:
/// * Alpha: [`arch/alpha/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/alpha/include/uapi/asm/ioctl.h#L19]
/// * MIPS: [`arch/mips/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L15]
/// * PowerPC: [`arch/powerpc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L22]
/// * SPARC: [`arch/sparc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L22]
/// * All others: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L36]
#[cfg(not(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "powerpc",
    target_arch = "sparc",
    target_arch = "sparc64"
)))]
pub const IOC_DIRBITS: usize = 2;

/// Direction bit specifying no data is read or written.
///
/// For Alpha (not supported by Rust), MIPS, PowerPC, and SPARC, this is 1. For all other architectures, this is
/// 0 (no bits are set).
///
/// References:
/// * Alpha: [`arch/alpha/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/alpha/include/uapi/asm/ioctl.h#L36]
/// * MIPS: [`arch/mips/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L22]
/// * PowerPC: [`arch/powerpc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/powerpc/include/uapi/asm/ioctl.h#L8]
/// * Sparc: [`arch/sparc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L35]
/// * All others: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L35]
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "powerpc",
    target_arch = "sparc",
    target_arch = "sparc64"
))]
pub const IOC_NONE: c_ulong = 1;

/// Direction bit specifying no data is read or written.
///
/// For Alpha (not supported by Rust), MIPS, PowerPC, and SPARC, this is 1. For all other architectures, this is
/// 0 (no bits are set).
///
/// References:
/// * Alpha: [`arch/alpha/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/alpha/include/uapi/asm/ioctl.h#L36]
/// * MIPS: [`arch/mips/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L22]
/// * PowerPC: [`arch/powerpc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/powerpc/include/uapi/asm/ioctl.h#L8]
/// * Sparc: [`arch/sparc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L35]
/// * All others: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L58]
#[cfg(not(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "powerpc",
    target_arch = "sparc",
    target_arch = "sparc64"
)))]
pub const IOC_NONE: c_ulong = 0;

/// Direction bit specifying data is read.
///
/// This is 2 on all platforms (except PA-RISC, which is not supported by Rust).
///
/// References:
/// * Alpha: [`arch/alpha/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/alpha/include/uapi/asm/ioctl.h#L37]
/// * MIPS: [`arch/mips/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L23]
/// * PowerPC: [`arch/powerpc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/powerpc/include/uapi/asm/ioctl.h#L9]
/// * Sparc: [`arch/sparc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L36]
/// * All others: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L62]
pub const IOC_READ: c_ulong = 2;

/// Direction bit specifying data is written.
///
/// For Alpha (not supported by Rust), MIPS, PowerPC, and SPARC, this is 4. For PA-RISC (not supported by Rust), this
/// is 2. For all other architectures, this is 1.
///
/// References:
/// * Alpha: [`arch/alpha/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/alpha/include/uapi/asm/ioctl.h#L38]
/// * MIPS: [`arch/mips/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L24]
/// * PowerPC: [`arch/powerpc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/powerpc/include/uapi/asm/ioctl.h#L10]
/// * Sparc: [`arch/sparc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L37]
/// * All others: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L66]
#[cfg(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "powerpc",
    target_arch = "sparc",
    target_arch = "sparc64"
))]
pub const IOC_WRITE: c_ulong = 4;

/// Direction bit specifying data is written.
///
/// For Alpha (not supported by Rust), MIPS, PowerPC, and SPARC, this is 4. For PA-RISC (not supported by Rust), this
/// is 2. For all other architectures, this is 1.
///
/// References:
/// * Alpha: [`arch/alpha/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/alpha/include/uapi/asm/ioctl.h#L38]
/// * MIPS: [`arch/mips/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/mips/include/uapi/asm/ioctl.h#L24]
/// * PowerPC: [`arch/powerpc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/powerpc/include/uapi/asm/ioctl.h#L10]
/// * Sparc: [`arch/sparc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L37]
/// * All others: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L66]
#[cfg(not(any(
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "mips64r6",
    target_arch = "powerpc",
    target_arch = "sparc",
    target_arch = "sparc64"
)))]
pub const IOC_WRITE: c_ulong = 1;

/// Bitmask for valid `ioctl(2)` call identifiers.
///
/// This is applied to the unshifted call identifier.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L39]
pub const IOC_NRMASK: c_ulong = (1 << IOC_NRBITS) - 1;

/// Bitmask for valid `ioctl(2)` type identifiers.
///
/// This is applied to the unshifted type identifier.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L40]
pub const IOC_TYPEMASK: c_ulong = (1 << IOC_TYPEBITS) - 1;

/// Bitmask for valid `ioctl(2)` size values.
///
/// This is applied to the unshifted size value.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L41]
pub const IOC_SIZEMASK: c_ulong = (1 << IOC_SIZEBITS) - 1;

/// Bitmask for valid `ioctl(2)` direction values.
///
/// This is applied to the unshifted direction value.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L42]
pub const IOC_DIRMASK: c_ulong = (1 << IOC_DIRBITS) - 1;

/// The number of bits to shift `ioctl(2)` call identifiers by.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L44]
pub const IOC_NRSHIFT: usize = 0;

/// The number of bits to shift `ioctl(2)` type identifiers by.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L45]
pub const IOC_TYPESHIFT: usize = IOC_NRSHIFT + IOC_NRBITS;

/// The number of bits to shift `ioctl(2)` size values by.
///
/// For SPARC, this is decremented by 1 to account for the overlap with the direction field.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L46]
#[cfg(target_arch = "sparc")]
///
/// SPARC Reference: [`arch/sparc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L26-L32]
pub const IOC_SIZESHIFT: usize = IOC_TYPESHIFT + IOC_TYPEBITS - 1;

/// The number of bits to shift `ioctl(2)` size values by.
///
/// For SPARC, this is decremented by 1 to account for the overlap with the direction field.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L46]
///
/// SPARC Reference: [`arch/sparc/include/uapi/asm/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/arch/sparc/include/uapi/asm/ioctl.h#L26-L32]
#[cfg(not(target_arch = "sparc"))]
pub const IOC_SIZESHIFT: usize = IOC_TYPESHIFT + IOC_TYPEBITS;

/// The number of bits to shift `ioctl(2)` direction values by.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L47]
pub const IOC_DIRSHIFT: usize = IOC_SIZESHIFT + IOC_SIZEBITS;

/// Create an `ioctl(2)` identifier from a direction value, type identifier,
/// call identifier, and size value.
///
/// Note that the order of the parameters does not match their shift values. This matches the definition of the
/// `_IOC()` macro from the C headers.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L69-L73]
pub const fn ioc(dir: c_ulong, r#type: c_ulong, nr: c_ulong, size: c_ulong) -> c_ulong {
    (dir << IOC_DIRSHIFT) | (r#type << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

/// Create an `ioctl(2)` identifier for a call that passes no data.
///
/// This is the equivalent of the `_IO()` macro from the C headers.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L85]
pub const fn io(r#type: c_ulong, nr: c_ulong) -> c_ulong {
    ioc(IOC_NONE, r#type, nr, 0)
}

/// Create an `ioctl(2)` identifier for a call that reads data.
///
/// This is the equivalent of the `_IOR()` macro from the C headers. The type must be passed as a generic parameter
/// to this function.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L86]
pub const fn ior<T>(r#type: c_ulong, nr: c_ulong) -> c_ulong {
    ioc(IOC_READ, r#type, nr, size_of::<T>() as c_ulong)
}

/// Create an `ioctl(2)` identifier for a call that writes data.
///
/// This is the equivalent of the `_IOW()` macro from the C headers. The type must be passed as a generic parameter
/// to this function.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L86]
pub const fn iow<T>(r#type: c_ulong, nr: c_ulong) -> c_ulong {
    ioc(IOC_WRITE, r#type, nr, size_of::<T>() as c_ulong)
}

/// Create an `ioctl(2)` identifier for a call that reads and writes data.
///
/// This is the equivalent of the `_IOWR()` macro from the C headers. The type must be passed as a generic parameter
/// to this function.
///
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L87]
pub const fn iowr<T>(r#type: c_ulong, nr: c_ulong) -> c_ulong {
    ioc(IOC_READ | IOC_WRITE, r#type, nr, size_of::<T>() as c_ulong)
}

/// Decode the direction value from an `ioctl(2)` identifier.
/// 
/// This is the equivalent of the `_IOC_DIR()` macro from the C headers.
/// 
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L94]
pub const fn ioc_dir(id: c_ulong) -> c_ulong {
    (id >> IOC_DIRSHIFT) & IOC_DIRMASK
}

/// Decode the type identifier from an `ioctl(2)` identifier.
/// 
/// This is the equivalent of the `_IOC_TYPE()` macro from the C headers.
/// 
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L95]
pub const fn ioc_type(id: c_ulong) -> c_ulong {
    (id >> IOC_TYPESHIFT) & IOC_TYPEMASK
}

/// Decode the call identifier from an `ioctl(2)` identifier.
/// 
/// This is the equivalent of the `_IOC_NR()` macro from the C headers.
/// 
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L96]
pub const fn ioc_nr(id: c_ulong) -> c_ulong {
    (id >> IOC_NRSHIFT) & IOC_NRMASK
}

/// Decode the size identifier from an `ioctl(2)` identifier.
/// 
/// This is the equivalent of the `_IOC_SIZE()` macro from the C headers.
/// 
/// Reference: [`include/uapi/asm-generic/ioctl.h`][https://github.com/torvalds/linux/blob/7c16f0a4ed1ce7b0dd1c01fc012e5bde89fe7748/include/uapi/asm-generic/ioctl.h#L97]
pub const fn ioc_size(id: c_ulong) -> c_ulong {
    (id >> IOC_SIZESHIFT) & IOC_SIZEMASK
}
