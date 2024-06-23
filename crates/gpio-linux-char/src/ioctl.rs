use std::mem::size_of;

/// The number of bits available for the ioctl id. This is bits 0-7 of the 32-bit ioctl id.
pub(crate) const IOC_NRBITS: usize = 8;

/// The number of bits available for the ioctl type. This is bits 8-15 of the 32-bit ioctl id.
pub(crate) const IOC_TYPEBITS: usize = 8;

/// Bits used to represent the size of the argument to the ioctl.
pub(crate) const IOC_SIZEBITS: usize = 14;

/// Bits used to represent the direction of the ioctl.
pub(crate) const IOC_DIRBITS: usize = 2;

pub(crate) const IOC_NRMASK: u32 = (1 << IOC_NRBITS) - 1;
pub(crate) const IOC_TYPEMASK: u32 = (1 << IOC_TYPEBITS) - 1;
pub(crate) const IOC_SIZEMASK: u32 = (1 << IOC_SIZEBITS) - 1;
pub(crate) const IOC_DIRMASK: u32 = (1 << IOC_DIRBITS) - 1;

pub(crate) const IOC_NRSHIFT: usize = 0;
pub(crate) const IOC_TYPESHIFT: usize = IOC_NRSHIFT + IOC_NRBITS;
pub(crate) const IOC_SIZESHIFT: usize = IOC_TYPESHIFT + IOC_TYPEBITS;
pub(crate) const IOC_DIRSHIFT: usize = IOC_SIZESHIFT + IOC_SIZEBITS;

/// No direction
pub(crate) const IOC_NONE: u32 = 0;

/// Write
pub(crate) const IOC_WRITE: u32 = 1;

/// Read
pub(crate) const IOC_READ: u32 = 2;

/// Create an ioctl number.
pub(crate) const fn ioc(dir: u32, r#type: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT) | (r#type << IOC_TYPESHIFT) | (nr << IOC_NRSHIFT) | (size << IOC_SIZESHIFT)
}

pub(crate) const fn ioc_typecheck<T>() -> u32 {
    size_of::<T>() as u32
}

pub(crate) const fn io(r#type: u32, nr: u32) -> u32 {
    ioc(IOC_NONE, r#type, nr, 0)
}

pub(crate) const fn ior<T>(r#type: u32, nr: u32) -> u32 {
    ioc(IOC_READ, r#type, nr, ioc_typecheck::<T>())
}

pub(crate) const fn iow<T>(r#type: u32, nr: u32) -> u32 {
    ioc(IOC_WRITE, r#type, nr, ioc_typecheck::<T>())
}

pub(crate) const fn iowr<T>(r#type: u32, nr: u32) -> u32 {
    ioc(IOC_READ | IOC_WRITE, r#type, nr, ioc_typecheck::<T>())
}

/// Decode ioctl code to its direction.
pub(crate) const fn ioc_dir(nr: u32) -> u32 {
    (nr >> IOC_DIRSHIFT) & IOC_DIRMASK
}

/// Decode ioctl code to its type.
pub(crate) const fn ioc_type(nr: u32) -> u32 {
    (nr >> IOC_TYPESHIFT) & IOC_TYPEMASK
}

/// Decode ioctl code to its number.
pub(crate) const fn ioc_nr(nr: u32) -> u32 {
    (nr >> IOC_NRSHIFT) & IOC_NRMASK
}

/// Decode ioctl code to its size.
pub(crate) const fn ioc_size(nr: u32) -> u32 {
    (nr >> IOC_SIZESHIFT) & IOC_SIZEMASK
}

/// Convert a C string of a maximum size (which might not be NUL-terminated if the size is reached)
/// info a Rust `String``.
///
/// This differs from `std::ffi::CStr::from_bytes_with_nul` in that it does not require the input to
/// be NUL-terminated.
pub(crate) fn cstr_to_string(buf: &[u8]) -> String {
    let len = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..len]).to_string()
}

#[cfg(test)]
mod ccompat_tests;
