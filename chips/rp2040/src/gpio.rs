//! GPIO, RP2040
//!
//! ### Author
//! * Ioana Culic <ioana.culic@wyliodrin.com>

use enum_primitive::cast::FromPrimitive;
use enum_primitive::enum_from_primitive;
use kernel::common::cells::OptionalCell;
use kernel::common::registers::{register_bitfields, register_structs, ReadOnly, ReadWrite};
use kernel::common::StaticRef;
use kernel::hil;

struct GpioPin {
    status: ReadOnly<u32, GPIOx_STATUS::Register>,
    ctrl: ReadWrite<u32, GPIOx_CTRL::Register>,
}

struct GpioInterrupt {
    intr0: ReadWrite<u32, GPIO_INTR0::Register>,
    intr1: ReadWrite<u32, GPIO_INTR1::Register>,
    intr2: ReadWrite<u32, GPIO_INTR1::Register>,
    intr3: ReadWrite<u32, GPIO_INTR3::Register>,
}

struct GpioProc {
    enable: GpioInterrupt,
    force: GpioInterrupt,
    status: GpioInterrupt,
}

register_structs! {
    /// GPIO Registers.
    GpioRegisters {
        (0x000 => pin: [GpioPin; 30]),

        /// Raw interrupts
        (0x0f0 => intr: GpioInterrupt),

        /// Interrupts for procs
        (0x100 => proc: [GpioProc; 2]),

        /// Wake
        (0x160 => wake: GpioProc),

        /// End
        (0x190 => @END),
    }
}

register_structs! {
    /// User Bank Pad Control Registers
    GpioPadRegisters {
        /// Voltage select
        (0x00 => voltage: ReadWrite<u32, VOLTAGE_SELECT::Register>),

        /// Pads control
        (0x04 => gpio_pad: [ReadWrite<u32, GPIO_PAD::Register>; 32]),

        /// End
        (0x84 => @END),
    }
}

register_structs! {
    /// SIO Control Registers
    SIORegisters {
        /// Not used
        (0x000 => _reserved0),

        /// Input value for GPIO pins
        (0x004 => gpio_in: ReadWrite<u32, GPIO_IN::Register>),

        /// Not used
        (0x008 => _reserved1),

        /// GPIO output value
        (0x010 => gpio_out: ReadWrite<u32, GPIO_OUT::Register>),

        /// GPIO output value set
        (0x014 => gpio_out_set: ReadWrite<u32, GPIO_OUT_SET::Register>),

        /// GPIO output value clear
        (0x018 => gpio_out_clr: ReadWrite<u32, GPIO_OUT_CLR::Register>),

        /// GPIO output value XOR
        (0x01c => gpio_out_xor: ReadWrite<u32, GPIO_OUT_XOR::Register>),

        /// GPIO output enable
        (0x020 => gpio_oe: ReadWrite<u32, GPIO_OE::Register>),

        /// GPIO output enable set
        (0x024 => gpio_oe_set: ReadWrite<u32, GPIO_OE_SET::Register>),

        /// GPIO output enable clear
        (0x028 => gpio_oe_clr: ReadWrite<u32, GPIO_OE_CLR::Register>),

        /// End
        (0x02c => @END),
    }
}

register_bitfields![u32,
    GPIOx_STATUS [
        /// interrupt to processors, after override is applied
        IRQTOPROC OFFSET(26) NUMBITS(1) [],
        /// interrupt from pad before override is applied
        IRQFROMPAD OFFSET(24) NUMBITS(1) [],
        /// input signal to peripheral, after override is applied
        INTOPERI OFFSET(19) NUMBITS(1) [],
        /// input signal from pad, before override is applied
        INFROMPAD OFFSET(17) NUMBITS(1) [],
        /// output enable to pad after register override is applied
        OETOPAD OFFSET(13) NUMBITS(1) [],
        /// output enable from selected peripheral, before registeroverride is applied
        OEFROMPERI OFFSET(12) NUMBITS(1) [],
        /// output signal to pad after register override is applied
        OUTTOPAD OFFSET(9) NUMBITS(1) [],
        /// output signal from selected peripheral, before registeroverride is applied
        OUTFROMPERI OFFSET(8) NUMBITS(1) []
    ],
    GPIOx_CTRL [
        /// interrupt override?
        IRQOVER OFFSET(28) NUMBITS(2) [
            NoInvert = 0,
            Invert = 1,
            DriveLow = 2,
            DriveHigh = 3
        ],
        /// input override
        INOVER OFFSET(16) NUMBITS(2) [
            NoInvert = 0,
            Invert = 1,
            DriveLow = 2,
            DriveHigh = 3
        ],
        /// output enable override
        OEOVER OFFSET(12) NUMBITS(2) [
            EnableSignal = 0,
            EnableInverseSignal = 1,
            Disable = 2,
            Enable = 3
        ],
        /// output override
        OUTOVER OFFSET(8) NUMBITS(2) [
            Signal = 0,
            InverseSignal = 1,
            Low = 2,
            High = 3
        ],
        /// Function select
        FUNCSEL OFFSET(0) NUMBITS(5) [
            GPIO_FUNC_XIP = 0,
            GPIO_FUNC_SPI = 1,
            GPIO_FUNC_UART = 2,
            GPIO_FUNC_I2C = 3,
            GPIO_FUNC_PWM = 4,
            GPIO_FUNC_SIO = 5,
            GPIO_FUNC_PIO0 = 6,
            GPIO_FUNC_PIO1 = 7,
            GPIO_FUNC_GPCK = 8,
            GPIO_FUNC_USB = 9,
            GPIO_FUNC_NULL = 0x1f
        ]
    ],
    GPIO_INTR0 [
        GPIO7_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO7_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO7_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO7_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO6_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO6_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO6_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO6_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO5_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO5_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO5_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO5_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO4_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO4_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO4_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO4_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO3_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO3_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO3_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO3_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO2_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO2_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO2_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO2_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO1_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO1_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO1_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO1_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO0_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO0_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO0_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO0_LEVEL_LOW OFFSET(0) NUMBITS(1) []
    ],
    GPIO_INTR1[
        GPIO15_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO15_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO15_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO15_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO14_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO14_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO14_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO14_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO13_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO13_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO13_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO13_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO12_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO12_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO12_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO12_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO11_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO11_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO11_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO11_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO10_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO10_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO10_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO10_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO9_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO9_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO9_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO9_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO8_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO8_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO8_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO8_LEVEL_LOW OFFSET(0) NUMBITS(1) []
    ],
    GPIO_INTR2[
        GPIO23_EDGE_HIGH OFFSET(31) NUMBITS(1) [],
        GPIO23_EDGE_LOW OFFSET(30) NUMBITS(1) [],
        GPIO23_LEVEL_HIGH OFFSET(29) NUMBITS(1) [],
        GPIO23_LEVEL_LOW OFFSET(28) NUMBITS(1) [],

        GPIO22_EDGE_HIGH OFFSET(27) NUMBITS(1) [],
        GPIO22_EDGE_LOW OFFSET(26) NUMBITS(1) [],
        GPIO22_LEVEL_HIGH OFFSET(25) NUMBITS(1) [],
        GPIO22_LEVEL_LOW OFFSET(24) NUMBITS(1) [],

        GPIO21_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO21_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO21_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO21_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO20_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO20_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO20_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO20_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO19_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO19_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO19_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO19_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO18_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO18_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO18_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO18_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO17_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO17_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO17_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPIO17_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO16_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO16_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO16_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO16_LEVEL_LOW OFFSET(0) NUMBITS(1) []
    ],
    GPIO_INTR3[
        GPIO29_EDGE_HIGH OFFSET(23) NUMBITS(1) [],
        GPIO29_EDGE_LOW OFFSET(22) NUMBITS(1) [],
        GPIO29_LEVEL_HIGH OFFSET(21) NUMBITS(1) [],
        GPIO29_LEVEL_LOW OFFSET(20) NUMBITS(1) [],

        GPIO28_EDGE_HIGH OFFSET(19) NUMBITS(1) [],
        GPIO28_EDGE_LOW OFFSET(18) NUMBITS(1) [],
        GPIO28_LEVEL_HIGH OFFSET(17) NUMBITS(1) [],
        GPIO28_LEVEL_LOW OFFSET(16) NUMBITS(1) [],

        GPIO27_EDGE_HIGH OFFSET(15) NUMBITS(1) [],
        GPIO27_EDGE_LOW OFFSET(14) NUMBITS(1) [],
        GPIO27_LEVEL_HIGH OFFSET(13) NUMBITS(1) [],
        GPIO27_LEVEL_LOW OFFSET(12) NUMBITS(1) [],

        GPIO26_EDGE_HIGH OFFSET(11) NUMBITS(1) [],
        GPIO26_EDGE_LOW OFFSET(10) NUMBITS(1) [],
        GPIO26_LEVEL_HIGH OFFSET(9) NUMBITS(1) [],
        GPIO26_LEVEL_LOW OFFSET(8) NUMBITS(1) [],

        GPIO25_EDGE_HIGH OFFSET(7) NUMBITS(1) [],
        GPIO25_EDGE_LOW OFFSET(6) NUMBITS(1) [],
        GPIO25_LEVEL_HIGH OFFSET(5) NUMBITS(1) [],
        GPI025_LEVEL_LOW OFFSET(4) NUMBITS(1) [],

        GPIO24_EDGE_HIGH OFFSET(3) NUMBITS(1) [],
        GPIO24_EDGE_LOW OFFSET(2) NUMBITS(1) [],
        GPIO24_LEVEL_HIGH OFFSET(1) NUMBITS(1) [],
        GPIO24_LEVEL_LOW OFFSET(0) NUMBITS(1) []
    ],
    VOLTAGE_SELECT[
        VOLTAGE OFFSET(0) NUMBITS(1) [
            Set3V3 = 0,
            Set1V8 = 1
        ]
    ],
    GPIO_PAD [
        OD OFFSET(7) NUMBITS(1) [],
        IE OFFSET(6) NUMBITS(1) [],
        DRIVE OFFSET(4) NUMBITS(2) [],
        PUE OFFSET(3) NUMBITS(1) [],
        PDE OFFSET(2) NUMBITS(1) [],
        SCHMITT OFFSET(1) NUMBITS(1) [],
        SLEWFAST OFFSET(0) NUMBITS(1) []
    ],
    GPIO_IN [
        ///Input value for GPIO0..29
        IN OFFSET(0) NUMBITS(30) []
    ],
    GPIO_OUT [
        ///Set output level (1/0 → high/low) for GPIO0...29.
        OUT OFFSET(0) NUMBITS(30) []
    ],
    GPIO_OUT_SET [
        ///Perform an atomic bit-set on GPIO_OUT
        OUT OFFSET(0) NUMBITS(30) []
    ],
    GPIO_OUT_CLR [
        ///Perform an atomic bit-clear on GPIO_OUT
        OUT OFFSET(0) NUMBITS(30) []
    ],
    GPIO_OUT_XOR [
        ///Perform an atomic bitwise XOR on GPIO_OUT
        OUT OFFSET(0) NUMBITS(30) []
    ],
    GPIO_OE [
        ///Set output enable (1/0 → output/input) for GPIO0...29
        OE OFFSET(0) NUMBITS(30) []
    ],
    GPIO_OE_SET [
        ///Perform an atomic bit-set on GPIO_OE
        OE OFFSET(0) NUMBITS(30) []
    ],
    GPIO_OE_CLR [
        ///Perform an atomic bit-clear on GPIO_OE
        OE OFFSET(0) NUMBITS(30) []
    ],
];

const GPIO_BASE_ADDRESS: usize = 0x40014000;
const GPIO_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(GPIO_BASE_ADDRESS as *const GpioRegisters) };

const GPIO_PAD_BASE_ADDRESS: usize = 0x4001c000;
const GPIO_PAD_BASE: StaticRef<GpioPadRegisters> =
    unsafe { StaticRef::new(GPIO_PAD_BASE_ADDRESS as *const GpioPadRegisters) };

const SIO_BASE_ADDRESS: usize = 0xd0000000;
const SIO_BASE: StaticRef<SIORegisters> =
    unsafe { StaticRef::new(SIO_BASE_ADDRESS as *const SIORegisters) };

enum_from_primitive! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[repr(usize)]
    #[rustfmt::skip]
    pub enum RPGpio {
        GPIO0=0, GPIO1=1, GPIO2=2, GPIO3=3, GPIO4=4, GPIO5=5, GPIO6=6, GPIO7=7,
        GPIO8=8, GPIO9=9, GPIO10=10, GPIO11=11, GPIO12=12, GPIO13=13, GPIO14=14, GPIO15=15,
        GPIO16=16, GPIO17=17, GPIO18=18, GPIO19=19, GPIO20=20, GPIO21=21, GPIO22=22, GPIO23=23,
        GPIO24=24, GPIO25=25, GPIO26=26, GPIO27=27, GPIO28=28, GPIO29=29
    }
}
enum_from_primitive! {
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[repr(u32)]
    #[rustfmt::skip]

    pub enum GpioFunction {
       SPI = 1,
       UART = 2,
       I2C = 3,
       PWM = 4,
       SIO = 5,
       PIO0 = 6,
       PIO1 = 7,
       GPCK = 8,
       USB = 9,
       NULL = 0x1f
    }
}

pub struct RPGpioPin<'a> {
    pin: usize,
    client: OptionalCell<&'a dyn hil::gpio::Client>,
    gpio_registers: StaticRef<GpioRegisters>,
    gpio_pad_registers: StaticRef<GpioPadRegisters>,
    sio_registers: StaticRef<SIORegisters>,
}

impl<'a> RPGpioPin<'a> {
    pub const fn new(pin: RPGpio) -> RPGpioPin<'a> {
        RPGpioPin {
            pin: pin as usize,
            client: OptionalCell::empty(),
            gpio_registers: GPIO_BASE,
            gpio_pad_registers: GPIO_PAD_BASE,
            sio_registers: SIO_BASE,
        }
    }

    fn get_mode(&self) -> hil::gpio::Configuration {
        //TODO - read alternate function
        let pad_output_disable = !self.gpio_pad_registers.gpio_pad[self.pin].is_set(GPIO_PAD::OD);
        let pin_mask = 1 << self.pin;
        let sio_output_enable = (self.sio_registers.gpio_oe.read(GPIO_OE::OE) & pin_mask) != 0;

        match (pad_output_disable, sio_output_enable) {
            (true, true) => hil::gpio::Configuration::Output,
            (true, false) => hil::gpio::Configuration::Input,
            (false, _) => hil::gpio::Configuration::LowPower,
            _ => panic!("Invalid GPIO state mode"),
        }
    }

    fn read_pin(&self) -> bool {
        //TODO - read alternate function
        let value = self.sio_registers.gpio_out.read(GPIO_OUT::OUT) & (1 << self.pin);
        if value == 0 {
            false
        } else {
            true
        }
    }

    fn set_function(&self, f: GpioFunction) {
        self.gpio_registers.pin[self.pin]
            .ctrl
            .write(GPIOx_CTRL::FUNCSEL.val(f as u32));
    }

    fn get_pullup_pulldown(&self) -> hil::gpio::FloatingState {
        //TODO - read alternate function
        let pullup = self.gpio_pad_registers.gpio_pad[self.pin].read(GPIO_PAD::PUE);
        let pulldown = self.gpio_pad_registers.gpio_pad[self.pin].read(GPIO_PAD::PDE);

        match (pullup, pulldown) {
            (0, 0) => hil::gpio::FloatingState::PullNone,
            (0, 1) => hil::gpio::FloatingState::PullDown,
            (1, 0) => hil::gpio::FloatingState::PullUp,
            _ => panic!("Invalid GPIO floating state"),
        }
    }

    fn activate_pads(&self) {
        self.gpio_pad_registers.gpio_pad[self.pin].modify(GPIO_PAD::OD::CLEAR + GPIO_PAD::IE::SET);
    }
}

impl hil::gpio::Pin for RPGpioPin<'_> {}

impl hil::gpio::Configure for RPGpioPin<'_> {
    fn configuration(&self) -> hil::gpio::Configuration {
        self.get_mode()
    }
    /// Set output mode
    fn make_output(&self) -> hil::gpio::Configuration {
        self.set_function(GpioFunction::SIO);
        self.activate_pads();
        self.sio_registers.gpio_oe_set.set(1 << self.pin);
        self.get_mode()
    }
    /// Disable pad output
    fn disable_output(&self) -> hil::gpio::Configuration {
        self.set_function(GpioFunction::SIO);
        self.gpio_pad_registers.gpio_pad[self.pin].modify(GPIO_PAD::OD::SET);
        self.get_mode()
    }
    /// Set input mode
    fn make_input(&self) -> hil::gpio::Configuration {
        self.set_function(GpioFunction::SIO);
        self.activate_pads();
        self.sio_registers.gpio_oe_clr.set(1 << self.pin);
        self.get_mode()
    }
    /// Disable input mode, will set pin to output mode
    fn disable_input(&self) -> hil::gpio::Configuration {
        self.make_output()
    }
    fn deactivate_to_low_power(&self) {
        self.set_function(GpioFunction::SIO);
        self.gpio_pad_registers.gpio_pad[self.pin].modify(GPIO_PAD::OD::SET);
    }

    fn set_floating_state(&self, mode: hil::gpio::FloatingState) {
        match mode {
            hil::gpio::FloatingState::PullUp => self.gpio_pad_registers.gpio_pad[self.pin]
                .modify(GPIO_PAD::PUE::SET + GPIO_PAD::PDE::CLEAR),
            hil::gpio::FloatingState::PullDown => self.gpio_pad_registers.gpio_pad[self.pin]
                .modify(GPIO_PAD::PUE::CLEAR + GPIO_PAD::PDE::SET),
            hil::gpio::FloatingState::PullNone => self.gpio_pad_registers.gpio_pad[self.pin]
                .modify(GPIO_PAD::PUE::CLEAR + GPIO_PAD::PDE::CLEAR),
        }
    }

    fn floating_state(&self) -> hil::gpio::FloatingState {
        self.get_pullup_pulldown()
    }

    fn is_input(&self) -> bool {
        let mode = self.get_mode();
        match mode {
            hil::gpio::Configuration::Input => true,
            hil::gpio::Configuration::InputOutput => true,
            _ => false,
        }
    }

    fn is_output(&self) -> bool {
        let mode = self.get_mode();
        match mode {
            hil::gpio::Configuration::Output => true,
            hil::gpio::Configuration::InputOutput => true,
            _ => false,
        }
    }
}

impl hil::gpio::Output for RPGpioPin<'_> {
    fn set(&self) {
        // For performance this match might be skipped
        match self.get_mode() {
            hil::gpio::Configuration::Output | hil::gpio::Configuration::InputOutput => {
                self.sio_registers.gpio_out_set.set(1 << self.pin);
            }
            _ => {}
        }
    }

    fn clear(&self) {
        // For performance this match might be skipped
        match self.get_mode() {
            hil::gpio::Configuration::Output | hil::gpio::Configuration::InputOutput => {
                self.sio_registers.gpio_out_clr.set(1 << self.pin);
            }
            _ => {}
        }
    }

    fn toggle(&self) -> bool {
        // For performance this match might be skipped
        match self.get_mode() {
            hil::gpio::Configuration::Output | hil::gpio::Configuration::InputOutput => {
                self.sio_registers.gpio_out_xor.set(1 << self.pin);
            }
            _ => {}
        }
        self.read_pin()
    }
}

impl hil::gpio::Input for RPGpioPin<'_> {
    fn read(&self) -> bool {
        let value = self.sio_registers.gpio_in.read(GPIO_IN::IN) & (1 << self.pin);
        if value == 0 {
            false
        } else {
            true
        }
    }
}
