//! General Purpose Inout/Output (GPIO) driver for Linux using the character device interface.

#![warn(missing_docs)]

use {
    log::warn,
    std::{
        error::Error,
        fmt::{Display, Formatter, Result as FmtResult},
        fs::File,
        io::{Error as IoError, Result as IoResult},
        ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
        os::{
            fd::{IntoRawFd, RawFd},
            unix::fs::FileTypeExt,
        },
        path::{Path, PathBuf},
        time::Duration,
    },
};

pub(crate) mod gpio_ioctl;

/// Maximum number of lines per chip.
pub const MAX_GPIO_LINES_PER_CHIP: usize = gpio_ioctl::GPIO_V2_LINES_MAX;

/// General Purpose Input/Output (GPIO) driver.
#[derive(Debug)]
pub struct Gpio {
    fd: RawFd,
}

/// Indicates whether a string is composed entirely of ASCII digits.
fn str_is_ascii_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
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

impl Gpio {
    /// Convert a string describing a GPIO chip into a path, or return an error.
    pub fn parse_chip_descriptor(desc: &str) -> IoResult<PathBuf> {
        if desc.is_empty() {
            Err(IoError::new(std::io::ErrorKind::InvalidInput, "Empty GPIO chip descriptor"))
        } else if desc.starts_with('/') {
            Ok(PathBuf::from(desc))
        } else if str_is_ascii_digits(desc) {
            Ok(PathBuf::from(format!("/dev/gpiochip{desc}")))
        } else {
            Ok(PathBuf::from("/dev").join(desc))
        }
    }

    /// Open a GPIO character device.
    ///
    /// # Arguments
    /// * `path`: The full path to the GPIO character device.
    ///
    /// # Errors
    /// If the GPIO character device cannot be opened, the underlying [`IoError`][std::io::Error] is returned.
    ///
    /// If the GPIO character device is not a character device, an [`IoError`][std::io::Error] is returned with a kind of
    /// [`Other`][std::io::ErrorKind::Other] wrapping a [`GpioError::NotCharDev`].
    pub fn open(path: impl AsRef<Path>) -> IoResult<Self> {
        let fd = File::options().read(true).write(true).open(path)?;
        if !fd.metadata()?.file_type().is_char_device() {
            Err(IoError::other(GpioError::NotCharDev))
        } else {
            Ok(Self {
                fd: fd.into_raw_fd(),
            })
        }
    }

    /// List available GPIO chips.
    ///
    /// The result is a list of [`PathBuf`]s representing absolute paths to GPIO character devices.
    ///
    /// # Errors
    /// If the `/dev` directory cannot be read, the underlying [`IoError`][std::io::Error] is returned.
    ///
    /// Device files that cannot be interrogated are skipped (with an error message logged).
    pub fn list_chips() -> IoResult<Vec<PathBuf>> {
        let mut devices = vec![];
        for entry in Path::new("/dev").read_dir()? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    warn!("Failed to read directory entry: {e}");
                    continue;
                }
            };

            if !entry.file_name().to_string_lossy().starts_with("gpiochip") {
                continue;
            }

            let path = entry.path();

            let ft = match entry.file_type() {
                Ok(ft) => ft,
                Err(e) => {
                    warn!("Failed to get file type for {}: {}", path.to_string_lossy(), e);
                    continue;
                }
            };

            if ft.is_char_device() {
                devices.push(path);
            }
        }

        Ok(devices)
    }

    /// Get information about this GPIO chip.
    pub fn get_chip_info(&self) -> IoResult<GpioChipInfo> {
        let raw = gpio_ioctl::RawGpioChipInfo::default();
        let raw = raw.get_chip_info(self.fd)?;
        Ok(raw.into())
    }

    /// Get information about a GPIO line.
    pub fn get_line_info(&self, line: usize) -> IoResult<GpioLineInfo> {
        let Ok(line) = line.try_into() else {
            return Err(IoError::new(std::io::ErrorKind::InvalidInput, "Invalid GPIO line number"));
        };

        let raw = gpio_ioctl::RawGpioV2LineInfo::get_line_info(self.fd, line)?;
        Ok(raw.into())
    }
}

impl Drop for Gpio {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}

/// GPIO chip information.
#[derive(Clone, Debug)]
pub struct GpioChipInfo {
    /// The name of the GPIO chip.
    pub name: String,

    /// A label set on the GPIO chip.
    pub label: String,

    /// The number of GPIO lines on this chip.
    pub lines: usize,
}

impl From<gpio_ioctl::RawGpioChipInfo> for GpioChipInfo {
    fn from(raw: gpio_ioctl::RawGpioChipInfo) -> Self {
        Self {
            name: cstr_to_string(&raw.name),
            label: cstr_to_string(&raw.label),
            lines: raw.lines as usize,
        }
    }
}

/// GPIO line information.
#[derive(Clone, Debug)]
pub struct GpioLineInfo {
    /// The name of the GPIO line.
    pub name: String,

    /// The consumer of the GPIO line.
    pub consumer: String,

    /// The offset of the GPIO line (bit position for the line within the GPIO chip).
    pub offset: usize,

    /// Flags associated with this line.
    pub flags: GpioLineFlags,

    /// Attributes associated with this line.
    pub attrs: Vec<GpioLineAttr>,
}

impl From<gpio_ioctl::RawGpioV2LineInfo> for GpioLineInfo {
    fn from(raw: gpio_ioctl::RawGpioV2LineInfo) -> Self {
        let mut attrs = Vec::with_capacity(raw.num_attrs as usize);

        let name = cstr_to_string(&raw.name);
        let consumer = cstr_to_string(&raw.consumer);
        let offset = raw.offset as usize;
        let flags = GpioLineFlags(raw.flags);

        for raw_attr in raw.attrs.iter().take(raw.num_attrs as usize) {
            match raw_attr.id {
                gpio_ioctl::GPIO_V2_LINE_ATTR_ID_FLAGS => {
                    let flags = GpioLineFlags(unsafe { raw_attr.data.flags });
                    attrs.push(GpioLineAttr::Flags(flags));
                }
                gpio_ioctl::GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES => {
                    let values = unsafe { raw_attr.data.values };
                    attrs.push(GpioLineAttr::Values(values));
                }
                gpio_ioctl::GPIO_V2_LINE_ATTR_ID_DEBOUNCE => {
                    let period = Duration::from_micros(unsafe { raw_attr.data.debounce_period_us } as u64);
                    attrs.push(GpioLineAttr::DebouncePeriod(period));
                }
                _ => {
                    warn!("Unknown GPIO line attribute ID: {}", raw_attr.id);
                }
            }
        }

        Self {
            name,
            consumer,
            offset,
            flags,
            attrs,
        }
    }
}

/// Flags associated with a GPIO line.
#[derive(Clone, Copy, Debug, Default)]
pub struct GpioLineFlags(u64);

impl BitAnd for GpioLineFlags {
    type Output = Self;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for GpioLineFlags {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for GpioLineFlags {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for GpioLineFlags {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXor for GpioLineFlags {
    type Output = Self;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for GpioLineFlags {
    #[inline(always)]
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Not for GpioLineFlags {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Display for GpioLineFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.0 == 0 {
            f.write_str("None")
        } else {
            let mut parts = vec![];
            for flag in GpioLineFlag::all() {
                if self.0 & (*flag as u64) != 0 {
                    parts.push(flag.to_string());
                }
            }

            f.write_str(&parts.join(" | "))
        }
    }
}

impl From<GpioLineFlag> for GpioLineFlags {
    #[inline(always)]
    fn from(flag: GpioLineFlag) -> Self {
        Self(flag as u64)
    }
}

/// Possible bits for GpioLineFlags.
#[repr(u64)]
#[derive(Clone, Copy, Debug)]
pub enum GpioLineFlag {
    /// Line is not available for requests
    Used = 1 << 0,

    /// The line's active state is physical low.
    ActiveLow = 1 << 1,

    /// Line is an input.
    Input = 1 << 2,

    /// Line is an output.
    Output = 1 << 3,

    /// The line detects rising (inactive to active) edges.
    EdgeRising = 1 << 4,

    /// The line detects falling (active to inactive) edges.
    EdgeFalling = 1 << 5,

    /// The line is an open-drain output.
    OpenDrain = 1 << 6,

    /// The line is an open-source output.
    OpenSource = 1 << 7,

    /// The line has a pull-up bias resistor enabled.
    BiasPullUp = 1 << 8,

    /// The line has a pull-down bias resistor enabled.
    BiasPullDown = 1 << 9,

    /// The line has no bias resistor enabled.
    BiasDisabled = 1 << 10,

    /// Line events contain realtime timestamps.
    EventClockRealtime = 1 << 11,

    /// Line events contain timestamps from the hardware timestamp engine.
    EventClockHte = 1 << 12,
}

impl BitAnd for GpioLineFlag {
    type Output = GpioLineFlags;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> <Self as BitAnd>::Output {
        // Always zero for now
        GpioLineFlags(self as u64 & rhs as u64)
    }
}

impl BitOr for GpioLineFlag {
    type Output = GpioLineFlags;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> <Self as BitOr>::Output {
        GpioLineFlags(self as u64 | rhs as u64)
    }
}

impl BitXor for GpioLineFlag {
    type Output = GpioLineFlags;

    #[inline(always)]
    fn bitxor(self, rhs: Self) -> <Self as BitXor>::Output {
        // Always zero for now
        GpioLineFlags(self as u64 ^ rhs as u64)
    }
}

impl Not for GpioLineFlag {
    type Output = GpioLineFlags;

    #[inline(always)]
    fn not(self) -> <Self as Not>::Output {
        GpioLineFlags(!(self as u64))
    }
}

impl Display for GpioLineFlag {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Used => f.write_str("Used"),
            Self::ActiveLow => f.write_str("ActiveLow"),
            Self::Input => f.write_str("Input"),
            Self::Output => f.write_str("Output"),
            Self::EdgeRising => f.write_str("EdgeRising"),
            Self::EdgeFalling => f.write_str("EdgeFalling"),
            Self::OpenDrain => f.write_str("OpenDrain"),
            Self::OpenSource => f.write_str("OpenSource"),
            Self::BiasPullUp => f.write_str("BiasPullUp"),
            Self::BiasPullDown => f.write_str("BiasPullDown"),
            Self::BiasDisabled => f.write_str("BiasDisabled"),
            Self::EventClockRealtime => f.write_str("EventClockRealtime"),
            Self::EventClockHte => f.write_str("EventClockHte"),
        }
    }
}

impl GpioLineFlag {
    /// Return all flags.
    pub fn all() -> &'static [Self] {
        &[
            Self::Used,
            Self::ActiveLow,
            Self::Input,
            Self::Output,
            Self::EdgeRising,
            Self::EdgeFalling,
            Self::OpenDrain,
            Self::OpenSource,
            Self::BiasPullUp,
            Self::BiasPullDown,
            Self::BiasDisabled,
            Self::EventClockRealtime,
            Self::EventClockHte,
        ]
    }
}

/// Configurable attribute of a line.
#[derive(Clone, Copy, Debug)]
pub enum GpioLineAttr {
    /// Flags associated with the line.
    Flags(GpioLineFlags),

    /// Bitmap contaiing the values to which the line will be set.
    Values(u64),

    /// The desired debounce period.
    DebouncePeriod(Duration),
}

impl Display for GpioLineAttr {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Flags(flags) => write!(f, "Flags: {}", flags),
            Self::Values(values) => write!(f, "Values: 0b{:064b}", values),
            Self::DebouncePeriod(period) => write!(f, "Debounce Period: {:?}", period),
        }
    }
}

/// Errors returned by this driver.
#[derive(Debug)]
pub enum GpioError {
    /// The underlying file is not a character device.
    NotCharDev,
}

impl Display for GpioError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::NotCharDev => write!(f, "GPIO device is not a character device"),
        }
    }
}

impl Error for GpioError {}
