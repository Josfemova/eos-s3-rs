#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Last sampled value"]
    pub out: crate::Reg<out::OUT_SPEC>,
    #[doc = "0x04 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Control register"]
    pub control: crate::Reg<control::CONTROL_SPEC>,
}
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Last sampled value"]
pub mod out;
#[doc = "Status register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "Control register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Control register"]
pub mod control;
