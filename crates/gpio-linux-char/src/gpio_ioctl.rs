use {
    crate::ioctl::{ior, iowr},
    std::{
        io::{Error as IoError, Result as IoResult},
        os::fd::RawFd,
    },
};

/// The maximum size of GPIO name and label arrays.
pub(crate) const GPIO_MAX_NAME_SIZE: usize = 32;

/// Maximum number of requested lines.
pub(crate) const GPIO_V2_LINES_MAX: usize = 64;

/// The maximum number of configuration attributes associated with a line request.
pub(crate) const GPIO_V2_LINE_NUM_ATTRS_MAX: usize = 10;

/// IOCTL: Get chip information.
pub(crate) const GPIO_GET_CHIPINFO_IOCTL: u64 = ior::<RawGpioChipInfo>(0xb4, 0x01) as u64;

/// IOCTL: ?
#[allow(dead_code)]
pub(crate) const GPIO_GET_LINEINFO_UNWATCH_IOCTL: u64 = iowr::<u32>(0xb4, 0x0c) as u64;

/// IOCTL: Get line information.
pub(crate) const GPIO_V2_GET_LINEINFO_IOCTL: u64 = iowr::<RawGpioV2LineInfo>(0xb4, 0x05) as u64;

/// IOCTL: ?
#[allow(dead_code)]
pub(crate) const GPIO_V2_GET_LINEINFO_WATCH_IOCTL: u64 = iowr::<RawGpioV2LineInfo>(0xb4, 0x06) as u64;

/// Line attribute id: flags
pub(crate) const GPIO_V2_LINE_ATTR_ID_FLAGS: u32 = 1;

/// Line attribute id: values
pub(crate) const GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES: u32 = 2;

/// Line attribute id: debounce period
pub(crate) const GPIO_V2_LINE_ATTR_ID_DEBOUNCE: u32 = 3;

/// Struct `gpiochip_info` from `/usr/include/linux/gpio.h`.
#[repr(C)]
#[derive(Default)]
pub(crate) struct RawGpioChipInfo {
    pub(crate) name: [u8; GPIO_MAX_NAME_SIZE],
    pub(crate) label: [u8; GPIO_MAX_NAME_SIZE],
    pub(crate) lines: u32,
}

impl RawGpioChipInfo {
    /// Returns information about this GPIO chip.
    pub fn get_chip_info(&self, fd: RawFd) -> IoResult<RawGpioChipInfo> {
        let mut raw = RawGpioChipInfo::default();
        let ret = unsafe { libc::ioctl(fd, GPIO_GET_CHIPINFO_IOCTL, &mut raw as *mut _) };
        if ret != 0 {
            Err(IoError::last_os_error())
        } else {
            Ok(raw)
        }
    }
}

/// Struct `gpio_v2_line_info` from `/usr/include/linux/gpio.h`.
#[repr(C)]
#[derive(Default)]
pub(crate) struct RawGpioV2LineInfo {
    pub(crate) name: [u8; GPIO_MAX_NAME_SIZE],
    pub(crate) consumer: [u8; GPIO_MAX_NAME_SIZE],
    pub(crate) offset: u32,
    pub(crate) num_attrs: u32,
    pub(crate) flags: u64,
    pub(crate) attrs: [RawGpioV2LineAttr; GPIO_V2_LINE_NUM_ATTRS_MAX],
    pub(crate) padding: [u32; 4],
}

impl RawGpioV2LineInfo {
    pub(crate) fn get_line_info(fd: RawFd, offset: u32) -> IoResult<Self> {
        let mut result = Self {
            offset,
            ..Default::default()
        };
        let ret = unsafe { libc::ioctl(fd, GPIO_V2_GET_LINEINFO_IOCTL, &mut result as *mut _) };
        if ret != 0 {
            Err(IoError::last_os_error())
        } else {
            Ok(result)
        }
    }
}

/// Struct `gpio_v2_line_attribute` from `/usr/include/linux/gpio.h`.
#[repr(C)]
#[derive(Default)]
pub(crate) struct RawGpioV2LineAttr {
    pub(crate) id: u32,
    pub(crate) padding: u32,
    pub(crate) data: RawGpioV2LineAttrValue,
}

/// Line attribute information union.
#[repr(C)]
pub(crate) union RawGpioV2LineAttrValue {
    pub(crate) flags: u64,
    pub(crate) values: u64,
    pub(crate) debounce_period_us: u32,
}

impl Default for RawGpioV2LineAttrValue {
    fn default() -> Self {
        Self {
            flags: 0,
        }
    }
}

#[cfg(test)]
mod ccompat_tests;
