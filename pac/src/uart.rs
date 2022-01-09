#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uart: [u8; 0x04],
    #[doc = "0x04 - UART receive status register/error clear register. (WC)"]
    pub uart_rsr: crate::Reg<uart_rsr::UART_RSR_SPEC>,
    _reserved2: [u8; 0x10],
    #[doc = "0x18 - UART Flag Register"]
    pub uart_tfr: crate::Reg<uart_tfr::UART_TFR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x20 - 8-bit low-power divisor value."]
    pub uart_ilpr: crate::Reg<uart_ilpr::UART_ILPR_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x2c - UART Line Control Register. This register accesses bit 29 to 22 of the UART Line Control Register, UARTLCR."]
    pub uart_lcr_h: crate::Reg<uart_lcr_h::UART_LCR_H_SPEC>,
    #[doc = "0x30 - UART Control Register"]
    pub uart_cr: crate::Reg<uart_cr::UART_CR_SPEC>,
    #[doc = "0x34 - Interrupt FIFO Level Select Register"]
    pub uart_ifls: crate::Reg<uart_ifls::UART_IFLS_SPEC>,
    #[doc = "0x38 - Interrupt Mask Set/Clear Register"]
    pub uart_imsc: crate::Reg<uart_imsc::UART_IMSC_SPEC>,
    #[doc = "0x3c - Raw interrupt status register"]
    pub uart_ris: crate::Reg<uart_ris::UART_RIS_SPEC>,
    #[doc = "0x40 - Masked interrupt status register"]
    pub uart_mis: crate::Reg<uart_mis::UART_MIS_SPEC>,
    #[doc = "0x44 - Interrupt clear register"]
    pub uart_icr: crate::Reg<uart_icr::UART_ICR_SPEC>,
    _reserved11: [u8; 0x38],
    #[doc = "0x80 - Test Control Register"]
    pub uart_tcr: crate::Reg<uart_tcr::UART_TCR_SPEC>,
    #[doc = "0x84 - Integration test input register"]
    pub uart_itip: crate::Reg<uart_itip::UART_ITIP_SPEC>,
    #[doc = "0x88 - Integration test output register"]
    pub uart_itop: crate::Reg<uart_itop::UART_ITOP_SPEC>,
    #[doc = "0x8c - Test data register"]
    pub uart_tdr: crate::Reg<uart_tdr::UART_TDR_SPEC>,
    _reserved15: [u8; 0x0f50],
    #[doc = "0xfe0 - UART Peripheral ID 0 register"]
    pub uart_periph_id0: crate::Reg<uart_periph_id0::UART_PERIPHID0_SPEC>,
    #[doc = "0xfe4 - UART Peripheral ID 1 register"]
    pub uart_periph_id1: crate::Reg<uart_periph_id1::UART_PERIPHID1_SPEC>,
    #[doc = "0xfe8 - UART Peripheral ID 2 register"]
    pub uart_periph_id2: crate::Reg<uart_periph_id2::UART_PERIPHID2_SPEC>,
    #[doc = "0xfec - UART Peripheral ID 3 register"]
    pub uart_periph_id3: crate::Reg<uart_periph_id3::UART_PERIPHID3_SPEC>,
    #[doc = "0xff0 - UART PCell ID 0 register"]
    pub uart_pcell_id0: crate::Reg<uart_pcell_id0::UART_PCELLID0_SPEC>,
    #[doc = "0xff4 - UART PCell ID 1 register"]
    pub uart_pcell_id1: crate::Reg<uart_pcell_id1::UART_PCELLID1_SPEC>,
    #[doc = "0xff8 - UART PCell ID 2 register"]
    pub uart_pcell_id2: crate::Reg<uart_pcell_id2::UART_PCELLID2_SPEC>,
    #[doc = "0xffc - UART PCell ID 4 register"]
    pub uart_pcell_id4: crate::Reg<uart_pcell_id4::UART_PCELLID4_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - The fractional baud rate divisor."]
    #[inline(always)]
    pub fn uart_fbrd(&self) -> &crate::Reg<uart_fbrd::UART_FBRD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<uart_fbrd::UART_FBRD_SPEC>)
        }
    }
    #[doc = "0x00 - The integer baud rate divisor."]
    #[inline(always)]
    pub fn uart_ibrd(&self) -> &crate::Reg<uart_ibrd::UART_IBRD_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<uart_ibrd::UART_IBRD_SPEC>)
        }
    }
    #[doc = "0x00 - Uart Data register"]
    #[inline(always)]
    pub fn uart_dr(&self) -> &crate::Reg<uart_dr::UART_DR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<uart_dr::UART_DR_SPEC>)
        }
    }
}
#[doc = "UART_DR register accessor: an alias for `Reg<UART_DR_SPEC>`"]
pub type UART_DR = crate::Reg<uart_dr::UART_DR_SPEC>;
#[doc = "Uart Data register"]
pub mod uart_dr;
#[doc = "UART_RSR register accessor: an alias for `Reg<UART_RSR_SPEC>`"]
pub type UART_RSR = crate::Reg<uart_rsr::UART_RSR_SPEC>;
#[doc = "UART receive status register/error clear register. (WC)"]
pub mod uart_rsr;
#[doc = "UART_TFR register accessor: an alias for `Reg<UART_TFR_SPEC>`"]
pub type UART_TFR = crate::Reg<uart_tfr::UART_TFR_SPEC>;
#[doc = "UART Flag Register"]
pub mod uart_tfr;
#[doc = "UART_ILPR register accessor: an alias for `Reg<UART_ILPR_SPEC>`"]
pub type UART_ILPR = crate::Reg<uart_ilpr::UART_ILPR_SPEC>;
#[doc = "8-bit low-power divisor value."]
pub mod uart_ilpr;
#[doc = "UART_IBRD register accessor: an alias for `Reg<UART_IBRD_SPEC>`"]
pub type UART_IBRD = crate::Reg<uart_ibrd::UART_IBRD_SPEC>;
#[doc = "The integer baud rate divisor."]
pub mod uart_ibrd;
#[doc = "UART_FBRD register accessor: an alias for `Reg<UART_FBRD_SPEC>`"]
pub type UART_FBRD = crate::Reg<uart_fbrd::UART_FBRD_SPEC>;
#[doc = "The fractional baud rate divisor."]
pub mod uart_fbrd;
#[doc = "UART_LCR_H register accessor: an alias for `Reg<UART_LCR_H_SPEC>`"]
pub type UART_LCR_H = crate::Reg<uart_lcr_h::UART_LCR_H_SPEC>;
#[doc = "UART Line Control Register. This register accesses bit 29 to 22 of the UART Line Control Register, UARTLCR."]
pub mod uart_lcr_h;
#[doc = "UART_CR register accessor: an alias for `Reg<UART_CR_SPEC>`"]
pub type UART_CR = crate::Reg<uart_cr::UART_CR_SPEC>;
#[doc = "UART Control Register"]
pub mod uart_cr;
#[doc = "UART_IFLS register accessor: an alias for `Reg<UART_IFLS_SPEC>`"]
pub type UART_IFLS = crate::Reg<uart_ifls::UART_IFLS_SPEC>;
#[doc = "Interrupt FIFO Level Select Register"]
pub mod uart_ifls;
#[doc = "UART_IMSC register accessor: an alias for `Reg<UART_IMSC_SPEC>`"]
pub type UART_IMSC = crate::Reg<uart_imsc::UART_IMSC_SPEC>;
#[doc = "Interrupt Mask Set/Clear Register"]
pub mod uart_imsc;
#[doc = "UART_RIS register accessor: an alias for `Reg<UART_RIS_SPEC>`"]
pub type UART_RIS = crate::Reg<uart_ris::UART_RIS_SPEC>;
#[doc = "Raw interrupt status register"]
pub mod uart_ris;
#[doc = "UART_MIS register accessor: an alias for `Reg<UART_MIS_SPEC>`"]
pub type UART_MIS = crate::Reg<uart_mis::UART_MIS_SPEC>;
#[doc = "Masked interrupt status register"]
pub mod uart_mis;
#[doc = "UART_ICR register accessor: an alias for `Reg<UART_ICR_SPEC>`"]
pub type UART_ICR = crate::Reg<uart_icr::UART_ICR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod uart_icr;
#[doc = "UART_TCR register accessor: an alias for `Reg<UART_TCR_SPEC>`"]
pub type UART_TCR = crate::Reg<uart_tcr::UART_TCR_SPEC>;
#[doc = "Test Control Register"]
pub mod uart_tcr;
#[doc = "UART_ITIP register accessor: an alias for `Reg<UART_ITIP_SPEC>`"]
pub type UART_ITIP = crate::Reg<uart_itip::UART_ITIP_SPEC>;
#[doc = "Integration test input register"]
pub mod uart_itip;
#[doc = "UART_ITOP register accessor: an alias for `Reg<UART_ITOP_SPEC>`"]
pub type UART_ITOP = crate::Reg<uart_itop::UART_ITOP_SPEC>;
#[doc = "Integration test output register"]
pub mod uart_itop;
#[doc = "UART_TDR register accessor: an alias for `Reg<UART_TDR_SPEC>`"]
pub type UART_TDR = crate::Reg<uart_tdr::UART_TDR_SPEC>;
#[doc = "Test data register"]
pub mod uart_tdr;
#[doc = "UART_PeriphID0 register accessor: an alias for `Reg<UART_PERIPHID0_SPEC>`"]
pub type UART_PERIPHID0 = crate::Reg<uart_periph_id0::UART_PERIPHID0_SPEC>;
#[doc = "UART Peripheral ID 0 register"]
pub mod uart_periph_id0;
#[doc = "UART_PeriphID1 register accessor: an alias for `Reg<UART_PERIPHID1_SPEC>`"]
pub type UART_PERIPHID1 = crate::Reg<uart_periph_id1::UART_PERIPHID1_SPEC>;
#[doc = "UART Peripheral ID 1 register"]
pub mod uart_periph_id1;
#[doc = "UART_PeriphID2 register accessor: an alias for `Reg<UART_PERIPHID2_SPEC>`"]
pub type UART_PERIPHID2 = crate::Reg<uart_periph_id2::UART_PERIPHID2_SPEC>;
#[doc = "UART Peripheral ID 2 register"]
pub mod uart_periph_id2;
#[doc = "UART_PeriphID3 register accessor: an alias for `Reg<UART_PERIPHID3_SPEC>`"]
pub type UART_PERIPHID3 = crate::Reg<uart_periph_id3::UART_PERIPHID3_SPEC>;
#[doc = "UART Peripheral ID 3 register"]
pub mod uart_periph_id3;
#[doc = "UART_PCellID0 register accessor: an alias for `Reg<UART_PCELLID0_SPEC>`"]
pub type UART_PCELLID0 = crate::Reg<uart_pcell_id0::UART_PCELLID0_SPEC>;
#[doc = "UART PCell ID 0 register"]
pub mod uart_pcell_id0;
#[doc = "UART_PCellID1 register accessor: an alias for `Reg<UART_PCELLID1_SPEC>`"]
pub type UART_PCELLID1 = crate::Reg<uart_pcell_id1::UART_PCELLID1_SPEC>;
#[doc = "UART PCell ID 1 register"]
pub mod uart_pcell_id1;
#[doc = "UART_PCellID2 register accessor: an alias for `Reg<UART_PCELLID2_SPEC>`"]
pub type UART_PCELLID2 = crate::Reg<uart_pcell_id2::UART_PCELLID2_SPEC>;
#[doc = "UART PCell ID 2 register"]
pub mod uart_pcell_id2;
#[doc = "UART_PCellID4 register accessor: an alias for `Reg<UART_PCELLID4_SPEC>`"]
pub type UART_PCELLID4 = crate::Reg<uart_pcell_id4::UART_PCELLID4_SPEC>;
#[doc = "UART PCell ID 4 register"]
pub mod uart_pcell_id4;
