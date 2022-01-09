#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Wishbone master address selection"]
    pub addr: crate::Reg<addr::ADDR_SPEC>,
    #[doc = "0x04 - I2C slave data register via WishBone master"]
    pub wdata: crate::Reg<wdata::WDATA_SPEC>,
    #[doc = "0x08 - Control and status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x0c - Read data from I2C to Wishbone master is registered"]
    pub rdata: crate::Reg<rdata::RDATA_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x14 - SRAM test control register 1"]
    pub sram_test_reg1: crate::Reg<sram_test_reg1::SRAM_TEST_REG1_SPEC>,
    #[doc = "0x18 - SRAM test control register 2"]
    pub sram_test_reg2: crate::Reg<sram_test_reg2::SRAM_TEST_REG2_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - Flexible Fusion Engine status and control register"]
    pub ffe_csr: crate::Reg<ffe_csr::FFE_CSR_SPEC>,
    _reserved7: [u8; 0x14],
    #[doc = "0x38 - Combined Flexible Fusion Engine debug signals"]
    pub ffe_dbg_combined: crate::Reg<ffe_dbg_combined::FFE_DBG_COMBINED_SPEC>,
    _reserved8: [u8; 0xc4],
    #[doc = "0x100 - Commands for the Flexible Fusion Engine"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x108 - Varied interrupt configurations"]
    pub interrupt: crate::Reg<interrupt::INTERRUPT_SPEC>,
    #[doc = "0x10c - Control the masking for different Flexible Fusion Engine interrupts"]
    pub interrupt_en: crate::Reg<interrupt_en::INTERRUPT_EN_SPEC>,
    #[doc = "0x110 - FFE status register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x114 - Mailbox register to the FFE. This register can be set by system software to send a message or configuration information to the FFE as it runs its algorithm, thus affecting the algorithm while it is running. A special instruction may be used in the algorithm to read this mailbox register."]
    pub mailbox_to_ffe0: crate::Reg<mailbox_to_ffe0::MAILBOX_TO_FFE0_SPEC>,
    _reserved13: [u8; 0x08],
    #[doc = "0x120 - SM0/SM1 run time address"]
    pub sm_runtime_addr: crate::Reg<sm_runtime_addr::SM_RUNTIME_ADDR_SPEC>,
    #[doc = "0x124 - Used to toggle signal used to signal when a new value has been written."]
    pub sm0_runtime_addr_ctrl:
        crate::Reg<sm0_runtime_addr_ctrl::SM0_RUNTIME_ADDR_CTRL_SPEC>,
    #[doc = "0x128 - Used to toggle signal used to signal when a new value has been written."]
    pub sm1_runtime_addr_ctrl:
        crate::Reg<sm1_runtime_addr_ctrl::SM1_RUNTIME_ADDR_CTRL_SPEC>,
    #[doc = "0x12c - SM current program counter"]
    pub sm0_runtime_addr_cur:
        crate::Reg<sm0_runtime_addr_cur::SM0_RUNTIME_ADDR_CUR_SPEC>,
    #[doc = "0x130 - SM current program counter"]
    pub sm1_runtime_addr_cur:
        crate::Reg<sm1_runtime_addr_cur::SM1_RUNTIME_ADDR_CUR_SPEC>,
    _reserved18: [u8; 0x0c],
    #[doc = "0x140 - SM Debug selection"]
    pub sm0_debug_sel: crate::Reg<sm0_debug_sel::SM0_DEBUG_SEL_SPEC>,
    #[doc = "0x144 - SM Debug selection"]
    pub sm1_debug_sel: crate::Reg<sm1_debug_sel::SM1_DEBUG_SEL_SPEC>,
    #[doc = "0x148 - Debug Selection"]
    pub ffe_debug_sel: crate::Reg<ffe_debug_sel::FFE_DEBUG_SEL_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x150 - Break point control"]
    pub ffe0_break_point_cfg:
        crate::Reg<ffe0_break_point_cfg::FFE0_BREAK_POINT_CFG_SPEC>,
    #[doc = "0x154 - Seems to be another breakpoint control register"]
    pub ffe0_break_point_cont:
        crate::Reg<ffe0_break_point_cont::FFE0_BREAK_POINT_CONT_SPEC>,
    #[doc = "0x158 - FFE break point status register"]
    pub ffe0_break_point_stat:
        crate::Reg<ffe0_break_point_stat::FFE0_BREAK_POINT_STAT_SPEC>,
    _reserved24: [u8; 0x04],
    #[doc = "0x160 - These registers hold the xPC (program counter) address 'break points'."]
    pub ffe0_bp_xpc_0: crate::Reg<ffe0_bp_xpc_0::FFE0_BP_XPC_0_SPEC>,
    #[doc = "0x164 - These registers hold the xPC (program counter) address 'break points'."]
    pub ffe0_bp_xpc_1: crate::Reg<ffe0_bp_xpc_1::FFE0_BP_XPC_1_SPEC>,
    #[doc = "0x168 - These registers hold the xPC (program counter) address 'break points'."]
    pub ffe0_bp_xpc_2: crate::Reg<ffe0_bp_xpc_2::FFE0_BP_XPC_2_SPEC>,
    #[doc = "0x16c - These registers hold the xPC (program counter) address 'break points'."]
    pub ffe0_bp_xpc_3: crate::Reg<ffe0_bp_xpc_3::FFE0_BP_XPC_3_SPEC>,
}
#[doc = "ADDR register accessor: an alias for `Reg<ADDR_SPEC>`"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Wishbone master address selection"]
pub mod addr;
#[doc = "WDATA register accessor: an alias for `Reg<WDATA_SPEC>`"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "I2C slave data register via WishBone master"]
pub mod wdata;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control and status register"]
pub mod csr;
#[doc = "RDATA register accessor: an alias for `Reg<RDATA_SPEC>`"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "Read data from I2C to Wishbone master is registered"]
pub mod rdata;
#[doc = "SRAM_TEST_REG1 register accessor: an alias for `Reg<SRAM_TEST_REG1_SPEC>`"]
pub type SRAM_TEST_REG1 = crate::Reg<sram_test_reg1::SRAM_TEST_REG1_SPEC>;
#[doc = "SRAM test control register 1"]
pub mod sram_test_reg1;
#[doc = "SRAM_TEST_REG2 register accessor: an alias for `Reg<SRAM_TEST_REG2_SPEC>`"]
pub type SRAM_TEST_REG2 = crate::Reg<sram_test_reg2::SRAM_TEST_REG2_SPEC>;
#[doc = "SRAM test control register 2"]
pub mod sram_test_reg2;
#[doc = "FFE_CSR register accessor: an alias for `Reg<FFE_CSR_SPEC>`"]
pub type FFE_CSR = crate::Reg<ffe_csr::FFE_CSR_SPEC>;
#[doc = "Flexible Fusion Engine status and control register"]
pub mod ffe_csr;
#[doc = "FFE_DBG_COMBINED register accessor: an alias for `Reg<FFE_DBG_COMBINED_SPEC>`"]
pub type FFE_DBG_COMBINED = crate::Reg<ffe_dbg_combined::FFE_DBG_COMBINED_SPEC>;
#[doc = "Combined Flexible Fusion Engine debug signals"]
pub mod ffe_dbg_combined;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Commands for the Flexible Fusion Engine"]
pub mod cmd;
#[doc = "INTERRUPT register accessor: an alias for `Reg<INTERRUPT_SPEC>`"]
pub type INTERRUPT = crate::Reg<interrupt::INTERRUPT_SPEC>;
#[doc = "Varied interrupt configurations"]
pub mod interrupt;
#[doc = "INTERRUPT_EN register accessor: an alias for `Reg<INTERRUPT_EN_SPEC>`"]
pub type INTERRUPT_EN = crate::Reg<interrupt_en::INTERRUPT_EN_SPEC>;
#[doc = "Control the masking for different Flexible Fusion Engine interrupts"]
pub mod interrupt_en;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "FFE status register"]
pub mod status;
#[doc = "MAILBOX_TO_FFE0 register accessor: an alias for `Reg<MAILBOX_TO_FFE0_SPEC>`"]
pub type MAILBOX_TO_FFE0 = crate::Reg<mailbox_to_ffe0::MAILBOX_TO_FFE0_SPEC>;
#[doc = "Mailbox register to the FFE. This register can be set by system software to send a message or configuration information to the FFE as it runs its algorithm, thus affecting the algorithm while it is running. A special instruction may be used in the algorithm to read this mailbox register."]
pub mod mailbox_to_ffe0;
#[doc = "SM_RUNTIME_ADDR register accessor: an alias for `Reg<SM_RUNTIME_ADDR_SPEC>`"]
pub type SM_RUNTIME_ADDR = crate::Reg<sm_runtime_addr::SM_RUNTIME_ADDR_SPEC>;
#[doc = "SM0/SM1 run time address"]
pub mod sm_runtime_addr;
#[doc = "SM0_RUNTIME_ADDR_CTRL register accessor: an alias for `Reg<SM0_RUNTIME_ADDR_CTRL_SPEC>`"]
pub type SM0_RUNTIME_ADDR_CTRL =
    crate::Reg<sm0_runtime_addr_ctrl::SM0_RUNTIME_ADDR_CTRL_SPEC>;
#[doc = "Used to toggle signal used to signal when a new value has been written."]
pub mod sm0_runtime_addr_ctrl;
#[doc = "SM1_RUNTIME_ADDR_CTRL register accessor: an alias for `Reg<SM1_RUNTIME_ADDR_CTRL_SPEC>`"]
pub type SM1_RUNTIME_ADDR_CTRL =
    crate::Reg<sm1_runtime_addr_ctrl::SM1_RUNTIME_ADDR_CTRL_SPEC>;
#[doc = "Used to toggle signal used to signal when a new value has been written."]
pub mod sm1_runtime_addr_ctrl;
#[doc = "SM0_RUNTIME_ADDR_CUR register accessor: an alias for `Reg<SM0_RUNTIME_ADDR_CUR_SPEC>`"]
pub type SM0_RUNTIME_ADDR_CUR =
    crate::Reg<sm0_runtime_addr_cur::SM0_RUNTIME_ADDR_CUR_SPEC>;
#[doc = "SM current program counter"]
pub mod sm0_runtime_addr_cur;
#[doc = "SM1_RUNTIME_ADDR_CUR register accessor: an alias for `Reg<SM1_RUNTIME_ADDR_CUR_SPEC>`"]
pub type SM1_RUNTIME_ADDR_CUR =
    crate::Reg<sm1_runtime_addr_cur::SM1_RUNTIME_ADDR_CUR_SPEC>;
#[doc = "SM current program counter"]
pub mod sm1_runtime_addr_cur;
#[doc = "SM0_DEBUG_SEL register accessor: an alias for `Reg<SM0_DEBUG_SEL_SPEC>`"]
pub type SM0_DEBUG_SEL = crate::Reg<sm0_debug_sel::SM0_DEBUG_SEL_SPEC>;
#[doc = "SM Debug selection"]
pub mod sm0_debug_sel;
#[doc = "SM1_DEBUG_SEL register accessor: an alias for `Reg<SM1_DEBUG_SEL_SPEC>`"]
pub type SM1_DEBUG_SEL = crate::Reg<sm1_debug_sel::SM1_DEBUG_SEL_SPEC>;
#[doc = "SM Debug selection"]
pub mod sm1_debug_sel;
#[doc = "FFE_DEBUG_SEL register accessor: an alias for `Reg<FFE_DEBUG_SEL_SPEC>`"]
pub type FFE_DEBUG_SEL = crate::Reg<ffe_debug_sel::FFE_DEBUG_SEL_SPEC>;
#[doc = "Debug Selection"]
pub mod ffe_debug_sel;
#[doc = "FFE0_BREAK_POINT_CFG register accessor: an alias for `Reg<FFE0_BREAK_POINT_CFG_SPEC>`"]
pub type FFE0_BREAK_POINT_CFG =
    crate::Reg<ffe0_break_point_cfg::FFE0_BREAK_POINT_CFG_SPEC>;
#[doc = "Break point control"]
pub mod ffe0_break_point_cfg;
#[doc = "FFE0_BREAK_POINT_CONT register accessor: an alias for `Reg<FFE0_BREAK_POINT_CONT_SPEC>`"]
pub type FFE0_BREAK_POINT_CONT =
    crate::Reg<ffe0_break_point_cont::FFE0_BREAK_POINT_CONT_SPEC>;
#[doc = "Seems to be another breakpoint control register"]
pub mod ffe0_break_point_cont;
#[doc = "FFE0_BREAK_POINT_STAT register accessor: an alias for `Reg<FFE0_BREAK_POINT_STAT_SPEC>`"]
pub type FFE0_BREAK_POINT_STAT =
    crate::Reg<ffe0_break_point_stat::FFE0_BREAK_POINT_STAT_SPEC>;
#[doc = "FFE break point status register"]
pub mod ffe0_break_point_stat;
#[doc = "FFE0_BP_XPC_0 register accessor: an alias for `Reg<FFE0_BP_XPC_0_SPEC>`"]
pub type FFE0_BP_XPC_0 = crate::Reg<ffe0_bp_xpc_0::FFE0_BP_XPC_0_SPEC>;
#[doc = "These registers hold the xPC (program counter) address 'break points'."]
pub mod ffe0_bp_xpc_0;
#[doc = "FFE0_BP_XPC_1 register accessor: an alias for `Reg<FFE0_BP_XPC_1_SPEC>`"]
pub type FFE0_BP_XPC_1 = crate::Reg<ffe0_bp_xpc_1::FFE0_BP_XPC_1_SPEC>;
#[doc = "These registers hold the xPC (program counter) address 'break points'."]
pub mod ffe0_bp_xpc_1;
#[doc = "FFE0_BP_XPC_2 register accessor: an alias for `Reg<FFE0_BP_XPC_2_SPEC>`"]
pub type FFE0_BP_XPC_2 = crate::Reg<ffe0_bp_xpc_2::FFE0_BP_XPC_2_SPEC>;
#[doc = "These registers hold the xPC (program counter) address 'break points'."]
pub mod ffe0_bp_xpc_2;
#[doc = "FFE0_BP_XPC_3 register accessor: an alias for `Reg<FFE0_BP_XPC_3_SPEC>`"]
pub type FFE0_BP_XPC_3 = crate::Reg<ffe0_bp_xpc_3::FFE0_BP_XPC_3_SPEC>;
#[doc = "These registers hold the xPC (program counter) address 'break points'."]
pub mod ffe0_bp_xpc_3;
