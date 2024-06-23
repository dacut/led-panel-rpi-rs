package main

/*
#include <sys/ioctl.h>
#include <linux/gpio.h>

int get_gpio_chip_info(int fd, struct gpiochip_info *info) {
	return ioctl(fd, GPIO_GET_CHIPINFO_IOCTL, info);
}

int get_gpio_v2_line_info(int fd, struct gpio_v2_line_info *info) {
	return ioctl(fd, GPIO_V2_GET_LINEINFO_IOCTL, info);
}

__u64 convert_line_attribute(struct gpio_v2_line_attribute *attr) {
	switch (attr->id) {
	case GPIO_V2_LINE_ATTR_ID_FLAGS:
		return attr->flags;

	case GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES:
		return attr->values;

	case GPIO_V2_LINE_ATTR_ID_DEBOUNCE:
		return attr->debounce_period_us;

	default:
		return 0;
	}
}

*/
import "C"
import (
	"fmt"
	"os"
	"strings"
	"syscall"
)

type GPIO struct {
	fd *os.File
}

type GPIOLineFlag uint64

const (
	// GPIOLineFlagUsed indicates that the line is not available for requests.
	GPIOLineFlagUsed GPIOLineFlag = 1 << 0

	// GPIOLineFlagActiveLow indicates that the line active state is physical low.
	GPIOLineFlagActiveLow GPIOLineFlag = 1 << 1

	// GPIOLineFlagInput indicates that the line is an input.
	GPIOLineFlagInput GPIOLineFlag = 1 << 2

	// GPIOLineFlagOutput indicates that the line is an output.
	GPIOLineFlagOutput GPIOLineFlag = 1 << 3

	// GPIOLineFlagEdgeRising indicates that the line detects rising (inactive to active) edges.
	GPIOLineFlagEdgeRising GPIOLineFlag = 1 << 4

	// GPIOLineFlagEdgeFalling indicates that the line detects falling (active to inactive) edges.
	GPIOLineFlagEdgeFalling GPIOLineFlag = 1 << 5

	// GPIOLineFlagOpenDrain indicates that the line is an open-drain output.
	GPIOLineFlagOpenDrain GPIOLineFlag = 1 << 6

	// GPIOLineFlagOpenSource indicates that the line is an open-source output.
	GPIOLineFlagOpenSource GPIOLineFlag = 1 << 7

	// GPIOLineFlagBiasPullUp indicates that the line has pull-up bias enabled.
	GPIOLineFlagBiasPullUp GPIOLineFlag = 1 << 8

	// GPIOLineFlagBiasPullDown indicates that the line has pull-down bias enabled.
	GPIOLineFlagBiasPullDown GPIOLineFlag = 1 << 9

	// GPIOLineFlagBiasDisabled indicates that the line has bias disabled.
	GPIOLineFlagBiasDisabled GPIOLineFlag = 1 << 10

	// GPIOLineFlagEventClockRealtime indicates that line events contain realtime timestamps.
	GPIOLineFlagEventClockRealtime GPIOLineFlag = 1 << 11

	// GPIOLineFlagEventClockHTE indicates that line events contain timestamps from the hardware timestamp engine.
	GPIOLineFlagEventClockHTE GPIOLineFlag = 1 << 12
)

func (flag GPIOLineFlag) String() string {
	var flags []string
	if flag&GPIOLineFlagUsed != 0 {
		flags = append(flags, "Used")
	}

	if flag&GPIOLineFlagActiveLow != 0 {
		flags = append(flags, "ActiveLow")
	}

	if flag&GPIOLineFlagInput != 0 {
		flags = append(flags, "Input")
	}

	if flag&GPIOLineFlagOutput != 0 {
		flags = append(flags, "Output")
	}

	if flag&GPIOLineFlagEdgeRising != 0 {
		flags = append(flags, "EdgeRising")
	}

	if flag&GPIOLineFlagEdgeFalling != 0 {
		flags = append(flags, "EdgeFalling")
	}

	if flag&GPIOLineFlagOpenDrain != 0 {
		flags = append(flags, "OpenDrain")
	}

	if flag&GPIOLineFlagOpenSource != 0 {
		flags = append(flags, "OpenSource")
	}

	if flag&GPIOLineFlagBiasPullUp != 0 {
		flags = append(flags, "BiasPullUp")
	}

	if flag&GPIOLineFlagBiasPullDown != 0 {
		flags = append(flags, "BiasPullDown")
	}

	if flag&GPIOLineFlagBiasDisabled != 0 {
		flags = append(flags, "BiasDisabled")
	}

	if flag&GPIOLineFlagEventClockRealtime != 0 {
		flags = append(flags, "EventClockRealtime")
	}

	if flag&GPIOLineFlagEventClockHTE != 0 {
		flags = append(flags, "EventClockHTE")
	}

	if len(flags) == 0 {
		return "None"
	}

	return strings.Join(flags, "|")
}

type GPIOLineAttributeID uint32

const (
	// GPIOLineAttributeIDFlags requests flags for the line.
	GPIOLineAttributeIDFlags GPIOLineAttributeID = 1

	// GPIOLineAttributeIDOutputValues requests output values for the line.
	GPIOLineAttributeIDOutputValues GPIOLineAttributeID = 2

	// GPIOLineAttributeIDDebounce requests the debounce time (us) for the line.
	GPIOLineAttributeIDDebounce GPIOLineAttributeID = 3
)

type GPIOLineAttribute interface {
	GetID() GPIOLineAttributeID
}

type GPIOLineAttributeFlags struct {
	Flags uint64
}

func (a GPIOLineAttributeFlags) GetID() GPIOLineAttributeID {
	return GPIOLineAttributeIDFlags
}

func (a GPIOLineAttributeFlags) String() string {
	return fmt.Sprintf("flags=0x%x", a.Flags)
}

type GPIOLineAttributeOutputValues struct {
	Values uint64
}

func (a GPIOLineAttributeOutputValues) GetID() GPIOLineAttributeID {
	return GPIOLineAttributeIDOutputValues
}

func (a GPIOLineAttributeOutputValues) String() string {
	return fmt.Sprintf("values=0x%x", a.Values)
}

type GPIOLineAttributeDebounce struct {
	DebouncePeriodMicroseconds uint32
}

func (a GPIOLineAttributeDebounce) String() string {
	return fmt.Sprintf("debounce_period_us=%d", a.DebouncePeriodMicroseconds)
}

func (a GPIOLineAttributeDebounce) GetID() GPIOLineAttributeID {
	return GPIOLineAttributeIDDebounce
}

// GPIOChipInfo provides information about a GPIO chip.
type GPIOChipInfo struct {
	Name  string
	Label string
	Lines uint32
}

// GPIOLineInfo provides information about a GPIO line.
type GPIOLineInfo struct {
	Name       string
	Consumer   string
	Offset     uint32
	Flags      GPIOLineFlag
	Attributes []GPIOLineAttribute
}

func (li *GPIOLineInfo) String() string {
	return fmt.Sprintf("name=%#v consumer=%#v offset=%v flags=%s attributes=%v", li.Name, li.Consumer, li.Offset, li.Flags, li.Attributes)
}

func ListGPIODevices() ([]string, error) {
	entries, err := os.ReadDir("/dev")
	if err != nil {
		return nil, err
	}

	var gpios []string
	for _, entry := range entries {
		if strings.HasPrefix(entry.Name(), "gpiochip") {
			gpios = append(gpios, "/dev/"+entry.Name())
		}
	}

	return gpios, nil
}

func OpenGPIO(dev string) (*GPIO, error) {
	fd, err := os.OpenFile(dev, os.O_RDWR, 0)
	if err != nil {
		return nil, err
	}

	return &GPIO{fd}, nil
}

func (g *GPIO) GetChipInfo() (*GPIOChipInfo, error) {
	info := &C.struct_gpiochip_info{}
	fd := int(g.fd.Fd())
	if err, _ := C.get_gpio_chip_info(C.int(fd), info); err != 0 {
		return nil, syscall.Errno(err)
	}

	return &GPIOChipInfo{
		Name:  C.GoString(&info.name[0]),
		Label: C.GoString(&info.label[0]),
		Lines: uint32(info.lines),
	}, nil
}

func (g *GPIO) GetLineInfo(line uint32) (*GPIOLineInfo, error) {
	info := &C.struct_gpio_v2_line_info{offset: C.uint(line)}
	fd := int(g.fd.Fd())
	if err, _ := C.get_gpio_v2_line_info(C.int(fd), info); err != 0 {
		return nil, syscall.Errno(err)
	}

	var attributes []GPIOLineAttribute
	nAttrs := int(info.num_attrs)

	for i := 0; i < nAttrs; i++ {
		value := C.convert_line_attribute(&info.attrs[i])
		switch info.attrs[i].id {
		case C.GPIO_V2_LINE_ATTR_ID_FLAGS:
			attributes = append(attributes, GPIOLineAttributeFlags{Flags: uint64(value)})
		case C.GPIO_V2_LINE_ATTR_ID_OUTPUT_VALUES:
			attributes = append(attributes, GPIOLineAttributeOutputValues{Values: uint64(value)})
		case C.GPIO_V2_LINE_ATTR_ID_DEBOUNCE:
			attributes = append(attributes, GPIOLineAttributeDebounce{DebouncePeriodMicroseconds: uint32(value)})
		}
	}

	return &GPIOLineInfo{
		Name:       C.GoString(&info.name[0]),
		Consumer:   C.GoString(&info.consumer[0]),
		Offset:     uint32(info.offset),
		Flags:      GPIOLineFlag(info.flags),
		Attributes: attributes,
	}, nil
}

func (g *GPIO) Close() error {
	return g.fd.Close()
}
