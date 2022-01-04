#![doc = "Peripheral access API for EOS-S3 microcontrollers (generated using svd2rust v0.20.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.20.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {
    fn SENSOR_GPIO();
    fn UART();
    fn TIMER();
    fn ADC_DONE();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 20] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: SENSOR_GPIO,
    },
    Vector { _reserved: 0 },
    Vector { _handler: UART },
    Vector { _handler: TIMER },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC_DONE },
];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "5 - Global GPIO interrupt"]
    SENSOR_GPIO = 5,
    #[doc = "7 - Global UART interrupt"]
    UART = 7,
    #[doc = "8 - Interrupt triggered when a timer counts down to 0. The status can be read and cleared (0x4000_4830\\[2\\]), and can be masked (0x4000_4834\\[2\\]
for Host), and (0x4000_4838\\[2\\]
for M4)."]
    TIMER = 8,
    #[doc = "19 - ADC Done interrupt"]
    ADC_DONE = 19,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "Selects source APB Master to SPI Master between M4/AP and Fabric"]
pub struct A1_REGS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for A1_REGS {}
impl A1_REGS {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const a1_regs::RegisterBlock = 0x4000_6000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const a1_regs::RegisterBlock {
        Self::PTR
    }
}
impl Deref for A1_REGS {
    type Target = a1_regs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for A1_REGS {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("A1_REGS").finish()
    }
}
#[doc = "Selects source APB Master to SPI Master between M4/AP and Fabric"]
pub mod a1_regs;
#[doc = "IO Multiplexing Control"]
pub struct IOMUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IOMUX {}
impl IOMUX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const iomux::RegisterBlock = 0x4000_4c00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const iomux::RegisterBlock {
        Self::PTR
    }
}
impl Deref for IOMUX {
    type Target = iomux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IOMUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IOMUX").finish()
    }
}
#[doc = "IO Multiplexing Control"]
pub mod iomux;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc::RegisterBlock = 0x4000_5a00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC").finish()
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc;
#[doc = "Analog IP block"]
pub struct AIP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AIP {}
impl AIP {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const aip::RegisterBlock = 0x4000_5400 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aip::RegisterBlock {
        Self::PTR
    }
}
impl Deref for AIP {
    type Target = aip::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for AIP {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AIP").finish()
    }
}
#[doc = "Analog IP block"]
pub mod aip;
#[doc = "Clock Reset Unit"]
pub struct CRU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRU {}
impl CRU {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const cru::RegisterBlock = 0x4000_4000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cru::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CRU {
    type Target = cru::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CRU {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRU").finish()
    }
}
#[doc = "Clock Reset Unit"]
pub mod cru;
#[doc = "MISC registers"]
pub struct MISC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MISC {}
impl MISC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const misc::RegisterBlock = 0x4000_5000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const misc::RegisterBlock {
        Self::PTR
    }
}
impl Deref for MISC {
    type Target = misc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for MISC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC").finish()
    }
}
#[doc = "MISC registers"]
pub mod misc;
#[doc = "TIMER"]
pub struct TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER {}
impl TIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer::RegisterBlock = 0x4001_3000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER {
    type Target = timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER").finish()
    }
}
#[doc = "TIMER"]
pub mod timer;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "A1_REGS"]
    pub A1_REGS: A1_REGS,
    #[doc = "IOMUX"]
    pub IOMUX: IOMUX,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "AIP"]
    pub AIP: AIP,
    #[doc = "CRU"]
    pub CRU: CRU,
    #[doc = "MISC"]
    pub MISC: MISC,
    #[doc = "TIMER"]
    pub TIMER: TIMER,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            A1_REGS: A1_REGS {
                _marker: PhantomData,
            },
            IOMUX: IOMUX {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            AIP: AIP {
                _marker: PhantomData,
            },
            CRU: CRU {
                _marker: PhantomData,
            },
            MISC: MISC {
                _marker: PhantomData,
            },
            TIMER: TIMER {
                _marker: PhantomData,
            },
        }
    }
}
