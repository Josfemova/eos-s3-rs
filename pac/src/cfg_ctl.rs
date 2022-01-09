#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Fabric Configuration Control Register"]
    pub cfg_ctl: crate::Reg<cfg_ctl::CFG_CTL_SPEC>,
    #[doc = "0x04 - Maximum Bit Length Count: ARM firmware/software sets this register 1'b1 to set the Maximum Bit Line Count"]
    pub max_bl_cnt: crate::Reg<max_bl_cnt::MAX_BL_CNT_SPEC>,
    #[doc = "0x08 - Maximum Word Length Count: ARM firmware/software sets this register 1'b1 to set the Maximum Word Line Count"]
    pub max_wl_cnt: crate::Reg<max_wl_cnt::MAX_WL_CNT_SPEC>,
    _reserved3: [u8; 0x0ff0],
    #[doc = "0xffc - Configuration Data: ARM firmware/software Access this register to Read/Write the configuration bit cells."]
    pub cfg_data: crate::Reg<cfg_data::CFG_DATA_SPEC>,
    _reserved4: [u8; 0x3000],
    #[doc = "0x4000 - RAMFIFO0 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO0. From 0x8000 to 0x8FFC."]
    pub ramfifo0: crate::Reg<ramfifo0::RAMFIFO0_SPEC>,
    _reserved5: [u8; 0x0ffc],
    #[doc = "0x5000 - RAMFIFO1 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO1. From 0x9000 to 0x9FFC."]
    pub ramfifo1: crate::Reg<ramfifo1::RAMFIFO1_SPEC>,
    _reserved6: [u8; 0x0ffc],
    #[doc = "0x6000 - RAMFIFO2 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO2. From 0xA000 to 0xAFFC."]
    pub ramfifo2: crate::Reg<ramfifo2::RAMFIFO2_SPEC>,
    _reserved7: [u8; 0x0ffc],
    #[doc = "0x7000 - RAMFIFO3 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO3. From 0xB000 to 0xBFFC."]
    pub ramfifo3: crate::Reg<ramfifo3::RAMFIFO3_SPEC>,
}
#[doc = "CFG_CTL register accessor: an alias for `Reg<CFG_CTL_SPEC>`"]
pub type CFG_CTL = crate::Reg<cfg_ctl::CFG_CTL_SPEC>;
#[doc = "Fabric Configuration Control Register"]
pub mod cfg_ctl;
#[doc = "MAX_BL_CNT register accessor: an alias for `Reg<MAX_BL_CNT_SPEC>`"]
pub type MAX_BL_CNT = crate::Reg<max_bl_cnt::MAX_BL_CNT_SPEC>;
#[doc = "Maximum Bit Length Count: ARM firmware/software sets this register 1'b1 to set the Maximum Bit Line Count"]
pub mod max_bl_cnt;
#[doc = "MAX_WL_CNT register accessor: an alias for `Reg<MAX_WL_CNT_SPEC>`"]
pub type MAX_WL_CNT = crate::Reg<max_wl_cnt::MAX_WL_CNT_SPEC>;
#[doc = "Maximum Word Length Count: ARM firmware/software sets this register 1'b1 to set the Maximum Word Line Count"]
pub mod max_wl_cnt;
#[doc = "CFG_DATA register accessor: an alias for `Reg<CFG_DATA_SPEC>`"]
pub type CFG_DATA = crate::Reg<cfg_data::CFG_DATA_SPEC>;
#[doc = "Configuration Data: ARM firmware/software Access this register to Read/Write the configuration bit cells."]
pub mod cfg_data;
#[doc = "RAMFIFO0 register accessor: an alias for `Reg<RAMFIFO0_SPEC>`"]
pub type RAMFIFO0 = crate::Reg<ramfifo0::RAMFIFO0_SPEC>;
#[doc = "RAMFIFO0 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO0. From 0x8000 to 0x8FFC."]
pub mod ramfifo0;
#[doc = "RAMFIFO1 register accessor: an alias for `Reg<RAMFIFO1_SPEC>`"]
pub type RAMFIFO1 = crate::Reg<ramfifo1::RAMFIFO1_SPEC>;
#[doc = "RAMFIFO1 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO1. From 0x9000 to 0x9FFC."]
pub mod ramfifo1;
#[doc = "RAMFIFO2 register accessor: an alias for `Reg<RAMFIFO2_SPEC>`"]
pub type RAMFIFO2 = crate::Reg<ramfifo2::RAMFIFO2_SPEC>;
#[doc = "RAMFIFO2 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO2. From 0xA000 to 0xAFFC."]
pub mod ramfifo2;
#[doc = "RAMFIFO3 register accessor: an alias for `Reg<RAMFIFO3_SPEC>`"]
pub type RAMFIFO3 = crate::Reg<ramfifo3::RAMFIFO3_SPEC>;
#[doc = "RAMFIFO3 Address: ARM firmware/software Access these registers to Read/Write the RAMFIFO3. From 0xB000 to 0xBFFC."]
pub mod ramfifo3;
