#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Debug Monitor information"]
    pub dbg_mon: crate::Reg<dbg_mon::DBG_MON_SPEC>,
    #[doc = "0x04 - Register for selecting the subsystem routed to the debug monitor"]
    pub subsys_dbg_mon_sel: crate::Reg<subsys_dbg_mon_sel::SUBSYS_DBG_MON_SEL_SPEC>,
    #[doc = "0x08 - Select A0 debug monitors"]
    pub a0_dbg_mon_sel: crate::Reg<a0_dbg_mon_sel::A0_DBG_MON_SEL_SPEC>,
    #[doc = "0x0c - Selects the data present in the PMU debug monitor. The output will be (Except 0 and 10), {Status0, Status1, ISO, RET, GateCLK_N, Mem_DS, MP_Gate, RP_Gate)"]
    pub a0_pmu_dbg_mon_sel: crate::Reg<a0_pmu_dbg_mon_sel::A0_PMU_DBG_MON_SEL_SPEC>,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - Reads the value of the IO pins"]
    pub io_input: crate::Reg<io_input::IO_INPUT_SPEC>,
    #[doc = "0x104 - Allows FW to drive the IO with the values specified in this register"]
    pub io_output: crate::Reg<io_output::IO_OUTPUT_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x110 - Software Mailbox (can be used for communication between M4 and AP)"]
    pub sw_mb_1: crate::Reg<sw_mb_1::SW_MB_1_SPEC>,
    #[doc = "0x114 - Software Mailbox (can be used for communication between M4 and AP)"]
    pub sw_mb_2: crate::Reg<sw_mb_2::SW_MB_2_SPEC>,
    _reserved8: [u8; 0xe8],
    #[doc = "0x200 - Select 1.8V for VCCIO for up to 4 banks. Write a 1 to a bank field to make the IO VCC = 1.8V"]
    pub pad_sel18: crate::Reg<pad_sel18::PAD_SEL18_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x210 - Memory Configuration"]
    pub config_mem128_aon: crate::Reg<config_mem128_aon::CONFIG_MEM128_AON_SPEC>,
    _reserved10: [u8; 0xfc],
    #[doc = "0x310 - Control value and status of LOCK_KEY"]
    pub lock_key_ctrl: crate::Reg<lock_key_ctrl::LOCK_KEY_CTRL_SPEC>,
    _reserved11: [u8; 0xe8],
    #[doc = "0x3fc - Fabric device ID"]
    pub fb_device_id: crate::Reg<fb_device_id::FB_DEVICE_ID_SPEC>,
}
#[doc = "DBG_MON register accessor: an alias for `Reg<DBG_MON_SPEC>`"]
pub type DBG_MON = crate::Reg<dbg_mon::DBG_MON_SPEC>;
#[doc = "Debug Monitor information"]
pub mod dbg_mon;
#[doc = "SUBSYS_DBG_MON_SEL register accessor: an alias for `Reg<SUBSYS_DBG_MON_SEL_SPEC>`"]
pub type SUBSYS_DBG_MON_SEL = crate::Reg<subsys_dbg_mon_sel::SUBSYS_DBG_MON_SEL_SPEC>;
#[doc = "Register for selecting the subsystem routed to the debug monitor"]
pub mod subsys_dbg_mon_sel;
#[doc = "A0_DBG_MON_SEL register accessor: an alias for `Reg<A0_DBG_MON_SEL_SPEC>`"]
pub type A0_DBG_MON_SEL = crate::Reg<a0_dbg_mon_sel::A0_DBG_MON_SEL_SPEC>;
#[doc = "Select A0 debug monitors"]
pub mod a0_dbg_mon_sel;
#[doc = "A0_PMU_DBG_MON_SEL register accessor: an alias for `Reg<A0_PMU_DBG_MON_SEL_SPEC>`"]
pub type A0_PMU_DBG_MON_SEL = crate::Reg<a0_pmu_dbg_mon_sel::A0_PMU_DBG_MON_SEL_SPEC>;
#[doc = "Selects the data present in the PMU debug monitor. The output will be (Except 0 and 10), {Status0, Status1, ISO, RET, GateCLK_N, Mem_DS, MP_Gate, RP_Gate)"]
pub mod a0_pmu_dbg_mon_sel;
#[doc = "IO_INPUT register accessor: an alias for `Reg<IO_INPUT_SPEC>`"]
pub type IO_INPUT = crate::Reg<io_input::IO_INPUT_SPEC>;
#[doc = "Reads the value of the IO pins"]
pub mod io_input;
#[doc = "IO_OUTPUT register accessor: an alias for `Reg<IO_OUTPUT_SPEC>`"]
pub type IO_OUTPUT = crate::Reg<io_output::IO_OUTPUT_SPEC>;
#[doc = "Allows FW to drive the IO with the values specified in this register"]
pub mod io_output;
#[doc = "SW_MB_1 register accessor: an alias for `Reg<SW_MB_1_SPEC>`"]
pub type SW_MB_1 = crate::Reg<sw_mb_1::SW_MB_1_SPEC>;
#[doc = "Software Mailbox (can be used for communication between M4 and AP)"]
pub mod sw_mb_1;
#[doc = "SW_MB_2 register accessor: an alias for `Reg<SW_MB_2_SPEC>`"]
pub type SW_MB_2 = crate::Reg<sw_mb_2::SW_MB_2_SPEC>;
#[doc = "Software Mailbox (can be used for communication between M4 and AP)"]
pub mod sw_mb_2;
#[doc = "PAD_SEL18 register accessor: an alias for `Reg<PAD_SEL18_SPEC>`"]
pub type PAD_SEL18 = crate::Reg<pad_sel18::PAD_SEL18_SPEC>;
#[doc = "Select 1.8V for VCCIO for up to 4 banks. Write a 1 to a bank field to make the IO VCC = 1.8V"]
pub mod pad_sel18;
#[doc = "CONFIG_MEM128_AON register accessor: an alias for `Reg<CONFIG_MEM128_AON_SPEC>`"]
pub type CONFIG_MEM128_AON = crate::Reg<config_mem128_aon::CONFIG_MEM128_AON_SPEC>;
#[doc = "Memory Configuration"]
pub mod config_mem128_aon;
#[doc = "LOCK_KEY_CTRL register accessor: an alias for `Reg<LOCK_KEY_CTRL_SPEC>`"]
pub type LOCK_KEY_CTRL = crate::Reg<lock_key_ctrl::LOCK_KEY_CTRL_SPEC>;
#[doc = "Control value and status of LOCK_KEY"]
pub mod lock_key_ctrl;
#[doc = "FB_DEVICE_ID register accessor: an alias for `Reg<FB_DEVICE_ID_SPEC>`"]
pub type FB_DEVICE_ID = crate::Reg<fb_device_id::FB_DEVICE_ID_SPEC>;
#[doc = "Fabric device ID"]
pub mod fb_device_id;
