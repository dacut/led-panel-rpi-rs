//! This is an automatically generated file; do not edit.
//! Generated by gen_ccompat_tests.c on Jun 23 2024 07:28:17
use std::mem::{offset_of, size_of};

#[test]
fn test_gpio_ioctl_ccompat() {
    assert_eq!(super::GPIO_GET_CHIPINFO_IOCTL, 0x8044b401);
    assert_eq!(super::GPIO_GET_LINEINFO_UNWATCH_IOCTL, 0xc004b40c);
    assert_eq!(super::GPIO_V2_GET_LINEINFO_IOCTL, 0xc100b405);
    assert_eq!(super::GPIO_V2_LINE_ATTR_ID_FLAGS, 0x1);
    assert_eq!(super::GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES, 0x2);
    assert_eq!(super::GPIO_V2_LINE_ATTR_ID_DEBOUNCE, 0x3);
    assert_eq!(size_of::<super::RawGpioChipInfo>(), 68);
    assert_eq!(size_of::<super::RawGpioV2LineAttr>(), 16);
    assert_eq!(size_of::<super::RawGpioV2LineInfo>(), 256);
    assert_eq!(offset_of!(super::RawGpioV2LineAttr, id), 0);
    assert_eq!(offset_of!(super::RawGpioV2LineAttr, padding), 4);
    assert_eq!(offset_of!(super::RawGpioV2LineAttr, data) + offset_of!(super::RawGpioV2LineAttrValue, flags), 8);
    assert_eq!(offset_of!(super::RawGpioV2LineAttr, data) + offset_of!(super::RawGpioV2LineAttrValue, values), 8);
    assert_eq!(
        offset_of!(super::RawGpioV2LineAttr, data) + offset_of!(super::RawGpioV2LineAttrValue, debounce_period_us),
        8
    );
    assert_eq!(offset_of!(super::RawGpioV2LineInfo, name), 0);
    assert_eq!(offset_of!(super::RawGpioV2LineInfo, consumer), 32);
    assert_eq!(offset_of!(super::RawGpioV2LineInfo, offset), 64);
    assert_eq!(offset_of!(super::RawGpioV2LineInfo, num_attrs), 68);
    assert_eq!(offset_of!(super::RawGpioV2LineInfo, flags), 72);
    assert_eq!(offset_of!(super::RawGpioV2LineInfo, attrs), 80);
    assert_eq!(offset_of!(super::RawGpioV2LineInfo, padding), 240);
}
