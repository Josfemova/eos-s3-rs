#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dma_ctrl: [u8; 0x04],
    #[doc = "0x04 - DMA destination address : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
    pub dma_dest_addr: crate::Reg<dma_dest_addr::DMA_DEST_ADDR_SPEC>,
    #[doc = "0x08 - DMA transfer count in frames (8 bit) (minus 1) : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
    pub dma_xfer_cnt: crate::Reg<dma_xfer_cnt::DMA_XFER_CNT_SPEC>,
    #[doc = "0x0c - Header values read from EEPROM : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
    pub cfg_flash_header: crate::Reg<cfg_flash_header::CFG_FLASH_HEADER_SPEC>,
    #[doc = "0x10 - DMA interrupts"]
    pub dma_intr: crate::Reg<dma_intr::DMA_INTR_SPEC>,
    #[doc = "0x14 - DMA interrupt mask"]
    pub dma_intr_mask: crate::Reg<dma_intr_mask::DMA_INTR_MASK_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - This is the delay value used in the config state machine. It is used for both deep sleep wakeup delay and between retries."]
    #[inline(always)]
    pub fn cfg_machine_st_delay(
        &self,
    ) -> &crate::Reg<cfg_machine_st_delay::CFG_MACHINE_ST_DELAY_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<
                    cfg_machine_st_delay::CFG_MACHINE_ST_DELAY_SPEC,
                >)
        }
    }
    #[doc = "0x00 - DMA Control : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
    #[inline(always)]
    pub fn dma_ctrl(&self) -> &crate::Reg<dma_ctrl::DMA_CTRL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<dma_ctrl::DMA_CTRL_SPEC>)
        }
    }
}
#[doc = "DMA_CTRL register accessor: an alias for `Reg<DMA_CTRL_SPEC>`"]
pub type DMA_CTRL = crate::Reg<dma_ctrl::DMA_CTRL_SPEC>;
#[doc = "DMA Control : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
pub mod dma_ctrl;
#[doc = "DMA_DEST_ADDR register accessor: an alias for `Reg<DMA_DEST_ADDR_SPEC>`"]
pub type DMA_DEST_ADDR = crate::Reg<dma_dest_addr::DMA_DEST_ADDR_SPEC>;
#[doc = "DMA destination address : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
pub mod dma_dest_addr;
#[doc = "DMA_XFER_CNT register accessor: an alias for `Reg<DMA_XFER_CNT_SPEC>`"]
pub type DMA_XFER_CNT = crate::Reg<dma_xfer_cnt::DMA_XFER_CNT_SPEC>;
#[doc = "DMA transfer count in frames (8 bit) (minus 1) : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
pub mod dma_xfer_cnt;
#[doc = "CFG_FLASH_HEADER register accessor: an alias for `Reg<CFG_FLASH_HEADER_SPEC>`"]
pub type CFG_FLASH_HEADER = crate::Reg<cfg_flash_header::CFG_FLASH_HEADER_SPEC>;
#[doc = "Header values read from EEPROM : this register is only accessable when the dma or cfg_sm is not selecting the dmas_mux."]
pub mod cfg_flash_header;
#[doc = "DMA_INTR register accessor: an alias for `Reg<DMA_INTR_SPEC>`"]
pub type DMA_INTR = crate::Reg<dma_intr::DMA_INTR_SPEC>;
#[doc = "DMA interrupts"]
pub mod dma_intr;
#[doc = "DMA_INTR_MASK register accessor: an alias for `Reg<DMA_INTR_MASK_SPEC>`"]
pub type DMA_INTR_MASK = crate::Reg<dma_intr_mask::DMA_INTR_MASK_SPEC>;
#[doc = "DMA interrupt mask"]
pub mod dma_intr_mask;
#[doc = "CFG_MACHINE_ST_DELAY register accessor: an alias for `Reg<CFG_MACHINE_ST_DELAY_SPEC>`"]
pub type CFG_MACHINE_ST_DELAY =
    crate::Reg<cfg_machine_st_delay::CFG_MACHINE_ST_DELAY_SPEC>;
#[doc = "This is the delay value used in the config state machine. It is used for both deep sleep wakeup delay and between retries."]
pub mod cfg_machine_st_delay;
