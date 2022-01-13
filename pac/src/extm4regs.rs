#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System tick timer register"]
    pub systick_reg: crate::Reg<systick_reg::SYSTICK_REG_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Configuration register 1"]
    pub config1: crate::Reg<config1::CONFIG1_SPEC>,
    #[doc = "0x14 - Configuration register 2"]
    pub config2: crate::Reg<config2::CONFIG2_SPEC>,
    _reserved3: [u8; 0xe8],
    #[doc = "0x100 - FPU configuration register 1"]
    pub config_fp1: crate::Reg<config_fp1::CONFIG_FP1_SPEC>,
    #[doc = "0x104 - FPU configuration register 2"]
    pub config_fp2: crate::Reg<config_fp2::CONFIG_FP2_SPEC>,
    _reserved5: [u8; 0xf8],
    #[doc = "0x200 - Memory configuration register 1"]
    pub config_mem1: crate::Reg<config_mem1::CONFIG_MEM1_SPEC>,
    #[doc = "0x204 - Memory configuration register 2"]
    pub config_mem2: crate::Reg<config_mem2::CONFIG_MEM2_SPEC>,
    #[doc = "0x208 - Memory configuration register 3"]
    pub config_mem3: crate::Reg<config_mem3::CONFIG_MEM3_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x21c - SRAM access while in low power mode interrupt flag register (Set bit to clear)"]
    pub m4_mem_int: crate::Reg<m4_mem_int::M4_MEM_INT_SPEC>,
    #[doc = "0x220 - SRAM memory access while M4 in low power mode interrupts enable register"]
    pub m4_mem_intr_en: crate::Reg<m4_mem_intr_en::M4_MEM_INTR_EN_SPEC>,
    _reserved10: [u8; 0x0c],
    #[doc = "0x230 - No description provided"]
    pub to_intr: crate::Reg<to_intr::TO_INTR_SPEC>,
    #[doc = "0x234 - No description provided"]
    pub to_intr_en: crate::Reg<to_intr_en::TO_INTR_EN_SPEC>,
    _reserved12: [u8; 0x18],
    #[doc = "0x250 - Status of the A1 subsystem power status"]
    pub a1_power_stat: crate::Reg<a1_power_stat::A1_POWER_STAT_SPEC>,
    _reserved13: [u8; 0xac],
    #[doc = "0x300 - Sets Fabric in APB mode"]
    pub fb_ramfifo: crate::Reg<fb_ramfifo::FB_RAMFIFO_SPEC>,
}
#[doc = "SYSTICK_REG register accessor: an alias for `Reg<SYSTICK_REG_SPEC>`"]
pub type SYSTICK_REG = crate::Reg<systick_reg::SYSTICK_REG_SPEC>;
#[doc = "System tick timer register"]
pub mod systick_reg;
#[doc = "CONFIG1 register accessor: an alias for `Reg<CONFIG1_SPEC>`"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = "Configuration register 1"]
pub mod config1;
#[doc = "CONFIG2 register accessor: an alias for `Reg<CONFIG2_SPEC>`"]
pub type CONFIG2 = crate::Reg<config2::CONFIG2_SPEC>;
#[doc = "Configuration register 2"]
pub mod config2;
#[doc = "CONFIG_FP1 register accessor: an alias for `Reg<CONFIG_FP1_SPEC>`"]
pub type CONFIG_FP1 = crate::Reg<config_fp1::CONFIG_FP1_SPEC>;
#[doc = "FPU configuration register 1"]
pub mod config_fp1;
#[doc = "CONFIG_FP2 register accessor: an alias for `Reg<CONFIG_FP2_SPEC>`"]
pub type CONFIG_FP2 = crate::Reg<config_fp2::CONFIG_FP2_SPEC>;
#[doc = "FPU configuration register 2"]
pub mod config_fp2;
#[doc = "CONFIG_MEM1 register accessor: an alias for `Reg<CONFIG_MEM1_SPEC>`"]
pub type CONFIG_MEM1 = crate::Reg<config_mem1::CONFIG_MEM1_SPEC>;
#[doc = "Memory configuration register 1"]
pub mod config_mem1;
#[doc = "CONFIG_MEM2 register accessor: an alias for `Reg<CONFIG_MEM2_SPEC>`"]
pub type CONFIG_MEM2 = crate::Reg<config_mem2::CONFIG_MEM2_SPEC>;
#[doc = "Memory configuration register 2"]
pub mod config_mem2;
#[doc = "CONFIG_MEM3 register accessor: an alias for `Reg<CONFIG_MEM3_SPEC>`"]
pub type CONFIG_MEM3 = crate::Reg<config_mem3::CONFIG_MEM3_SPEC>;
#[doc = "Memory configuration register 3"]
pub mod config_mem3;
#[doc = "M4_MEM_INT register accessor: an alias for `Reg<M4_MEM_INT_SPEC>`"]
pub type M4_MEM_INT = crate::Reg<m4_mem_int::M4_MEM_INT_SPEC>;
#[doc = "SRAM access while in low power mode interrupt flag register (Set bit to clear)"]
pub mod m4_mem_int;
#[doc = "M4_MEM_INTR_EN register accessor: an alias for `Reg<M4_MEM_INTR_EN_SPEC>`"]
pub type M4_MEM_INTR_EN = crate::Reg<m4_mem_intr_en::M4_MEM_INTR_EN_SPEC>;
#[doc = "SRAM memory access while M4 in low power mode interrupts enable register"]
pub mod m4_mem_intr_en;
#[doc = "TO_INTR register accessor: an alias for `Reg<TO_INTR_SPEC>`"]
pub type TO_INTR = crate::Reg<to_intr::TO_INTR_SPEC>;
#[doc = "No description provided"]
pub mod to_intr;
#[doc = "TO_INTR_EN register accessor: an alias for `Reg<TO_INTR_EN_SPEC>`"]
pub type TO_INTR_EN = crate::Reg<to_intr_en::TO_INTR_EN_SPEC>;
#[doc = "No description provided"]
pub mod to_intr_en;
#[doc = "A1_POWER_STAT register accessor: an alias for `Reg<A1_POWER_STAT_SPEC>`"]
pub type A1_POWER_STAT = crate::Reg<a1_power_stat::A1_POWER_STAT_SPEC>;
#[doc = "Status of the A1 subsystem power status"]
pub mod a1_power_stat;
#[doc = "FB_RAMFIFO register accessor: an alias for `Reg<FB_RAMFIFO_SPEC>`"]
pub type FB_RAMFIFO = crate::Reg<fb_ramfifo::FB_RAMFIFO_SPEC>;
#[doc = "Sets Fabric in APB mode"]
pub mod fb_ramfifo;
