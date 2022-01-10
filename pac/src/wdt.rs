#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The WDOGLOAD Register contains the value from which the counter is to decrement. When this register is written to, the count is immediately restarted from the new value. The minimum valid value for WDOGLOAD is 1."]
    pub wdogload: crate::Reg<wdogload::WDOGLOAD_SPEC>,
    #[doc = "0x04 - The WDOGVALUE Register gives the current value of the decrementing counter."]
    pub wdogvalue: crate::Reg<wdogvalue::WDOGVALUE_SPEC>,
    #[doc = "0x08 - Control register for the WatchDog timer"]
    pub wdogcontrol: crate::Reg<wdogcontrol::WDOGCONTROL_SPEC>,
    #[doc = "0x0c - A write of any value to the WDOGINTCLR Register clears the watchdog interrupt, and reloads the counter from the value in WDOGLOAD."]
    pub wdogintclr: crate::Reg<wdogintclr::WDOGINTCLR_SPEC>,
    #[doc = "0x10 - The WDOGRIS Register indicates the raw interrupt status from the counter. This value is ANDed with the interrupt enable bit from the control register to create the masked interrupt, that is passed to the interrupt output pin."]
    pub wdogris: crate::Reg<wdogris::WDOGRIS_SPEC>,
    #[doc = "0x14 - The WDOGMIS Register indicates the masked interrupt status from the counter. This value is the logical AND of the raw interrupt status with the INTEN bit from the control register, and is the same value that is passed to the interrupt output pin. Enabled interrupt status from the counter."]
    pub wdogmis: crate::Reg<wdogmis::WDOGMIS_SPEC>,
    _reserved6: [u8; 0x0be8],
    #[doc = "0xc00 - The WDOGLOCK Register diables write accesses to all other registers. This is to prevent rogue software from diabling the watchdog functionality. Writing a value 0x1ACCE551 enables write access to all other registers. Writing any other value disables write accesses."]
    pub wdoglock: crate::Reg<wdoglock::WDOGLOCK_SPEC>,
    _reserved7: [u8; 0x02fc],
    #[doc = "0xf00 - The WDOGITCR Register enables integration test mode. When in this more, the test output register directly controls the masted interrup output, WDOGINT, and reset output, WDOGRES. Integration Test mode Enable \\[0\\]
When set HIGH, places th watchdog into integration test mode."]
    pub wdogitcr: crate::Reg<wdogitcr::WDOGITCR_SPEC>,
    #[doc = "0xf04 - Watchdog Integration Test Output Set Register When the WDOGITOP Register is in integration test mode, the values in this register directly drive the enabled interrupt output and reset output."]
    pub wdogitop: crate::Reg<wdogitop::WDOGITOP_SPEC>,
    _reserved9: [u8; 0xc8],
    #[doc = "0xfd0 - Peripheral ID Register 4: \\[7:4\\]
Block count. \\[3:0\\]
jep106_c_code."]
    pub wdogperiphid4: crate::Reg<wdogperiphid4::WDOGPERIPHID4_SPEC>,
    #[doc = "0xfd4 - Peripheral ID Register 5."]
    pub wdogperiphid5: crate::Reg<wdogperiphid5::WDOGPERIPHID5_SPEC>,
    #[doc = "0xfd8 - Peripheral ID Register 6."]
    pub wdogperiphid6: crate::Reg<wdogperiphid6::WDOGPERIPHID6_SPEC>,
    #[doc = "0xfdc - Peripheral ID Register 7."]
    pub wdogperiphid7: crate::Reg<wdogperiphid7::WDOGPERIPHID7_SPEC>,
    #[doc = "0xfe0 - Periperhal ID Register 0. \\[7:0\\]
Part number\\[7:0\\]."]
    pub wdogperiphid0: crate::Reg<wdogperiphid0::WDOGPERIPHID0_SPEC>,
    #[doc = "0xfe4 - Peripheral ID Register 1. \\[7:4\\]
jep106_id_3_0. \\[3:0\\]
Part number \\[11:8\\]."]
    pub wdogperiphid1: crate::Reg<wdogperiphid1::WDOGPERIPHID1_SPEC>,
    #[doc = "0xfe8 - Peripheral ID Register 2. \\[7:4\\]
Revision. \\[3\\]
jedec_used. \\[2:0\\]
jep106_id_6_4."]
    pub wdogperiphid2: crate::Reg<wdogperiphid2::WDOGPERIPHID2_SPEC>,
    #[doc = "0xfec - Peripherial ID Register 3. \\[7:4\\]
ECO revision number. \\[3:0\\]
Customer modification number."]
    pub wdogperiphid3: crate::Reg<wdogperiphid3::WDOGPERIPHID3_SPEC>,
    #[doc = "0xff0 - Component ID Register 0."]
    pub wdogpcellid0: crate::Reg<wdogpcellid0::WDOGPCELLID0_SPEC>,
    #[doc = "0xff4 - Component ID Register 1."]
    pub wdogpcellid1: crate::Reg<wdogpcellid1::WDOGPCELLID1_SPEC>,
    #[doc = "0xff8 - Component ID Register 2."]
    pub wdogpcellid2: crate::Reg<wdogpcellid2::WDOGPCELLID2_SPEC>,
    #[doc = "0xffc - Component ID Register 3."]
    pub wdogpcellid3: crate::Reg<wdogpcellid3::WDOGPCELLID3_SPEC>,
}
#[doc = "WDOGLOAD register accessor: an alias for `Reg<WDOGLOAD_SPEC>`"]
pub type WDOGLOAD = crate::Reg<wdogload::WDOGLOAD_SPEC>;
#[doc = "The WDOGLOAD Register contains the value from which the counter is to decrement. When this register is written to, the count is immediately restarted from the new value. The minimum valid value for WDOGLOAD is 1."]
pub mod wdogload;
#[doc = "WDOGVALUE register accessor: an alias for `Reg<WDOGVALUE_SPEC>`"]
pub type WDOGVALUE = crate::Reg<wdogvalue::WDOGVALUE_SPEC>;
#[doc = "The WDOGVALUE Register gives the current value of the decrementing counter."]
pub mod wdogvalue;
#[doc = "WDOGCONTROL register accessor: an alias for `Reg<WDOGCONTROL_SPEC>`"]
pub type WDOGCONTROL = crate::Reg<wdogcontrol::WDOGCONTROL_SPEC>;
#[doc = "Control register for the WatchDog timer"]
pub mod wdogcontrol;
#[doc = "WDOGINTCLR register accessor: an alias for `Reg<WDOGINTCLR_SPEC>`"]
pub type WDOGINTCLR = crate::Reg<wdogintclr::WDOGINTCLR_SPEC>;
#[doc = "A write of any value to the WDOGINTCLR Register clears the watchdog interrupt, and reloads the counter from the value in WDOGLOAD."]
pub mod wdogintclr;
#[doc = "WDOGRIS register accessor: an alias for `Reg<WDOGRIS_SPEC>`"]
pub type WDOGRIS = crate::Reg<wdogris::WDOGRIS_SPEC>;
#[doc = "The WDOGRIS Register indicates the raw interrupt status from the counter. This value is ANDed with the interrupt enable bit from the control register to create the masked interrupt, that is passed to the interrupt output pin."]
pub mod wdogris;
#[doc = "WDOGMIS register accessor: an alias for `Reg<WDOGMIS_SPEC>`"]
pub type WDOGMIS = crate::Reg<wdogmis::WDOGMIS_SPEC>;
#[doc = "The WDOGMIS Register indicates the masked interrupt status from the counter. This value is the logical AND of the raw interrupt status with the INTEN bit from the control register, and is the same value that is passed to the interrupt output pin. Enabled interrupt status from the counter."]
pub mod wdogmis;
#[doc = "WDOGLOCK register accessor: an alias for `Reg<WDOGLOCK_SPEC>`"]
pub type WDOGLOCK = crate::Reg<wdoglock::WDOGLOCK_SPEC>;
#[doc = "The WDOGLOCK Register diables write accesses to all other registers. This is to prevent rogue software from diabling the watchdog functionality. Writing a value 0x1ACCE551 enables write access to all other registers. Writing any other value disables write accesses."]
pub mod wdoglock;
#[doc = "WDOGITCR register accessor: an alias for `Reg<WDOGITCR_SPEC>`"]
pub type WDOGITCR = crate::Reg<wdogitcr::WDOGITCR_SPEC>;
#[doc = "The WDOGITCR Register enables integration test mode. When in this more, the test output register directly controls the masted interrup output, WDOGINT, and reset output, WDOGRES. Integration Test mode Enable \\[0\\]
When set HIGH, places th watchdog into integration test mode."]
pub mod wdogitcr;
#[doc = "WDOGITOP register accessor: an alias for `Reg<WDOGITOP_SPEC>`"]
pub type WDOGITOP = crate::Reg<wdogitop::WDOGITOP_SPEC>;
#[doc = "Watchdog Integration Test Output Set Register When the WDOGITOP Register is in integration test mode, the values in this register directly drive the enabled interrupt output and reset output."]
pub mod wdogitop;
#[doc = "WDOGPERIPHID4 register accessor: an alias for `Reg<WDOGPERIPHID4_SPEC>`"]
pub type WDOGPERIPHID4 = crate::Reg<wdogperiphid4::WDOGPERIPHID4_SPEC>;
#[doc = "Peripheral ID Register 4: \\[7:4\\]
Block count. \\[3:0\\]
jep106_c_code."]
pub mod wdogperiphid4;
#[doc = "WDOGPERIPHID5 register accessor: an alias for `Reg<WDOGPERIPHID5_SPEC>`"]
pub type WDOGPERIPHID5 = crate::Reg<wdogperiphid5::WDOGPERIPHID5_SPEC>;
#[doc = "Peripheral ID Register 5."]
pub mod wdogperiphid5;
#[doc = "WDOGPERIPHID6 register accessor: an alias for `Reg<WDOGPERIPHID6_SPEC>`"]
pub type WDOGPERIPHID6 = crate::Reg<wdogperiphid6::WDOGPERIPHID6_SPEC>;
#[doc = "Peripheral ID Register 6."]
pub mod wdogperiphid6;
#[doc = "WDOGPERIPHID7 register accessor: an alias for `Reg<WDOGPERIPHID7_SPEC>`"]
pub type WDOGPERIPHID7 = crate::Reg<wdogperiphid7::WDOGPERIPHID7_SPEC>;
#[doc = "Peripheral ID Register 7."]
pub mod wdogperiphid7;
#[doc = "WDOGPERIPHID0 register accessor: an alias for `Reg<WDOGPERIPHID0_SPEC>`"]
pub type WDOGPERIPHID0 = crate::Reg<wdogperiphid0::WDOGPERIPHID0_SPEC>;
#[doc = "Periperhal ID Register 0. \\[7:0\\]
Part number\\[7:0\\]."]
pub mod wdogperiphid0;
#[doc = "WDOGPERIPHID1 register accessor: an alias for `Reg<WDOGPERIPHID1_SPEC>`"]
pub type WDOGPERIPHID1 = crate::Reg<wdogperiphid1::WDOGPERIPHID1_SPEC>;
#[doc = "Peripheral ID Register 1. \\[7:4\\]
jep106_id_3_0. \\[3:0\\]
Part number \\[11:8\\]."]
pub mod wdogperiphid1;
#[doc = "WDOGPERIPHID2 register accessor: an alias for `Reg<WDOGPERIPHID2_SPEC>`"]
pub type WDOGPERIPHID2 = crate::Reg<wdogperiphid2::WDOGPERIPHID2_SPEC>;
#[doc = "Peripheral ID Register 2. \\[7:4\\]
Revision. \\[3\\]
jedec_used. \\[2:0\\]
jep106_id_6_4."]
pub mod wdogperiphid2;
#[doc = "WDOGPERIPHID3 register accessor: an alias for `Reg<WDOGPERIPHID3_SPEC>`"]
pub type WDOGPERIPHID3 = crate::Reg<wdogperiphid3::WDOGPERIPHID3_SPEC>;
#[doc = "Peripherial ID Register 3. \\[7:4\\]
ECO revision number. \\[3:0\\]
Customer modification number."]
pub mod wdogperiphid3;
#[doc = "WDOGPCELLID0 register accessor: an alias for `Reg<WDOGPCELLID0_SPEC>`"]
pub type WDOGPCELLID0 = crate::Reg<wdogpcellid0::WDOGPCELLID0_SPEC>;
#[doc = "Component ID Register 0."]
pub mod wdogpcellid0;
#[doc = "WDOGPCELLID1 register accessor: an alias for `Reg<WDOGPCELLID1_SPEC>`"]
pub type WDOGPCELLID1 = crate::Reg<wdogpcellid1::WDOGPCELLID1_SPEC>;
#[doc = "Component ID Register 1."]
pub mod wdogpcellid1;
#[doc = "WDOGPCELLID2 register accessor: an alias for `Reg<WDOGPCELLID2_SPEC>`"]
pub type WDOGPCELLID2 = crate::Reg<wdogpcellid2::WDOGPCELLID2_SPEC>;
#[doc = "Component ID Register 2."]
pub mod wdogpcellid2;
#[doc = "WDOGPCELLID3 register accessor: an alias for `Reg<WDOGPCELLID3_SPEC>`"]
pub type WDOGPCELLID3 = crate::Reg<wdogpcellid3::WDOGPCELLID3_SPEC>;
#[doc = "Component ID Register 3."]
pub mod wdogpcellid3;
