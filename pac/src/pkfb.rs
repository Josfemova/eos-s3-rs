#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Packet FIFO Bank control"]
    pub pkfb_fifoctrl: crate::Reg<pkfb_fifoctrl::PKFB_FIFOCTRL_SPEC>,
    #[doc = "0x04 - SRAM Test Control 0"]
    pub pkfb_fifosramctrl0:
        crate::Reg<pkfb_fifosramctrl0::PKFB_FIFOSRAMCTRL0_SPEC>,
    #[doc = "0x08 - SRAM Test Control 1"]
    pub pkfb_fifosramctrl1:
        crate::Reg<pkfb_fifosramctrl1::PKFB_FIFOSRAMCTRL1_SPEC>,
    #[doc = "0x0c - Packet FIFO Status register"]
    pub pkfb_fifostatus: crate::Reg<pkfb_fifostatus::PKFB_FIFOSTATUS_SPEC>,
}
#[doc = "PKFB_FIFOCTRL register accessor: an alias for `Reg<PKFB_FIFOCTRL_SPEC>`"]
pub type PKFB_FIFOCTRL = crate::Reg<pkfb_fifoctrl::PKFB_FIFOCTRL_SPEC>;
#[doc = "Packet FIFO Bank control"]
pub mod pkfb_fifoctrl;
#[doc = "PKFB_FIFOSRAMCTRL0 register accessor: an alias for `Reg<PKFB_FIFOSRAMCTRL0_SPEC>`"]
pub type PKFB_FIFOSRAMCTRL0 =
    crate::Reg<pkfb_fifosramctrl0::PKFB_FIFOSRAMCTRL0_SPEC>;
#[doc = "SRAM Test Control 0"]
pub mod pkfb_fifosramctrl0;
#[doc = "PKFB_FIFOSRAMCTRL1 register accessor: an alias for `Reg<PKFB_FIFOSRAMCTRL1_SPEC>`"]
pub type PKFB_FIFOSRAMCTRL1 =
    crate::Reg<pkfb_fifosramctrl1::PKFB_FIFOSRAMCTRL1_SPEC>;
#[doc = "SRAM Test Control 1"]
pub mod pkfb_fifosramctrl1;
#[doc = "PKFB_FIFOSTATUS register accessor: an alias for `Reg<PKFB_FIFOSTATUS_SPEC>`"]
pub type PKFB_FIFOSTATUS = crate::Reg<pkfb_fifostatus::PKFB_FIFOSTATUS_SPEC>;
#[doc = "Packet FIFO Status register"]
pub mod pkfb_fifostatus;
