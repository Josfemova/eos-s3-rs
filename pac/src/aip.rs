#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - RTC control register 1"]
    pub rtc_ctrl_1: crate::Reg<rtc_ctrl_1::RTC_CTRL_1_SPEC>,
    #[doc = "0x08 - RTC control register 2"]
    pub rtc_ctrl_2: crate::Reg<rtc_ctrl_2::RTC_CTRL_2_SPEC>,
    #[doc = "0x0c - RTC control register 3"]
    pub rtc_ctrl_3: crate::Reg<rtc_ctrl_3::RTC_CTRL_3_SPEC>,
    #[doc = "0x10 - RTC Control register 4"]
    pub rtc_ctrl_4: crate::Reg<rtc_ctrl_4::RTC_CTRL_4_SPEC>,
    #[doc = "0x14 - RTC control register 5"]
    pub rtc_ctrl_5: crate::Reg<rtc_ctrl_5::RTC_CTRL_5_SPEC>,
    #[doc = "0x18 - RTC control register 6"]
    pub rtc_ctrl_6: crate::Reg<rtc_ctrl_6::RTC_CTRL_6_SPEC>,
    #[doc = "0x1c - RTC control register 7"]
    pub rtc_ctrl_7: crate::Reg<rtc_ctrl_7::RTC_CTRL_7_SPEC>,
    #[doc = "0x20 - Incomplete information. Probably related to AIP RTC hardware status"]
    pub rtc_sta_0: crate::Reg<rtc_sta_0::RTC_STA_0_SPEC>,
    #[doc = "0x24 - Incomplete information. Probably related to AIP RTC hardware status"]
    pub rtc_sta_1: crate::Reg<rtc_sta_1::RTC_STA_1_SPEC>,
    _reserved9: [u8; 0x58],
    #[doc = "0x80 - Oscilator control register 0"]
    pub osc_ctrl_0: crate::Reg<osc_ctrl_0::OSC_CTRL_0_SPEC>,
    #[doc = "0x84 - Oscilator control register 1"]
    pub osc_ctrl_1: crate::Reg<osc_ctrl_1::OSC_CTRL_1_SPEC>,
    #[doc = "0x88 - Oscilator control register 2"]
    pub osc_ctrl_2: crate::Reg<osc_ctrl_2::OSC_CTRL_2_SPEC>,
    #[doc = "0x8c - Oscilator control register 3"]
    pub osc_ctrl_3: crate::Reg<osc_ctrl_3::OSC_CTRL_3_SPEC>,
    #[doc = "0x90 - Oscilator control register 4"]
    pub osc_ctrl_4: crate::Reg<osc_ctrl_4::OSC_CTRL_4_SPEC>,
    #[doc = "0x94 - Oscilator control register 5"]
    pub osc_ctrl_5: crate::Reg<osc_ctrl_5::OSC_CTRL_5_SPEC>,
    #[doc = "0x98 - Oscilator control register 6"]
    pub osc_ctrl_6: crate::Reg<osc_ctrl_6::OSC_CTRL_6_SPEC>,
    #[doc = "0x9c - Oscilator control register 7"]
    pub osc_ctrl_7: crate::Reg<osc_ctrl_7::OSC_CTRL_7_SPEC>,
    #[doc = "0xa0 - Contains information about oscilator status"]
    pub osc_sta_0: crate::Reg<osc_sta_0::OSC_STA_0_SPEC>,
    #[doc = "0xa4 - Contains information about oscilator status"]
    pub osc_sta_1: crate::Reg<osc_sta_1::OSC_STA_1_SPEC>,
    _reserved19: [u8; 0x58],
    #[doc = "0x100 - APC control register 0"]
    pub apc_ctrl_0: crate::Reg<apc_ctrl_0::APC_CTRL_0_SPEC>,
    #[doc = "0x104 - APC control register 1"]
    pub apc_ctrl_1: crate::Reg<apc_ctrl_1::APC_CTRL_1_SPEC>,
    #[doc = "0x108 - APC control register 2"]
    pub apc_ctrl_2: crate::Reg<apc_ctrl_2::APC_CTRL_2_SPEC>,
    #[doc = "0x10c - APC control register 3 // Reserved"]
    pub apc_ctrl_3: crate::Reg<apc_ctrl_3::APC_CTRL_3_SPEC>,
    #[doc = "0x110 - APC control register 4 // Reserved"]
    pub apc_ctrl_4: crate::Reg<apc_ctrl_4::APC_CTRL_4_SPEC>,
    #[doc = "0x114 - APC control register 5 // Reserved"]
    pub apc_ctrl_5: crate::Reg<apc_ctrl_5::APC_CTRL_5_SPEC>,
    #[doc = "0x118 - APC control register 6 // Reserved"]
    pub apc_ctrl_6: crate::Reg<apc_ctrl_6::APC_CTRL_6_SPEC>,
    #[doc = "0x11c - APC control register 7 // Reserved"]
    pub apc_ctrl_7: crate::Reg<apc_ctrl_7::APC_CTRL_7_SPEC>,
    #[doc = "0x120 - APC status register 0"]
    pub apc_sta_0: crate::Reg<apc_sta_0::APC_STA_0_SPEC>,
    #[doc = "0x124 - APC status register 1"]
    pub apc_sta_1: crate::Reg<apc_sta_1::APC_STA_1_SPEC>,
    _reserved29: [u8; 0x58],
    #[doc = "0x180 - Ring oscilator control register"]
    pub ring_osc: crate::Reg<ring_osc::RING_OSC_SPEC>,
    _reserved30: [u8; 0x7c],
    #[doc = "0x200 - LDO_30 control register 0"]
    pub ldo_30_ctrl_0: crate::Reg<ldo_30_ctrl_0::LDO_30_CTRL_0_SPEC>,
    #[doc = "0x204 - LDO_30 control register 1"]
    pub ldo_30_ctrl_1: crate::Reg<ldo_30_ctrl_1::LDO_30_CTRL_1_SPEC>,
    _reserved32: [u8; 0x08],
    #[doc = "0x210 - LDO_50 control register 0"]
    pub ldo_50_ctrl_0: crate::Reg<ldo_50_ctrl_0::LDO_50_CTRL_0_SPEC>,
    #[doc = "0x214 - LDO_50 control register 1"]
    pub ldo_50_ctrl_1: crate::Reg<ldo_50_ctrl_1::LDO_50_CTRL_1_SPEC>,
}
#[doc = "RTC_CTRL_1 register accessor: an alias for `Reg<RTC_CTRL_1_SPEC>`"]
pub type RTC_CTRL_1 = crate::Reg<rtc_ctrl_1::RTC_CTRL_1_SPEC>;
#[doc = "RTC control register 1"]
pub mod rtc_ctrl_1;
#[doc = "RTC_CTRL_2 register accessor: an alias for `Reg<RTC_CTRL_2_SPEC>`"]
pub type RTC_CTRL_2 = crate::Reg<rtc_ctrl_2::RTC_CTRL_2_SPEC>;
#[doc = "RTC control register 2"]
pub mod rtc_ctrl_2;
#[doc = "RTC_CTRL_3 register accessor: an alias for `Reg<RTC_CTRL_3_SPEC>`"]
pub type RTC_CTRL_3 = crate::Reg<rtc_ctrl_3::RTC_CTRL_3_SPEC>;
#[doc = "RTC control register 3"]
pub mod rtc_ctrl_3;
#[doc = "RTC_CTRL_4 register accessor: an alias for `Reg<RTC_CTRL_4_SPEC>`"]
pub type RTC_CTRL_4 = crate::Reg<rtc_ctrl_4::RTC_CTRL_4_SPEC>;
#[doc = "RTC Control register 4"]
pub mod rtc_ctrl_4;
#[doc = "RTC_CTRL_5 register accessor: an alias for `Reg<RTC_CTRL_5_SPEC>`"]
pub type RTC_CTRL_5 = crate::Reg<rtc_ctrl_5::RTC_CTRL_5_SPEC>;
#[doc = "RTC control register 5"]
pub mod rtc_ctrl_5;
#[doc = "RTC_CTRL_6 register accessor: an alias for `Reg<RTC_CTRL_6_SPEC>`"]
pub type RTC_CTRL_6 = crate::Reg<rtc_ctrl_6::RTC_CTRL_6_SPEC>;
#[doc = "RTC control register 6"]
pub mod rtc_ctrl_6;
#[doc = "RTC_CTRL_7 register accessor: an alias for `Reg<RTC_CTRL_7_SPEC>`"]
pub type RTC_CTRL_7 = crate::Reg<rtc_ctrl_7::RTC_CTRL_7_SPEC>;
#[doc = "RTC control register 7"]
pub mod rtc_ctrl_7;
#[doc = "RTC_STA_0 register accessor: an alias for `Reg<RTC_STA_0_SPEC>`"]
pub type RTC_STA_0 = crate::Reg<rtc_sta_0::RTC_STA_0_SPEC>;
#[doc = "Incomplete information. Probably related to AIP RTC hardware status"]
pub mod rtc_sta_0;
#[doc = "RTC_STA_1 register accessor: an alias for `Reg<RTC_STA_1_SPEC>`"]
pub type RTC_STA_1 = crate::Reg<rtc_sta_1::RTC_STA_1_SPEC>;
#[doc = "Incomplete information. Probably related to AIP RTC hardware status"]
pub mod rtc_sta_1;
#[doc = "OSC_CTRL_0 register accessor: an alias for `Reg<OSC_CTRL_0_SPEC>`"]
pub type OSC_CTRL_0 = crate::Reg<osc_ctrl_0::OSC_CTRL_0_SPEC>;
#[doc = "Oscilator control register 0"]
pub mod osc_ctrl_0;
#[doc = "OSC_CTRL_1 register accessor: an alias for `Reg<OSC_CTRL_1_SPEC>`"]
pub type OSC_CTRL_1 = crate::Reg<osc_ctrl_1::OSC_CTRL_1_SPEC>;
#[doc = "Oscilator control register 1"]
pub mod osc_ctrl_1;
#[doc = "OSC_CTRL_2 register accessor: an alias for `Reg<OSC_CTRL_2_SPEC>`"]
pub type OSC_CTRL_2 = crate::Reg<osc_ctrl_2::OSC_CTRL_2_SPEC>;
#[doc = "Oscilator control register 2"]
pub mod osc_ctrl_2;
#[doc = "OSC_CTRL_3 register accessor: an alias for `Reg<OSC_CTRL_3_SPEC>`"]
pub type OSC_CTRL_3 = crate::Reg<osc_ctrl_3::OSC_CTRL_3_SPEC>;
#[doc = "Oscilator control register 3"]
pub mod osc_ctrl_3;
#[doc = "OSC_CTRL_4 register accessor: an alias for `Reg<OSC_CTRL_4_SPEC>`"]
pub type OSC_CTRL_4 = crate::Reg<osc_ctrl_4::OSC_CTRL_4_SPEC>;
#[doc = "Oscilator control register 4"]
pub mod osc_ctrl_4;
#[doc = "OSC_CTRL_5 register accessor: an alias for `Reg<OSC_CTRL_5_SPEC>`"]
pub type OSC_CTRL_5 = crate::Reg<osc_ctrl_5::OSC_CTRL_5_SPEC>;
#[doc = "Oscilator control register 5"]
pub mod osc_ctrl_5;
#[doc = "OSC_CTRL_6 register accessor: an alias for `Reg<OSC_CTRL_6_SPEC>`"]
pub type OSC_CTRL_6 = crate::Reg<osc_ctrl_6::OSC_CTRL_6_SPEC>;
#[doc = "Oscilator control register 6"]
pub mod osc_ctrl_6;
#[doc = "OSC_CTRL_7 register accessor: an alias for `Reg<OSC_CTRL_7_SPEC>`"]
pub type OSC_CTRL_7 = crate::Reg<osc_ctrl_7::OSC_CTRL_7_SPEC>;
#[doc = "Oscilator control register 7"]
pub mod osc_ctrl_7;
#[doc = "OSC_STA_0 register accessor: an alias for `Reg<OSC_STA_0_SPEC>`"]
pub type OSC_STA_0 = crate::Reg<osc_sta_0::OSC_STA_0_SPEC>;
#[doc = "Contains information about oscilator status"]
pub mod osc_sta_0;
#[doc = "OSC_STA_1 register accessor: an alias for `Reg<OSC_STA_1_SPEC>`"]
pub type OSC_STA_1 = crate::Reg<osc_sta_1::OSC_STA_1_SPEC>;
#[doc = "Contains information about oscilator status"]
pub mod osc_sta_1;
#[doc = "APC_CTRL_0 register accessor: an alias for `Reg<APC_CTRL_0_SPEC>`"]
pub type APC_CTRL_0 = crate::Reg<apc_ctrl_0::APC_CTRL_0_SPEC>;
#[doc = "APC control register 0"]
pub mod apc_ctrl_0;
#[doc = "APC_CTRL_1 register accessor: an alias for `Reg<APC_CTRL_1_SPEC>`"]
pub type APC_CTRL_1 = crate::Reg<apc_ctrl_1::APC_CTRL_1_SPEC>;
#[doc = "APC control register 1"]
pub mod apc_ctrl_1;
#[doc = "APC_CTRL_2 register accessor: an alias for `Reg<APC_CTRL_2_SPEC>`"]
pub type APC_CTRL_2 = crate::Reg<apc_ctrl_2::APC_CTRL_2_SPEC>;
#[doc = "APC control register 2"]
pub mod apc_ctrl_2;
#[doc = "APC_CTRL_3 register accessor: an alias for `Reg<APC_CTRL_3_SPEC>`"]
pub type APC_CTRL_3 = crate::Reg<apc_ctrl_3::APC_CTRL_3_SPEC>;
#[doc = "APC control register 3 // Reserved"]
pub mod apc_ctrl_3;
#[doc = "APC_CTRL_4 register accessor: an alias for `Reg<APC_CTRL_4_SPEC>`"]
pub type APC_CTRL_4 = crate::Reg<apc_ctrl_4::APC_CTRL_4_SPEC>;
#[doc = "APC control register 4 // Reserved"]
pub mod apc_ctrl_4;
#[doc = "APC_CTRL_5 register accessor: an alias for `Reg<APC_CTRL_5_SPEC>`"]
pub type APC_CTRL_5 = crate::Reg<apc_ctrl_5::APC_CTRL_5_SPEC>;
#[doc = "APC control register 5 // Reserved"]
pub mod apc_ctrl_5;
#[doc = "APC_CTRL_6 register accessor: an alias for `Reg<APC_CTRL_6_SPEC>`"]
pub type APC_CTRL_6 = crate::Reg<apc_ctrl_6::APC_CTRL_6_SPEC>;
#[doc = "APC control register 6 // Reserved"]
pub mod apc_ctrl_6;
#[doc = "APC_CTRL_7 register accessor: an alias for `Reg<APC_CTRL_7_SPEC>`"]
pub type APC_CTRL_7 = crate::Reg<apc_ctrl_7::APC_CTRL_7_SPEC>;
#[doc = "APC control register 7 // Reserved"]
pub mod apc_ctrl_7;
#[doc = "APC_STA_0 register accessor: an alias for `Reg<APC_STA_0_SPEC>`"]
pub type APC_STA_0 = crate::Reg<apc_sta_0::APC_STA_0_SPEC>;
#[doc = "APC status register 0"]
pub mod apc_sta_0;
#[doc = "APC_STA_1 register accessor: an alias for `Reg<APC_STA_1_SPEC>`"]
pub type APC_STA_1 = crate::Reg<apc_sta_1::APC_STA_1_SPEC>;
#[doc = "APC status register 1"]
pub mod apc_sta_1;
#[doc = "RING_OSC register accessor: an alias for `Reg<RING_OSC_SPEC>`"]
pub type RING_OSC = crate::Reg<ring_osc::RING_OSC_SPEC>;
#[doc = "Ring oscilator control register"]
pub mod ring_osc;
#[doc = "LDO_30_CTRL_0 register accessor: an alias for `Reg<LDO_30_CTRL_0_SPEC>`"]
pub type LDO_30_CTRL_0 = crate::Reg<ldo_30_ctrl_0::LDO_30_CTRL_0_SPEC>;
#[doc = "LDO_30 control register 0"]
pub mod ldo_30_ctrl_0;
#[doc = "LDO_30_CTRL_1 register accessor: an alias for `Reg<LDO_30_CTRL_1_SPEC>`"]
pub type LDO_30_CTRL_1 = crate::Reg<ldo_30_ctrl_1::LDO_30_CTRL_1_SPEC>;
#[doc = "LDO_30 control register 1"]
pub mod ldo_30_ctrl_1;
#[doc = "LDO_50_CTRL_0 register accessor: an alias for `Reg<LDO_50_CTRL_0_SPEC>`"]
pub type LDO_50_CTRL_0 = crate::Reg<ldo_50_ctrl_0::LDO_50_CTRL_0_SPEC>;
#[doc = "LDO_50 control register 0"]
pub mod ldo_50_ctrl_0;
#[doc = "LDO_50_CTRL_1 register accessor: an alias for `Reg<LDO_50_CTRL_1_SPEC>`"]
pub type LDO_50_CTRL_1 = crate::Reg<ldo_50_ctrl_1::LDO_50_CTRL_1_SPEC>;
#[doc = "LDO_50 control register 1"]
pub mod ldo_50_ctrl_1;
