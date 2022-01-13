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
    #[doc = "0x10 - FIFO 0 Push Control"]
    pub pkfb_pf0pushctl: crate::Reg<pkfb_pf0pushctl::PKFB_PF0PUSHCTL_SPEC>,
    #[doc = "0x14 - FIFO 0 Pop Control"]
    pub pkfb_pf0popctl: crate::Reg<pkfb_pf0popctl::PKFB_PF0POPCTL_SPEC>,
    #[doc = "0x18 - FIFO 0 Count"]
    pub pkfb_pf0cnt: crate::Reg<pkfb_pf0cnt::PKFB_PF0CNT_SPEC>,
    #[doc = "0x1c - FIFO 0 Push/POP Data Register"]
    pub pkfb_pf0data: crate::Reg<pkfb_pf0data::PKFB_PF0DATA_SPEC>,
    #[doc = "0x20 - FIFO 1 Push Control"]
    pub pkfb_pf1pushctl: crate::Reg<pkfb_pf1pushctl::PKFB_PF1PUSHCTL_SPEC>,
    #[doc = "0x24 - FIFO 1 Pop Control"]
    pub pkfb_pf1popctl: crate::Reg<pkfb_pf1popctl::PKFB_PF1POPCTL_SPEC>,
    #[doc = "0x28 - FIFO 1 Count"]
    pub pkfb_pf1cnt: crate::Reg<pkfb_pf1cnt::PKFB_PF1CNT_SPEC>,
    #[doc = "0x2c - FIFO 1 Push/POP Data Register"]
    pub pkfb_pf1data: crate::Reg<pkfb_pf1data::PKFB_PF1DATA_SPEC>,
    #[doc = "0x30 - FIFO 2 Push Control"]
    pub pkfb_pf2pushctl: crate::Reg<pkfb_pf2pushctl::PKFB_PF2PUSHCTL_SPEC>,
    #[doc = "0x34 - FIFO 2 Pop Control"]
    pub pkfb_pf2popctl: crate::Reg<pkfb_pf2popctl::PKFB_PF2POPCTL_SPEC>,
    #[doc = "0x38 - FIFO 2 Count"]
    pub pkfb_pf2cnt: crate::Reg<pkfb_pf2cnt::PKFB_PF2CNT_SPEC>,
    #[doc = "0x3c - FIFO 2 Push/POP Data Register"]
    pub pkfb_pf2data: crate::Reg<pkfb_pf2data::PKFB_PF2DATA_SPEC>,
    #[doc = "0x40 - FIFO 8k Push Control"]
    pub pkfb_pf8kpushctl: crate::Reg<pkfb_pf8kpushctl::PKFB_PF8KPUSHCTL_SPEC>,
    #[doc = "0x44 - FIFO 8k Pop Control"]
    pub pkfb_pf8kpopctl: crate::Reg<pkfb_pf8kpopctl::PKFB_PF8KPOPCTL_SPEC>,
    #[doc = "0x48 - FIFO 8k Count"]
    pub pkfb_pf8kcnt: crate::Reg<pkfb_pf8kcnt::PKFB_PF8KCNT_SPEC>,
    #[doc = "0x4c - FIFO 8k Push/POP Data Register"]
    pub pkfb_pf8k_data: crate::Reg<pkfb_pf8k_data::PKFB_PF8KDATA_SPEC>,
    #[doc = "0x50 - Control for collision interrupts"]
    pub pkfb_fifo_coll_intr:
        crate::Reg<pkfb_fifo_coll_intr::PKFB_FIFO_COLL_INTR_SPEC>,
    #[doc = "0x54 - Control register for enabling or masking the collisione interrupts"]
    pub pkfb_fifo_coll_intr_en:
        crate::Reg<pkfb_fifo_coll_intr_en::PKFB_FIFO_COLL_INTR_EN_SPEC>,
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
#[doc = "PKFB_PF0PUSHCTL register accessor: an alias for `Reg<PKFB_PF0PUSHCTL_SPEC>`"]
pub type PKFB_PF0PUSHCTL = crate::Reg<pkfb_pf0pushctl::PKFB_PF0PUSHCTL_SPEC>;
#[doc = "FIFO 0 Push Control"]
pub mod pkfb_pf0pushctl;
#[doc = "PKFB_PF0POPCTL register accessor: an alias for `Reg<PKFB_PF0POPCTL_SPEC>`"]
pub type PKFB_PF0POPCTL = crate::Reg<pkfb_pf0popctl::PKFB_PF0POPCTL_SPEC>;
#[doc = "FIFO 0 Pop Control"]
pub mod pkfb_pf0popctl;
#[doc = "PKFB_PF0CNT register accessor: an alias for `Reg<PKFB_PF0CNT_SPEC>`"]
pub type PKFB_PF0CNT = crate::Reg<pkfb_pf0cnt::PKFB_PF0CNT_SPEC>;
#[doc = "FIFO 0 Count"]
pub mod pkfb_pf0cnt;
#[doc = "PKFB_PF0DATA register accessor: an alias for `Reg<PKFB_PF0DATA_SPEC>`"]
pub type PKFB_PF0DATA = crate::Reg<pkfb_pf0data::PKFB_PF0DATA_SPEC>;
#[doc = "FIFO 0 Push/POP Data Register"]
pub mod pkfb_pf0data;
#[doc = "PKFB_PF1PUSHCTL register accessor: an alias for `Reg<PKFB_PF1PUSHCTL_SPEC>`"]
pub type PKFB_PF1PUSHCTL = crate::Reg<pkfb_pf1pushctl::PKFB_PF1PUSHCTL_SPEC>;
#[doc = "FIFO 1 Push Control"]
pub mod pkfb_pf1pushctl;
#[doc = "PKFB_PF1POPCTL register accessor: an alias for `Reg<PKFB_PF1POPCTL_SPEC>`"]
pub type PKFB_PF1POPCTL = crate::Reg<pkfb_pf1popctl::PKFB_PF1POPCTL_SPEC>;
#[doc = "FIFO 1 Pop Control"]
pub mod pkfb_pf1popctl;
#[doc = "PKFB_PF1CNT register accessor: an alias for `Reg<PKFB_PF1CNT_SPEC>`"]
pub type PKFB_PF1CNT = crate::Reg<pkfb_pf1cnt::PKFB_PF1CNT_SPEC>;
#[doc = "FIFO 1 Count"]
pub mod pkfb_pf1cnt;
#[doc = "PKFB_PF1DATA register accessor: an alias for `Reg<PKFB_PF1DATA_SPEC>`"]
pub type PKFB_PF1DATA = crate::Reg<pkfb_pf1data::PKFB_PF1DATA_SPEC>;
#[doc = "FIFO 1 Push/POP Data Register"]
pub mod pkfb_pf1data;
#[doc = "PKFB_PF2PUSHCTL register accessor: an alias for `Reg<PKFB_PF2PUSHCTL_SPEC>`"]
pub type PKFB_PF2PUSHCTL = crate::Reg<pkfb_pf2pushctl::PKFB_PF2PUSHCTL_SPEC>;
#[doc = "FIFO 2 Push Control"]
pub mod pkfb_pf2pushctl;
#[doc = "PKFB_PF2POPCTL register accessor: an alias for `Reg<PKFB_PF2POPCTL_SPEC>`"]
pub type PKFB_PF2POPCTL = crate::Reg<pkfb_pf2popctl::PKFB_PF2POPCTL_SPEC>;
#[doc = "FIFO 2 Pop Control"]
pub mod pkfb_pf2popctl;
#[doc = "PKFB_PF2CNT register accessor: an alias for `Reg<PKFB_PF2CNT_SPEC>`"]
pub type PKFB_PF2CNT = crate::Reg<pkfb_pf2cnt::PKFB_PF2CNT_SPEC>;
#[doc = "FIFO 2 Count"]
pub mod pkfb_pf2cnt;
#[doc = "PKFB_PF2DATA register accessor: an alias for `Reg<PKFB_PF2DATA_SPEC>`"]
pub type PKFB_PF2DATA = crate::Reg<pkfb_pf2data::PKFB_PF2DATA_SPEC>;
#[doc = "FIFO 2 Push/POP Data Register"]
pub mod pkfb_pf2data;
#[doc = "PKFB_PF8KPUSHCTL register accessor: an alias for `Reg<PKFB_PF8KPUSHCTL_SPEC>`"]
pub type PKFB_PF8KPUSHCTL = crate::Reg<pkfb_pf8kpushctl::PKFB_PF8KPUSHCTL_SPEC>;
#[doc = "FIFO 8k Push Control"]
pub mod pkfb_pf8kpushctl;
#[doc = "PKFB_PF8KPOPCTL register accessor: an alias for `Reg<PKFB_PF8KPOPCTL_SPEC>`"]
pub type PKFB_PF8KPOPCTL = crate::Reg<pkfb_pf8kpopctl::PKFB_PF8KPOPCTL_SPEC>;
#[doc = "FIFO 8k Pop Control"]
pub mod pkfb_pf8kpopctl;
#[doc = "PKFB_PF8KCNT register accessor: an alias for `Reg<PKFB_PF8KCNT_SPEC>`"]
pub type PKFB_PF8KCNT = crate::Reg<pkfb_pf8kcnt::PKFB_PF8KCNT_SPEC>;
#[doc = "FIFO 8k Count"]
pub mod pkfb_pf8kcnt;
#[doc = "PKFB_PF8kDATA register accessor: an alias for `Reg<PKFB_PF8KDATA_SPEC>`"]
pub type PKFB_PF8KDATA = crate::Reg<pkfb_pf8k_data::PKFB_PF8KDATA_SPEC>;
#[doc = "FIFO 8k Push/POP Data Register"]
pub mod pkfb_pf8k_data;
#[doc = "PKFB_FIFO_COLL_INTR register accessor: an alias for `Reg<PKFB_FIFO_COLL_INTR_SPEC>`"]
pub type PKFB_FIFO_COLL_INTR =
    crate::Reg<pkfb_fifo_coll_intr::PKFB_FIFO_COLL_INTR_SPEC>;
#[doc = "Control for collision interrupts"]
pub mod pkfb_fifo_coll_intr;
#[doc = "PKFB_FIFO_COLL_INTR_EN register accessor: an alias for `Reg<PKFB_FIFO_COLL_INTR_EN_SPEC>`"]
pub type PKFB_FIFO_COLL_INTR_EN =
    crate::Reg<pkfb_fifo_coll_intr_en::PKFB_FIFO_COLL_INTR_EN_SPEC>;
#[doc = "Control register for enabling or masking the collisione interrupts"]
pub mod pkfb_fifo_coll_intr_en;
