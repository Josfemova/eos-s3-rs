#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Current Value"]
    pub value: crate::Reg<value::VALUE_SPEC>,
    #[doc = "0x08 - Reload value. A write to this register sets the current value"]
    pub reload: crate::Reg<reload::RELOAD_SPEC>,
    #[doc = "0x0c - Timer interrupt. Write one to clear"]
    pub intstatus_intclear:
        crate::Reg<intstatus_intclear::INTSTATUS_INTCLEAR_SPEC>,
    _reserved4: [u8; 0x0fc0],
    #[doc = "0xfd0 - Peripheral ID register 4: \\[7:4\\]
Block count. \\[3:0\\]
jep106_c_code."]
    pub pid4: crate::Reg<pid4::PID4_SPEC>,
    #[doc = "0xfd4 - Peripheral ID register 5"]
    pub pid5: crate::Reg<pid5::PID5_SPEC>,
    #[doc = "0xfd8 - Peripheral ID register 6"]
    pub pid6: crate::Reg<pid6::PID6_SPEC>,
    #[doc = "0xfdc - Peripheral ID register 7"]
    pub pid7: crate::Reg<pid7::PID7_SPEC>,
    #[doc = "0xfe0 - Peripheral ID Register 0: \\[7:0\\]
Part Number\\[7:0\\]."]
    pub pid0: crate::Reg<pid0::PID0_SPEC>,
    #[doc = "0xfe4 - Peripheral ID register 1: \\[7:0\\]
jep106_id_3_0. \\[3:0\\]
Par number\\[11:8\\]."]
    pub pid1: crate::Reg<pid1::PID1_SPEC>,
    #[doc = "0xfe8 - Peripheral ID register 2: \\[7:4\\]
Revision. \\[3\\]
jedec_used. \\[2:0\\]
jep106_id_6_4."]
    pub pid2: crate::Reg<pid2::PID2_SPEC>,
    #[doc = "0xfec - Peripheral ID register 3: \\[7:4\\]
ECO revision number. \\[3:0\\]
Customer modification number."]
    pub pid3: crate::Reg<pid3::PID3_SPEC>,
    #[doc = "0xff0 - Component ID register 0"]
    pub cid0: crate::Reg<cid0::CID0_SPEC>,
    #[doc = "0xff4 - Component ID register 1"]
    pub cid1: crate::Reg<cid1::CID1_SPEC>,
    #[doc = "0xff8 - Component ID register 2"]
    pub cid2: crate::Reg<cid2::CID2_SPEC>,
    #[doc = "0xffc - Component ID register 3"]
    pub cid3: crate::Reg<cid3::CID3_SPEC>,
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Timer control register"]
pub mod ctrl;
#[doc = "VALUE register accessor: an alias for `Reg<VALUE_SPEC>`"]
pub type VALUE = crate::Reg<value::VALUE_SPEC>;
#[doc = "Current Value"]
pub mod value;
#[doc = "RELOAD register accessor: an alias for `Reg<RELOAD_SPEC>`"]
pub type RELOAD = crate::Reg<reload::RELOAD_SPEC>;
#[doc = "Reload value. A write to this register sets the current value"]
pub mod reload;
#[doc = "INTSTATUS_INTCLEAR register accessor: an alias for `Reg<INTSTATUS_INTCLEAR_SPEC>`"]
pub type INTSTATUS_INTCLEAR =
    crate::Reg<intstatus_intclear::INTSTATUS_INTCLEAR_SPEC>;
#[doc = "Timer interrupt. Write one to clear"]
pub mod intstatus_intclear;
#[doc = "PID4 register accessor: an alias for `Reg<PID4_SPEC>`"]
pub type PID4 = crate::Reg<pid4::PID4_SPEC>;
#[doc = "Peripheral ID register 4: \\[7:4\\]
Block count. \\[3:0\\]
jep106_c_code."]
pub mod pid4;
#[doc = "PID5 register accessor: an alias for `Reg<PID5_SPEC>`"]
pub type PID5 = crate::Reg<pid5::PID5_SPEC>;
#[doc = "Peripheral ID register 5"]
pub mod pid5;
#[doc = "PID6 register accessor: an alias for `Reg<PID6_SPEC>`"]
pub type PID6 = crate::Reg<pid6::PID6_SPEC>;
#[doc = "Peripheral ID register 6"]
pub mod pid6;
#[doc = "PID7 register accessor: an alias for `Reg<PID7_SPEC>`"]
pub type PID7 = crate::Reg<pid7::PID7_SPEC>;
#[doc = "Peripheral ID register 7"]
pub mod pid7;
#[doc = "PID0 register accessor: an alias for `Reg<PID0_SPEC>`"]
pub type PID0 = crate::Reg<pid0::PID0_SPEC>;
#[doc = "Peripheral ID Register 0: \\[7:0\\]
Part Number\\[7:0\\]."]
pub mod pid0;
#[doc = "PID1 register accessor: an alias for `Reg<PID1_SPEC>`"]
pub type PID1 = crate::Reg<pid1::PID1_SPEC>;
#[doc = "Peripheral ID register 1: \\[7:0\\]
jep106_id_3_0. \\[3:0\\]
Par number\\[11:8\\]."]
pub mod pid1;
#[doc = "PID2 register accessor: an alias for `Reg<PID2_SPEC>`"]
pub type PID2 = crate::Reg<pid2::PID2_SPEC>;
#[doc = "Peripheral ID register 2: \\[7:4\\]
Revision. \\[3\\]
jedec_used. \\[2:0\\]
jep106_id_6_4."]
pub mod pid2;
#[doc = "PID3 register accessor: an alias for `Reg<PID3_SPEC>`"]
pub type PID3 = crate::Reg<pid3::PID3_SPEC>;
#[doc = "Peripheral ID register 3: \\[7:4\\]
ECO revision number. \\[3:0\\]
Customer modification number."]
pub mod pid3;
#[doc = "CID0 register accessor: an alias for `Reg<CID0_SPEC>`"]
pub type CID0 = crate::Reg<cid0::CID0_SPEC>;
#[doc = "Component ID register 0"]
pub mod cid0;
#[doc = "CID1 register accessor: an alias for `Reg<CID1_SPEC>`"]
pub type CID1 = crate::Reg<cid1::CID1_SPEC>;
#[doc = "Component ID register 1"]
pub mod cid1;
#[doc = "CID2 register accessor: an alias for `Reg<CID2_SPEC>`"]
pub type CID2 = crate::Reg<cid2::CID2_SPEC>;
#[doc = "Component ID register 2"]
pub mod cid2;
#[doc = "CID3 register accessor: an alias for `Reg<CID3_SPEC>`"]
pub type CID3 = crate::Reg<cid3::CID3_SPEC>;
#[doc = "Component ID register 3"]
pub mod cid3;
