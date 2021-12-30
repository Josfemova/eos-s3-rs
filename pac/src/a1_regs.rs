#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Control"]
    pub cfg_ctrl: crate::Reg<cfg_ctrl::CFG_CTRL_SPEC>,
}
#[doc = "CFG_CTRL register accessor: an alias for `Reg<CFG_CTRL_SPEC>`"]
pub type CFG_CTRL = crate::Reg<cfg_ctrl::CFG_CTRL_SPEC>;
#[doc = "Configuration Control"]
pub mod cfg_ctrl;
